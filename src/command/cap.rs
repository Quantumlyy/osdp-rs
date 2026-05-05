//! `osdp_CAP` (`0x62`) — request PD capabilities list.
//!
//! # Spec: §6.3

use crate::error::Error;
use alloc::vec::Vec;

/// `osdp_CAP` body.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Cap {
    /// Reserved byte; spec mandates `0x00`.
    pub reserved: u8,
}

impl Cap {
    /// Standard request.
    pub const fn standard() -> Self {
        Self { reserved: 0 }
    }

    /// Encode.
    pub fn encode(&self) -> Result<Vec<u8>, Error> {
        Ok(alloc::vec![self.reserved])
    }

    /// Decode.
    pub fn decode(data: &[u8]) -> Result<Self, Error> {
        if data.len() != 1 {
            return Err(Error::MalformedPayload {
                code: 0x62,
                reason: "CAP requires 1-byte reserved field",
            });
        }
        Ok(Self { reserved: data[0] })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_encodes_zero() {
        assert_eq!(Cap::standard().encode().unwrap(), [0x00]);
    }

    #[test]
    fn roundtrip_preserves_reserved() {
        let body = Cap { reserved: 0x42 };
        let bytes = body.encode().unwrap();
        assert_eq!(bytes, [0x42]);
        assert_eq!(Cap::decode(&bytes).unwrap(), body);
    }

    #[test]
    fn decode_rejects_wrong_length() {
        assert!(matches!(
            Cap::decode(&[]),
            Err(Error::MalformedPayload { code: 0x62, .. })
        ));
        assert!(matches!(
            Cap::decode(&[0x00, 0x00]),
            Err(Error::MalformedPayload { code: 0x62, .. })
        ));
    }
}
