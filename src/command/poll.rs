//! `osdp_POLL` (`0x60`) — general inquiry; no payload.
//!
//! # Spec: §6.1

use crate::error::Error;
use crate::payload_util::require_exact_len;
use alloc::vec::Vec;

/// Empty body of the POLL command.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Poll;

impl Poll {
    /// Encode (always empty).
    pub fn encode(&self) -> Result<Vec<u8>, Error> {
        Ok(Vec::new())
    }

    /// Decode (must be empty).
    pub fn decode(data: &[u8]) -> Result<Self, Error> {
        require_exact_len(data, 0, 0x60)?;
        Ok(Self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_is_empty() {
        assert!(Poll.encode().unwrap().is_empty());
    }

    #[test]
    fn decode_empty() {
        assert_eq!(Poll::decode(&[]).unwrap(), Poll);
    }

    #[test]
    fn decode_rejects_payload() {
        assert!(matches!(
            Poll::decode(&[0x00]),
            Err(Error::PayloadLength { code: 0x60, .. })
        ));
    }
}
