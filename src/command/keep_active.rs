//! `osdp_KEEPACTIVE` (`0xA7`) — instruct the PD to keep its reader active.
//!
//! # Spec: §6.27
//!
//! Body is a 16-bit little-endian millisecond duration.

use crate::error::Error;
use crate::payload_util::require_exact_len;
use alloc::vec::Vec;

/// `osdp_KEEPACTIVE` body.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeepActive {
    /// Hold duration in milliseconds.
    pub duration_ms: u16,
}

impl KeepActive {
    /// Encode.
    pub fn encode(&self) -> Result<Vec<u8>, Error> {
        Ok(self.duration_ms.to_le_bytes().to_vec())
    }

    /// Decode.
    pub fn decode(data: &[u8]) -> Result<Self, Error> {
        require_exact_len(data, 2, 0xA7)?;
        Ok(Self {
            duration_ms: u16::from_le_bytes([data[0], data[1]]),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let body = KeepActive {
            duration_ms: 0x1234,
        };
        let bytes = body.encode().unwrap();
        assert_eq!(bytes, [0x34, 0x12]);
        assert_eq!(KeepActive::decode(&bytes).unwrap(), body);
    }

    #[test]
    fn decode_rejects_wrong_length() {
        assert!(matches!(
            KeepActive::decode(&[0x10]),
            Err(Error::PayloadLength { code: 0xA7, .. })
        ));
    }
}
