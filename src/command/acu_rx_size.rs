//! `osdp_ACURXSIZE` (`0x7B`) — inform the PD of the ACU's max receive size.
//!
//! # Spec: §6.19
//!
//! Body is a 16-bit little-endian byte count.

use crate::error::Error;
use alloc::vec::Vec;

/// `osdp_ACURXSIZE` body.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AcuRxSize {
    /// Bytes the ACU can receive in one packet.
    pub max_size: u16,
}

impl AcuRxSize {
    /// Encode.
    pub fn encode(&self) -> Result<Vec<u8>, Error> {
        Ok(self.max_size.to_le_bytes().to_vec())
    }

    /// Decode.
    pub fn decode(data: &[u8]) -> Result<Self, Error> {
        if data.len() != 2 {
            return Err(Error::MalformedPayload {
                code: 0x7B,
                reason: "ACURXSIZE requires 2 bytes",
            });
        }
        Ok(Self {
            max_size: u16::from_le_bytes([data[0], data[1]]),
        })
    }
}
