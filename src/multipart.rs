//! Multi-part message engine.
//!
//! # Spec: §5.10, Table 4
//!
//! When a command or reply payload is larger than a single packet supports,
//! the data block is prefixed with a 6-byte multi-part header:
//!
//! ```text
//! +---------------+--------------+--------------------+
//! | MpSizeTotal   | MpOffset     | MpFragmentSize     |
//! | (u16 LE)      | (u16 LE)     | (u16 LE)           |
//! +---------------+--------------+--------------------+
//! ```
//!
//! Rules enforced by this module:
//! - First fragment is always at offset 0.
//! - Fragments are strictly sequential (no gaps, no reorder).
//! - All fragments report the same `MpSizeTotal`.
//! - Sender may abort by setting `MpOffset >= MpSizeTotal` and
//!   `MpFragmentSize = 0`.
//!
//! See [`flow`] for a rendered fragment-exchange diagram.

use crate::error::{Error, MultipartError};

/// Rendered diagram of a multi-part transfer.
///
/// Each fragment is a normal OSDP packet whose data block is prefixed with a
/// [`MultipartHeader`]. The receiver tracks the next-expected offset and
/// rejects any out-of-order, overlapping, or total-changing fragment.
///
#[cfg_attr(feature = "_docs", aquamarine::aquamarine)]
/// ```mermaid
/// sequenceDiagram
///     participant TX as MultipartTx<br/>(splitter)
///     participant RX as MultipartRx<br/>(assembler)
///     TX->>RX: hdr(total=N, offset=0,         frag=k₀) ‖ payload₀
///     RX->>RX: state = AwaitOffset(k₀)
///     TX->>RX: hdr(total=N, offset=k₀,        frag=k₁) ‖ payload₁
///     RX->>RX: state = AwaitOffset(k₀+k₁)
///     TX->>RX: hdr(total=N, offset=k₀+k₁,     frag=k₂) ‖ payload₂
///     RX-->>TX: complete (reassembled body, N bytes)
///     Note over TX,RX: TX may abort by sending<br/>hdr(total=N, offset≥N, frag=0)
/// ```
#[cfg(feature = "alloc")]
pub mod flow {}

/// Multi-part header on the wire.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MultipartHeader {
    /// Total combined size of all fragments.
    pub total: u16,
    /// Byte offset of this fragment's data within the combined transfer.
    pub offset: u16,
    /// Length of this fragment's data.
    pub fragment: u16,
}

impl MultipartHeader {
    /// Encoded size on the wire.
    pub const WIRE_LEN: usize = 6;

    /// Encode into a 6-byte array.
    pub const fn encode(self) -> [u8; Self::WIRE_LEN] {
        let t = self.total.to_le_bytes();
        let o = self.offset.to_le_bytes();
        let f = self.fragment.to_le_bytes();
        [t[0], t[1], o[0], o[1], f[0], f[1]]
    }

    /// Decode from a 6-byte array.
    pub fn decode(bytes: &[u8]) -> Result<Self, Error> {
        if bytes.len() < Self::WIRE_LEN {
            return Err(Error::Truncated {
                have: bytes.len(),
                need: Self::WIRE_LEN,
            });
        }
        Ok(Self {
            total: u16::from_le_bytes([bytes[0], bytes[1]]),
            offset: u16::from_le_bytes([bytes[2], bytes[3]]),
            fragment: u16::from_le_bytes([bytes[4], bytes[5]]),
        })
    }

    /// `true` if the header signals an abort.
    pub const fn is_abort(self) -> bool {
        self.fragment == 0 && self.offset >= self.total
    }
}

#[cfg(feature = "alloc")]
mod alloc_impls {
    use super::*;
    use alloc::vec::Vec;

    /// Iterator-style splitter for outgoing multi-part transfers.
    #[derive(Debug, Clone)]
    pub struct MultipartTx<'a> {
        body: &'a [u8],
        offset: u16,
        fragment_size: u16,
    }

    impl<'a> MultipartTx<'a> {
        /// New splitter. `fragment_size` is the maximum payload size that
        /// fits into a single packet (excluding the 6-byte multi-part
        /// header).
        pub fn new(body: &'a [u8], fragment_size: u16) -> Result<Self, Error> {
            if fragment_size == 0 {
                return Err(Error::Multipart(MultipartError::BadFragmentSize));
            }
            if body.len() > u16::MAX as usize {
                return Err(Error::BufferOverflow {
                    need: body.len(),
                    have: u16::MAX as usize,
                });
            }
            Ok(Self {
                body,
                offset: 0,
                fragment_size,
            })
        }

        /// Total transfer size.
        pub fn total(&self) -> u16 {
            self.body.len() as u16
        }

        /// Bytes still queued.
        pub fn remaining(&self) -> u16 {
            self.total().saturating_sub(self.offset)
        }
    }

    impl<'a> Iterator for MultipartTx<'a> {
        type Item = (MultipartHeader, &'a [u8]);

        fn next(&mut self) -> Option<Self::Item> {
            if self.offset as usize >= self.body.len() {
                return None;
            }
            let take =
                (self.body.len() - self.offset as usize).min(self.fragment_size as usize) as u16;
            let frag = &self.body[self.offset as usize..self.offset as usize + take as usize];
            let header = MultipartHeader {
                total: self.body.len() as u16,
                offset: self.offset,
                fragment: take,
            };
            self.offset += take;
            Some((header, frag))
        }
    }

    /// Reassembler for incoming multi-part transfers.
    #[derive(Debug, Clone, Default)]
    pub struct MultipartRx {
        buf: Vec<u8>,
        total: Option<u16>,
    }

    impl MultipartRx {
        /// New empty reassembler.
        pub fn new() -> Self {
            Self::default()
        }

        /// Reset state, dropping any partial transfer.
        pub fn reset(&mut self) {
            self.buf.clear();
            self.total = None;
        }

        /// Feed one fragment. Returns `Ok(Some(_))` once the transfer is
        /// complete (and resets internal state), `Ok(None)` if more
        /// fragments are required.
        pub fn push(
            &mut self,
            header: MultipartHeader,
            data: &[u8],
        ) -> Result<Option<Vec<u8>>, Error> {
            if header.is_abort() {
                self.reset();
                return Err(Error::Multipart(MultipartError::Aborted));
            }

            if header.fragment as usize != data.len() {
                return Err(Error::Multipart(MultipartError::BadFragmentSize));
            }

            match self.total {
                None => {
                    if header.offset != 0 {
                        return Err(Error::Multipart(MultipartError::UnexpectedFirstOffset(
                            header.offset,
                        )));
                    }
                    self.total = Some(header.total);
                    self.buf.clear();
                    self.buf.reserve(header.total as usize);
                }
                Some(t) if t != header.total => {
                    return Err(Error::Multipart(MultipartError::InconsistentTotal {
                        first: t,
                        now: header.total,
                    }));
                }
                _ => {
                    if header.offset as usize != self.buf.len() {
                        return Err(Error::Multipart(MultipartError::OutOfOrderOffset {
                            expected: self.buf.len() as u16,
                            got: header.offset,
                        }));
                    }
                }
            }

            if (header.offset as usize) + data.len() > header.total as usize {
                return Err(Error::Multipart(MultipartError::OverflowsTotal));
            }

            self.buf.extend_from_slice(data);

            if self.buf.len() == header.total as usize {
                let total = header.total;
                let mut out = core::mem::take(&mut self.buf);
                self.total = None;
                out.truncate(total as usize);
                Ok(Some(out))
            } else {
                Ok(None)
            }
        }

        /// Bytes received so far (only meaningful while a transfer is in progress).
        pub fn progress(&self) -> usize {
            self.buf.len()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn split_then_assemble() {
            let body: Vec<u8> = (0u16..1024).flat_map(|n| n.to_le_bytes()).collect();
            let tx = MultipartTx::new(&body, 200).unwrap();
            let mut rx = MultipartRx::new();
            let mut out = None;
            for (h, frag) in tx {
                if let Some(v) = rx.push(h, frag).unwrap() {
                    out = Some(v);
                }
            }
            assert_eq!(out.unwrap(), body);
        }

        #[test]
        fn reject_out_of_order() {
            let mut rx = MultipartRx::new();
            let h0 = MultipartHeader {
                total: 8,
                offset: 0,
                fragment: 4,
            };
            rx.push(h0, &[0u8; 4]).unwrap();
            let bogus = MultipartHeader {
                total: 8,
                offset: 6,
                fragment: 2,
            };
            assert!(rx.push(bogus, &[0u8; 2]).is_err());
        }

        #[test]
        fn reject_inconsistent_total() {
            let mut rx = MultipartRx::new();
            let h0 = MultipartHeader {
                total: 8,
                offset: 0,
                fragment: 4,
            };
            rx.push(h0, &[0u8; 4]).unwrap();
            let bogus = MultipartHeader {
                total: 12,
                offset: 4,
                fragment: 4,
            };
            assert!(rx.push(bogus, &[0u8; 4]).is_err());
        }

        #[test]
        fn abort_signal() {
            let mut rx = MultipartRx::new();
            let h0 = MultipartHeader {
                total: 8,
                offset: 0,
                fragment: 4,
            };
            rx.push(h0, &[0u8; 4]).unwrap();
            let abort = MultipartHeader {
                total: 8,
                offset: 8,
                fragment: 0,
            };
            assert!(matches!(
                rx.push(abort, &[]),
                Err(Error::Multipart(MultipartError::Aborted))
            ));
        }

        #[test]
        fn header_roundtrip() {
            let h = MultipartHeader {
                total: 0x1234,
                offset: 0x0050,
                fragment: 0x0010,
            };
            let bytes = h.encode();
            assert_eq!(MultipartHeader::decode(&bytes).unwrap(), h);
        }
    }
}

#[cfg(feature = "alloc")]
pub use alloc_impls::{MultipartRx, MultipartTx};
