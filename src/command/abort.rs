//! `osdp_ABORT` (`0xA2`) — abort current operation. Empty body.
//!
//! # Spec: §6.24

use crate::error::Error;
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
        if !data.is_empty() {
            return Err(Error::MalformedPayload {
                code: 0xA2,
                reason: "ABORT has no payload",
            });
        }
        Ok(Self)
    }
}
