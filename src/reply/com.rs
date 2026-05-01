//! `osdp_COM` (`0x54`) — communications configuration report.
//!
//! # Spec: §7.13
//!
//! Body is 5 bytes: `address (1) + baud (4 LE)` — same layout as
//! `osdp_COMSET`.

use crate::error::Error;
use alloc::vec::Vec;

/// `osdp_COM` body.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Com {
    /// PD's address.
    pub address: u8,
    /// PD's baud rate.
    pub baud: u32,
}

impl Com {
    /// Encode.
    pub fn encode(&self) -> Result<Vec<u8>, Error> {
        let mut out = Vec::with_capacity(5);
        out.push(self.address);
        out.extend_from_slice(&self.baud.to_le_bytes());
        Ok(out)
    }

    /// Decode.
    pub fn decode(data: &[u8]) -> Result<Self, Error> {
        if data.len() != 5 {
            return Err(Error::MalformedPayload {
                code: 0x54,
                reason: "COM requires 5 bytes",
            });
        }
        Ok(Self {
            address: data[0],
            baud: u32::from_le_bytes([data[1], data[2], data[3], data[4]]),
        })
    }
}
