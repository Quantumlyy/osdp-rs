//! `osdp_XRD` (`0xB1`) — extended-read response.
//!
//! # Spec: §7.26
//!
//! Body's first byte is `XRW_MODE` (matches the request); the remainder is
//! mode-specific.

use crate::error::Error;
use crate::payload_util::require_at_least;
use alloc::vec::Vec;

/// `osdp_XRD` body — opaque container.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Xrd {
    /// XRW_MODE byte.
    pub mode: u8,
    /// Mode-specific reply (sub-reply code + data).
    pub payload: Vec<u8>,
}

impl Xrd {
    /// Encode.
    pub fn encode(&self) -> Result<Vec<u8>, Error> {
        let mut out = Vec::with_capacity(1 + self.payload.len());
        out.push(self.mode);
        out.extend_from_slice(&self.payload);
        Ok(out)
    }

    /// Decode.
    pub fn decode(data: &[u8]) -> Result<Self, Error> {
        require_at_least(data, 1, 0xB1)?;
        Ok(Self {
            mode: data[0],
            payload: data[1..].to_vec(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let body = Xrd {
            mode: 0x01,
            payload: alloc::vec![0xCA, 0xFE],
        };
        let bytes = body.encode().unwrap();
        assert_eq!(bytes, [0x01, 0xCA, 0xFE]);
        assert_eq!(Xrd::decode(&bytes).unwrap(), body);
    }

    #[test]
    fn mode_only_is_valid() {
        let body = Xrd {
            mode: 0x00,
            payload: Vec::new(),
        };
        let bytes = body.encode().unwrap();
        assert_eq!(bytes, [0x00]);
        assert_eq!(Xrd::decode(&bytes).unwrap(), body);
    }

    #[test]
    fn empty_decode_rejected() {
        assert!(matches!(
            Xrd::decode(&[]),
            Err(Error::PayloadTooShort { code: 0xB1, .. })
        ));
    }
}
