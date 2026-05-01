//! Security Control Block — the optional `SEC_BLK_*` field that follows the
//! [`ControlByte`](crate::ControlByte) when [`CtrlFlags::HAS_SCB`] is set.
//!
//! # Spec: §5.9, Annex D
//!
//! ```text
//! +------------+-----------+--------------+
//! | SEC_BLK_LEN| SEC_BLK_TY| SEC_BLK_DATA |
//! +------------+-----------+--------------+
//!  1 byte       1 byte      LEN-2 bytes
//! ```
//!
//! `SEC_BLK_LEN` covers the entire block including the LEN and TYPE bytes
//! themselves, so `SEC_BLK_DATA` is `SEC_BLK_LEN - 2` bytes long.

use crate::error::Error;

/// Security block type byte (`SEC_BLK_TYPE`).
///
/// # Spec: §5.9, Table 3 (cross-referenced via Annex D)
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ScsType {
    /// `SCS_11` — ACU → PD. Begin SC session, carries `RND.A`.
    Scs11 = 0x11,
    /// `SCS_12` — PD → ACU. cUID + `RND.B` + client cryptogram.
    Scs12 = 0x12,
    /// `SCS_13` — ACU → PD. Server cryptogram.
    Scs13 = 0x13,
    /// `SCS_14` — PD → ACU. Initial R-MAC.
    Scs14 = 0x14,
    /// `SCS_15` — ACU → PD. Secure message, MAC only.
    Scs15 = 0x15,
    /// `SCS_16` — PD → ACU. Secure reply, MAC only.
    Scs16 = 0x16,
    /// `SCS_17` — ACU → PD. Secure message, encrypted DATA + MAC.
    Scs17 = 0x17,
    /// `SCS_18` — PD → ACU. Secure reply, encrypted DATA + MAC.
    Scs18 = 0x18,
}

impl ScsType {
    /// Parse from byte.
    pub const fn from_byte(b: u8) -> Result<Self, Error> {
        Ok(match b {
            0x11 => Self::Scs11,
            0x12 => Self::Scs12,
            0x13 => Self::Scs13,
            0x14 => Self::Scs14,
            0x15 => Self::Scs15,
            0x16 => Self::Scs16,
            0x17 => Self::Scs17,
            0x18 => Self::Scs18,
            other => return Err(Error::BadSecurityBlock(other)),
        })
    }

    /// Raw byte value.
    pub const fn as_byte(self) -> u8 {
        self as u8
    }

    /// `true` if this SCS implies a 4-byte MAC trailer before the checksum / CRC.
    ///
    /// # Spec: §5.9
    pub const fn has_mac(self) -> bool {
        matches!(
            self,
            Self::Scs15 | Self::Scs16 | Self::Scs17 | Self::Scs18
        )
    }

    /// `true` if this SCS implies the DATA payload is AES-CBC encrypted.
    pub const fn is_encrypted(self) -> bool {
        matches!(self, Self::Scs17 | Self::Scs18)
    }

    /// `true` if this SCS is part of the handshake (no DATA encryption, no MAC).
    pub const fn is_handshake(self) -> bool {
        matches!(
            self,
            Self::Scs11 | Self::Scs12 | Self::Scs13 | Self::Scs14
        )
    }
}

/// Borrowed view of a parsed Security Control Block.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScbView<'a> {
    /// Block type.
    pub ty: ScsType,
    /// `SEC_BLK_DATA` — type-specific payload, `SEC_BLK_LEN - 2` bytes.
    pub data: &'a [u8],
}

impl<'a> ScbView<'a> {
    /// Length on the wire (includes LEN + TYPE bytes).
    pub fn wire_len(&self) -> usize {
        self.data.len() + 2
    }

    /// Decode SCB starting at the LEN byte.
    pub fn parse(buf: &'a [u8]) -> Result<Self, Error> {
        if buf.len() < 2 {
            return Err(Error::Truncated {
                have: buf.len(),
                need: 2,
            });
        }
        let len = buf[0] as usize;
        if len < 2 {
            return Err(Error::BadSecurityBlockLength(buf[1]));
        }
        if buf.len() < len {
            return Err(Error::Truncated {
                have: buf.len(),
                need: len,
            });
        }
        let ty = ScsType::from_byte(buf[1])?;
        Ok(Self {
            ty,
            data: &buf[2..len],
        })
    }
}

#[cfg(feature = "alloc")]
mod alloc_impls {
    use super::*;
    use alloc::vec::Vec;

    /// Owned Security Control Block.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Scb {
        /// Block type.
        pub ty: ScsType,
        /// `SEC_BLK_DATA`.
        pub data: Vec<u8>,
    }

    impl Scb {
        /// New block.
        pub fn new(ty: ScsType, data: impl Into<Vec<u8>>) -> Self {
            Self {
                ty,
                data: data.into(),
            }
        }

        /// Borrow as a [`ScbView`].
        pub fn as_view(&self) -> ScbView<'_> {
            ScbView {
                ty: self.ty,
                data: &self.data,
            }
        }

        /// Encode the SCB onto the back of `out`.
        pub fn encode(&self, out: &mut Vec<u8>) {
            let len = (self.data.len() + 2) as u8;
            out.push(len);
            out.push(self.ty.as_byte());
            out.extend_from_slice(&self.data);
        }
    }

    impl From<ScbView<'_>> for Scb {
        fn from(v: ScbView<'_>) -> Self {
            Self {
                ty: v.ty,
                data: v.data.to_vec(),
            }
        }
    }
}

#[cfg(feature = "alloc")]
pub use alloc_impls::Scb;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scs_type_roundtrip() {
        for byte in [0x11u8, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18] {
            assert_eq!(ScsType::from_byte(byte).unwrap().as_byte(), byte);
        }
    }

    #[test]
    fn scs_type_rejects_unknown() {
        assert!(ScsType::from_byte(0x10).is_err());
        assert!(ScsType::from_byte(0x19).is_err());
        assert!(ScsType::from_byte(0x00).is_err());
    }

    #[test]
    fn parse_minimal() {
        let buf = [0x02u8, 0x11];
        let scb = ScbView::parse(&buf).unwrap();
        assert_eq!(scb.ty, ScsType::Scs11);
        assert!(scb.data.is_empty());
        assert_eq!(scb.wire_len(), 2);
    }

    #[test]
    fn parse_with_data() {
        let buf = [0x05u8, 0x12, 0xAA, 0xBB, 0xCC];
        let scb = ScbView::parse(&buf).unwrap();
        assert_eq!(scb.ty, ScsType::Scs12);
        assert_eq!(scb.data, &[0xAA, 0xBB, 0xCC]);
    }

    #[test]
    fn parse_rejects_short_len() {
        assert!(ScbView::parse(&[0x01u8, 0x11]).is_err());
        assert!(ScbView::parse(&[]).is_err());
    }

    #[test]
    fn mac_predicate() {
        for ty in [ScsType::Scs15, ScsType::Scs16, ScsType::Scs17, ScsType::Scs18] {
            assert!(ty.has_mac());
        }
        for ty in [ScsType::Scs11, ScsType::Scs12, ScsType::Scs13, ScsType::Scs14] {
            assert!(!ty.has_mac());
        }
    }
}
