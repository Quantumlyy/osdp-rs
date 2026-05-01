//! `osdp_POLL` (`0x60`) — general inquiry; no payload.
//!
//! # Spec: §6.1

use crate::error::Error;
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
        if !data.is_empty() {
            return Err(Error::MalformedPayload {
                code: 0x60,
                reason: "POLL has no payload",
            });
        }
        Ok(Self)
    }
}
