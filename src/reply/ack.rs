//! `osdp_ACK` (`0x40`). Empty body.
//!
//! # Spec: §7.1

use crate::error::Error;
use crate::payload_util::require_exact_len;
use alloc::vec::Vec;

/// `osdp_ACK` body.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Ack;

impl Ack {
    /// Encode (always empty).
    pub fn encode(&self) -> Result<Vec<u8>, Error> {
        Ok(Vec::new())
    }

    /// Decode.
    pub fn decode(data: &[u8]) -> Result<Self, Error> {
        require_exact_len(data, 0, 0x40)?;
        Ok(Self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_empty() {
        assert!(Ack.encode().unwrap().is_empty());
        assert_eq!(Ack::decode(&[]).unwrap(), Ack);
    }

    #[test]
    fn decode_rejects_payload() {
        assert!(matches!(
            Ack::decode(&[0x00]),
            Err(Error::PayloadLength { code: 0x40, .. })
        ));
    }
}
