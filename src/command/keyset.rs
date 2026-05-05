//! `osdp_KEYSET` (`0x75`) — install encryption key.
//!
//! # Spec: §6.16
//!
//! Body layout:
//!
//! ```text
//! +---------+---------+----------+
//! | key_type| key_len | key data |
//! +---------+---------+----------+
//! ```
//!
//! `key_type` is currently always `0x01` (SCBK), `key_len` is the byte length
//! of the key (16 for AES-128).

use crate::error::Error;
use alloc::vec::Vec;

/// `osdp_KEYSET` body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeySet {
    /// Key type (0x01 = SCBK).
    pub key_type: u8,
    /// Key bytes.
    pub key: Vec<u8>,
}

impl KeySet {
    /// Build with the standard `key_type = 0x01` (SCBK) and a 16-byte key.
    pub fn scbk(key: [u8; 16]) -> Self {
        Self {
            key_type: 0x01,
            key: key.to_vec(),
        }
    }

    /// Encode.
    pub fn encode(&self) -> Result<Vec<u8>, Error> {
        if self.key.len() > u8::MAX as usize {
            return Err(Error::MalformedPayload {
                code: 0x75,
                reason: "KEYSET key length exceeds 255",
            });
        }
        let mut out = Vec::with_capacity(2 + self.key.len());
        out.push(self.key_type);
        out.push(self.key.len() as u8);
        out.extend_from_slice(&self.key);
        Ok(out)
    }

    /// Decode.
    pub fn decode(data: &[u8]) -> Result<Self, Error> {
        if data.len() < 2 {
            return Err(Error::MalformedPayload {
                code: 0x75,
                reason: "KEYSET requires at least 2 bytes",
            });
        }
        let key_len = data[1] as usize;
        if data.len() != 2 + key_len {
            return Err(Error::MalformedPayload {
                code: 0x75,
                reason: "KEYSET key length disagrees with payload",
            });
        }
        Ok(Self {
            key_type: data[0],
            key: data[2..2 + key_len].to_vec(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scbk_roundtrip() {
        let key = [0xAAu8; 16];
        let body = KeySet::scbk(key);
        let bytes = body.encode().unwrap();
        assert_eq!(bytes[0], 0x01);
        assert_eq!(bytes[1], 16);
        assert_eq!(&bytes[2..], &key);
        assert_eq!(KeySet::decode(&bytes).unwrap(), body);
    }

    #[test]
    fn decode_rejects_short() {
        assert!(matches!(
            KeySet::decode(&[0x01]),
            Err(Error::MalformedPayload { code: 0x75, .. })
        ));
    }

    #[test]
    fn decode_rejects_length_mismatch() {
        // key_len says 4 but only 2 key bytes follow.
        assert!(matches!(
            KeySet::decode(&[0x01, 0x04, 0xDE, 0xAD]),
            Err(Error::MalformedPayload { code: 0x75, .. })
        ));
    }
}
