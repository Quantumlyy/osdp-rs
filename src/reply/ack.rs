//! `osdp_ACK` (`0x40`). Empty body.
//!
//! # Spec: §7.1

use crate::error::Error;
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
        if !data.is_empty() {
            return Err(Error::MalformedPayload {
                code: 0x40,
                reason: "ACK has no payload",
            });
        }
        Ok(Self)
    }
}
