//! `osdp_BUSY` (`0x79`) — PD is busy. Empty body. SQN must always be `0`.
//!
//! # Spec: §7.18

use crate::error::Error;
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
        if !data.is_empty() {
            return Err(Error::MalformedPayload {
                code: 0x79,
                reason: "BUSY has no payload",
            });
        }
        Ok(Self)
    }
}
