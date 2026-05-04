//! `osdp_LSTATR` (`0x48`) — local status report.
//!
//! # Spec: §7.6
//!
//! Body is exactly 2 bytes: `tamper`, `power`. `0` means normal, `1` means a
//! fault.

use crate::error::Error;
use alloc::vec::Vec;

/// `osdp_LSTATR` body.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LStatR {
    /// Tamper switch state. `0` = normal, `1` = tamper.
    pub tamper: u8,
    /// Power state. `0` = normal, `1` = power failure.
    pub power: u8,
}

impl LStatR {
    /// Encode.
    pub fn encode(&self) -> Result<Vec<u8>, Error> {
        Ok(alloc::vec![self.tamper, self.power])
    }

    /// Decode.
    pub fn decode(data: &[u8]) -> Result<Self, Error> {
        if data.len() != 2 {
            return Err(Error::MalformedPayload {
                code: 0x48,
                reason: "LSTATR requires 2 bytes",
            });
        }
        Ok(Self {
            tamper: data[0],
            power: data[1],
        })
    }
}
