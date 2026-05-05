//! `osdp_ID` (`0x61`) — request PD identification.
//!
//! # Spec: §6.2
//!
//! Body is a single `Reserved` byte (always `0x00`).

use crate::error::Error;
use crate::payload_util::require_exact_len;
use alloc::vec::Vec;

/// `osdp_ID` body.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Id {
    /// Reserved byte; spec mandates `0x00`.
    pub reserved: u8,
}

impl Id {
    /// Standard request: `reserved = 0x00`.
    pub const fn standard() -> Self {
        Self { reserved: 0 }
    }

    /// Encode.
    pub fn encode(&self) -> Result<Vec<u8>, Error> {
        Ok(alloc::vec![self.reserved])
    }

    /// Decode.
    pub fn decode(data: &[u8]) -> Result<Self, Error> {
        require_exact_len(data, 1, 0x61)?;
        Ok(Self { reserved: data[0] })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_encodes_zero() {
        assert_eq!(Id::standard().encode().unwrap(), [0x00]);
    }

    #[test]
    fn roundtrip_preserves_reserved() {
        let body = Id { reserved: 0x55 };
        let bytes = body.encode().unwrap();
        assert_eq!(bytes, [0x55]);
        assert_eq!(Id::decode(&bytes).unwrap(), body);
    }

    #[test]
    fn decode_rejects_wrong_length() {
        assert!(matches!(
            Id::decode(&[]),
            Err(Error::PayloadLength { code: 0x61, .. })
        ));
    }
}
