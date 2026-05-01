//! `osdp_KEEPACTIVE` (`0xA7`) — instruct the PD to keep its reader active.
//!
//! # Spec: §6.27
//!
//! Body is a 16-bit little-endian millisecond duration.

use crate::error::Error;
use alloc::vec::Vec;

/// `osdp_KEEPACTIVE` body.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeepActive {
    /// Hold duration in milliseconds.
    pub duration_ms: u16,
}

impl KeepActive {
    /// Encode.
    pub fn encode(&self) -> Result<Vec<u8>, Error> {
        Ok(self.duration_ms.to_le_bytes().to_vec())
    }

    /// Decode.
    pub fn decode(data: &[u8]) -> Result<Self, Error> {
        if data.len() != 2 {
            return Err(Error::MalformedPayload {
                code: 0xA7,
                reason: "KEEPACTIVE requires 2 bytes",
            });
        }
        Ok(Self {
            duration_ms: u16::from_le_bytes([data[0], data[1]]),
        })
    }
}
