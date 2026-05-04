//! `osdp_ID` (`0x61`) — request PD identification.
//!
//! # Spec: §6.2
//!
//! Body is a single `Reserved` byte (always `0x00`).

use crate::error::Error;
use alloc::vec::Vec;

/// `osdp_ID` body.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Id {
    /// Reserved byte; spec mandates `0x00`.
    pub reserved: u8,
}

impl Id {
    /// Standard request: `reserved = 0x00`.
    pub const fn standard() -> Self {
        Self { reserved: 0 }
    }

    /// Encode.
    pub fn encode(&self) -> Result<Vec<u8>, Error> {
        Ok(alloc::vec![self.reserved])
    }

    /// Decode.
    pub fn decode(data: &[u8]) -> Result<Self, Error> {
        if data.len() != 1 {
            return Err(Error::MalformedPayload {
                code: 0x61,
                reason: "ID requires 1-byte reserved field",
            });
        }
        Ok(Self { reserved: data[0] })
    }
}
