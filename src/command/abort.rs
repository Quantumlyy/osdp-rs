//! `osdp_ABORT` (`0xA2`) — abort current operation. Empty body.
//!
//! # Spec: §6.24

use crate::error::Error;
use crate::payload_util::require_exact_len;
use alloc::vec::Vec;

/// `osdp_ABORT` body (empty).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Abort;

impl Abort {
    /// Encode.
    pub fn encode(&self) -> Result<Vec<u8>, Error> {
        Ok(Vec::new())
    }

    /// Decode.
    pub fn decode(data: &[u8]) -> Result<Self, Error> {
        require_exact_len(data, 0, 0xA2)?;
        Ok(Self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_empty() {
        assert!(Abort.encode().unwrap().is_empty());
        assert_eq!(Abort::decode(&[]).unwrap(), Abort);
    }

    #[test]
    fn decode_rejects_payload() {
        assert!(matches!(
            Abort::decode(&[0xFF]),
            Err(Error::PayloadLength { code: 0xA2, .. })
        ));
    }
}
