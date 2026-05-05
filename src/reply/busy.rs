//! `osdp_BUSY` (`0x79`) — PD is busy. Empty body. SQN must always be `0`.
//!
//! # Spec: §7.18

use crate::error::Error;
use crate::payload_util::require_exact_len;
use alloc::vec::Vec;

/// `osdp_BUSY` body.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Busy;

impl Busy {
    /// Encode.
    pub fn encode(&self) -> Result<Vec<u8>, Error> {
        Ok(Vec::new())
    }

    /// Decode.
    pub fn decode(data: &[u8]) -> Result<Self, Error> {
        require_exact_len(data, 0, 0x79)?;
        Ok(Self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_empty() {
        assert!(Busy.encode().unwrap().is_empty());
        assert_eq!(Busy::decode(&[]).unwrap(), Busy);
    }

    #[test]
    fn decode_rejects_payload() {
        assert!(matches!(
            Busy::decode(&[0xFF]),
            Err(Error::PayloadLength { code: 0x79, .. })
        ));
    }
}
