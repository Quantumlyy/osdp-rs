//! Packet codec — encode/decode the OSDP wire framing.
//!
//! # Spec: §5.9 Tables 1–3
//!
//! Layout:
//!
//! ```text
//! +------+------+---------+---------+------+========+======+----+======+
//! | SOM  | ADDR | LEN_LSB | LEN_MSB | CTRL |  SCB?  | CMND | DAT| MAC? |  cksum/CRC
//! +------+------+---------+---------+------+========+======+----+======+
//! ```

use crate::SOM;
use crate::error::Error;
use crate::packet::checksum::checksum8;
use crate::packet::crc::crc16;
use crate::packet::header::{Address, ControlByte};
use crate::packet::scb::ScbView;
use crate::packet::trailer::Trailer;

/// Length of the fixed OSDP header (SOM, ADDR, LEN_LSB, LEN_MSB, CTRL).
pub const HEADER_LEN: usize = 5;

/// MAC trailer length on the wire (the first 4 bytes of the computed MAC).
pub const MAC_LEN: usize = 4;

/// View of a parsed OSDP packet, borrowed from the input slice.
///
/// The codec is *zero-copy*: payload, SCB data, and MAC slices all point into
/// the input buffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParsedPacket<'a> {
    /// Source/destination address (with reply flag if from PD).
    pub addr: Address,
    /// Decoded CTRL byte.
    pub ctrl: ControlByte,
    /// Security Control Block, if `CTRL.HAS_SCB`.
    pub scb: Option<ScbView<'a>>,
    /// Command or reply code byte.
    pub code: u8,
    /// Application data following the code byte (excluding any MAC trailer
    /// and the CRC/checksum).
    pub data: &'a [u8],
    /// 4-byte MAC, if SCS_15..18.
    pub mac: Option<[u8; MAC_LEN]>,
    /// Trailer kind.
    pub trailer: Trailer,
}

impl<'a> ParsedPacket<'a> {
    /// Parse a complete packet from `buf`.
    ///
    /// Returns the parsed packet view and the number of bytes consumed
    /// (which equals the value of the LEN field).
    pub fn parse(buf: &'a [u8]) -> Result<(Self, usize), Error> {
        if buf.len() < HEADER_LEN {
            return Err(Error::Truncated {
                have: buf.len(),
                need: HEADER_LEN,
            });
        }
        if buf[0] != SOM {
            return Err(Error::BadSom(buf[0]));
        }

        let addr = Address::from_raw(buf[1]);
        let len = u16::from_le_bytes([buf[2], buf[3]]) as usize;
        let ctrl = ControlByte::decode(buf[4])?;

        if buf.len() < len {
            return Err(Error::Truncated {
                have: buf.len(),
                need: len,
            });
        }
        let frame = &buf[..len];

        let trailer_len = if ctrl.use_crc() { 2 } else { 1 };
        if frame.len() < HEADER_LEN + 1 + trailer_len {
            return Err(Error::BadLength {
                declared: len,
                actual: frame.len(),
            });
        }

        let mut cursor = HEADER_LEN;

        let scb = if ctrl.has_scb() {
            let view = ScbView::parse(&frame[cursor..])?;
            cursor += view.wire_len();
            Some(view)
        } else {
            None
        };

        if cursor + 1 + trailer_len > frame.len() {
            return Err(Error::BadLength {
                declared: len,
                actual: frame.len(),
            });
        }
        let code = frame[cursor];
        cursor += 1;

        let mac_present = scb.map(|s| s.ty.has_mac()).unwrap_or(false);
        let mac_room = if mac_present { MAC_LEN } else { 0 };

        let payload_end =
            frame
                .len()
                .checked_sub(trailer_len + mac_room)
                .ok_or(Error::BadLength {
                    declared: len,
                    actual: frame.len(),
                })?;
        if payload_end < cursor {
            return Err(Error::BadLength {
                declared: len,
                actual: frame.len(),
            });
        }
        let data = &frame[cursor..payload_end];

        let mac = if mac_present {
            let m = &frame[payload_end..payload_end + MAC_LEN];
            let mut arr = [0u8; MAC_LEN];
            arr.copy_from_slice(m);
            Some(arr)
        } else {
            None
        };

        let trailer = if ctrl.use_crc() {
            let got = u16::from_le_bytes([frame[frame.len() - 2], frame[frame.len() - 1]]);
            let want = crc16(&frame[..frame.len() - 2]);
            if got != want {
                return Err(Error::BadCrc { got, want });
            }
            Trailer::Crc(got)
        } else {
            let got = frame[frame.len() - 1];
            let want = checksum8(&frame[..frame.len() - 1]);
            if got != want {
                return Err(Error::BadChecksum { got, want });
            }
            Trailer::Checksum(got)
        };

        Ok((
            Self {
                addr,
                ctrl,
                scb,
                code,
                data,
                mac,
                trailer,
            },
            len,
        ))
    }
}

#[cfg(feature = "alloc")]
mod alloc_impls {
    use super::*;
    use crate::packet::scb::Scb;
    use alloc::vec::Vec;

    /// Builder for a single OSDP packet.
    ///
    /// Caller fills in the high-level fields and calls
    /// [`PacketBuilder::encode`] (for non-secure packets) or
    /// [`PacketBuilder::encode_with_mac`] (for `SCS_15..=SCS_18`) to produce
    /// the bytes ready for transmission.
    #[derive(Debug, Clone)]
    pub struct PacketBuilder {
        /// Address byte (with reply flag if PD → ACU).
        pub addr: Address,
        /// CTRL byte (SQN + flags).
        pub ctrl: ControlByte,
        /// Security Control Block (None if `CTRL.HAS_SCB` is clear).
        pub scb: Option<Scb>,
        /// Command or reply code byte.
        pub code: u8,
        /// DATA payload (already encrypted, if SCS_17/18).
        pub data: Vec<u8>,
    }

    impl PacketBuilder {
        /// Build a packet with neither SCB nor MAC.
        pub fn plain(addr: Address, ctrl: ControlByte, code: u8, data: Vec<u8>) -> Self {
            Self {
                addr,
                ctrl,
                scb: None,
                code,
                data,
            }
        }

        /// Encode the packet without any MAC.
        ///
        /// If the CTRL byte has [`super::CtrlFlags::HAS_SCB`] set but the
        /// SCB type *would* require a MAC, the caller must use
        /// [`Self::encode_with_mac`] instead.
        pub fn encode(&self) -> Result<Vec<u8>, Error> {
            self.encode_inner(None::<fn(&[u8]) -> [u8; super::MAC_LEN]>)
        }

        /// Encode the packet, computing the MAC trailer with `mac_fn`.
        ///
        /// `mac_fn` receives the bytes from SOM through end-of-DATA (i.e.
        /// what the wire MAC is computed over) and returns the first 4 bytes
        /// of the resulting MAC.
        pub fn encode_with_mac<F>(&self, mac_fn: F) -> Result<Vec<u8>, Error>
        where
            F: FnOnce(&[u8]) -> [u8; super::MAC_LEN],
        {
            self.encode_inner(Some(mac_fn))
        }

        fn encode_inner<F>(&self, mac_fn: Option<F>) -> Result<Vec<u8>, Error>
        where
            F: FnOnce(&[u8]) -> [u8; super::MAC_LEN],
        {
            // Validate SCB <-> CTRL coherence.
            let scb_present = self.ctrl.has_scb();
            if scb_present != self.scb.is_some() {
                return Err(Error::BadControlByte(self.ctrl.encode()));
            }
            let mac_required = self.scb.as_ref().map(|s| s.ty.has_mac()).unwrap_or(false);
            if mac_required && mac_fn.is_none() {
                return Err(Error::BadMac);
            }

            let trailer_len = if self.ctrl.use_crc() { 2 } else { 1 };
            let mac_len = if mac_required { super::MAC_LEN } else { 0 };
            let scb_len = self.scb.as_ref().map(|s| s.data.len() + 2).unwrap_or(0);
            let total = HEADER_LEN + scb_len + 1 + self.data.len() + mac_len + trailer_len;

            if total > u16::MAX as usize {
                return Err(Error::BufferOverflow {
                    need: total,
                    have: u16::MAX as usize,
                });
            }

            let mut out = Vec::with_capacity(total);
            out.push(SOM);
            out.push(self.addr.as_byte());
            let len_bytes = (total as u16).to_le_bytes();
            out.push(len_bytes[0]);
            out.push(len_bytes[1]);
            out.push(self.ctrl.encode());

            if let Some(scb) = &self.scb {
                scb.encode(&mut out);
            }

            out.push(self.code);
            out.extend_from_slice(&self.data);

            if let Some(f) = mac_fn {
                let mac = f(&out);
                out.extend_from_slice(&mac);
            }

            if self.ctrl.use_crc() {
                let crc = crc16(&out);
                out.extend_from_slice(&crc.to_le_bytes());
            } else {
                out.push(checksum8(&out));
            }

            debug_assert_eq!(out.len(), total);
            Ok(out)
        }
    }
}

#[cfg(feature = "alloc")]
pub use alloc_impls::PacketBuilder;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::header::{CtrlFlags, Sqn};
    use crate::packet::scb::{Scb, ScsType};
    use alloc::vec::Vec;

    fn make_poll(addr: u8, sqn: u8) -> Vec<u8> {
        PacketBuilder::plain(
            Address::pd(addr).unwrap(),
            ControlByte::new(Sqn::new(sqn).unwrap(), CtrlFlags::USE_CRC),
            0x60, // POLL
            Vec::new(),
        )
        .encode()
        .unwrap()
    }

    #[test]
    fn roundtrip_poll() {
        let bytes = make_poll(0x05, 1);
        let (parsed, used) = ParsedPacket::parse(&bytes).unwrap();
        assert_eq!(used, bytes.len());
        assert_eq!(parsed.addr.pd_addr(), 0x05);
        assert_eq!(parsed.ctrl.sqn.value(), 1);
        assert!(parsed.ctrl.use_crc());
        assert_eq!(parsed.code, 0x60);
        assert!(parsed.data.is_empty());
        assert!(parsed.scb.is_none());
        assert!(parsed.mac.is_none());
    }

    #[test]
    fn roundtrip_with_data_checksum() {
        let bytes = PacketBuilder::plain(
            Address::pd(0x01).unwrap(),
            ControlByte::new(Sqn::new(2).unwrap(), CtrlFlags::empty()),
            0x6E, // COMSET
            alloc::vec![0x00, 0x80, 0x25, 0x00, 0x00],
        )
        .encode()
        .unwrap();

        let (parsed, used) = ParsedPacket::parse(&bytes).unwrap();
        assert_eq!(used, bytes.len());
        assert_eq!(parsed.code, 0x6E);
        assert_eq!(parsed.data, &[0x00, 0x80, 0x25, 0x00, 0x00]);
        assert!(matches!(parsed.trailer, Trailer::Checksum(_)));
    }

    #[test]
    fn roundtrip_with_scb_no_mac() {
        let scb = Scb::new(ScsType::Scs11, [0x00, 1, 2, 3, 4, 5, 6, 7, 8]);
        let bytes = PacketBuilder {
            addr: Address::pd(0x01).unwrap(),
            ctrl: ControlByte::new(
                Sqn::new(0).unwrap(),
                CtrlFlags::USE_CRC | CtrlFlags::HAS_SCB,
            ),
            scb: Some(scb.clone()),
            code: 0x76,
            data: Vec::new(),
        }
        .encode()
        .unwrap();

        let (parsed, _) = ParsedPacket::parse(&bytes).unwrap();
        let view = parsed.scb.unwrap();
        assert_eq!(view.ty, ScsType::Scs11);
        assert_eq!(view.data, scb.data.as_slice());
        assert!(parsed.mac.is_none());
    }

    #[test]
    fn roundtrip_with_scb_and_mac() {
        let mac_value = [0xDE, 0xAD, 0xBE, 0xEF];
        let scb = Scb::new(ScsType::Scs15, []);
        let bytes = PacketBuilder {
            addr: Address::pd(0x02).unwrap(),
            ctrl: ControlByte::new(
                Sqn::new(2).unwrap(),
                CtrlFlags::USE_CRC | CtrlFlags::HAS_SCB,
            ),
            scb: Some(scb),
            code: 0x60,
            data: alloc::vec![0xAA, 0xBB],
        }
        .encode_with_mac(|_| mac_value)
        .unwrap();

        let (parsed, _) = ParsedPacket::parse(&bytes).unwrap();
        assert_eq!(parsed.mac.unwrap(), mac_value);
        assert_eq!(parsed.data, &[0xAA, 0xBB]);
    }

    #[test]
    fn rejects_bad_som() {
        let mut bytes = make_poll(1, 1);
        bytes[0] = 0x52;
        assert!(matches!(
            ParsedPacket::parse(&bytes),
            Err(Error::BadSom(0x52))
        ));
    }

    #[test]
    fn rejects_truncated() {
        let bytes = [0x53u8, 0x01];
        assert!(matches!(
            ParsedPacket::parse(&bytes),
            Err(Error::Truncated { .. })
        ));
    }

    #[test]
    fn rejects_bad_crc() {
        let mut bytes = make_poll(1, 1);
        let n = bytes.len();
        bytes[n - 1] ^= 0xFF;
        assert!(matches!(
            ParsedPacket::parse(&bytes),
            Err(Error::BadCrc { .. })
        ));
    }

    #[test]
    fn rejects_bad_checksum() {
        let mut bytes = PacketBuilder::plain(
            Address::pd(0x01).unwrap(),
            ControlByte::new(Sqn::new(1).unwrap(), CtrlFlags::empty()),
            0x60,
            Vec::new(),
        )
        .encode()
        .unwrap();
        let n = bytes.len();
        bytes[n - 1] ^= 0xFF;
        assert!(matches!(
            ParsedPacket::parse(&bytes),
            Err(Error::BadChecksum { .. })
        ));
    }

    #[test]
    fn parser_total_on_random_garbage() {
        // Phase 1 invariant: parser is total. Should never panic.
        for seed in 0u64..256 {
            let mut x = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15);
            let mut buf = [0u8; 64];
            for b in buf.iter_mut() {
                x = x
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                *b = (x >> 33) as u8;
            }
            let _ = ParsedPacket::parse(&buf);
        }
    }
}
