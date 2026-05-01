//! `osdp_SCRYPT` (`0x77`) — server cryptogram.
//!
//! # Spec: §6.18, Annex D.4
//!
//! Body is the 16-byte server cryptogram. Packet carries SCB of type `SCS_13`.

use crate::error::Error;
use alloc::vec::Vec;

/// `osdp_SCRYPT` body.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SCrypt {
    /// Server cryptogram.
    pub server_cryptogram: [u8; 16],
}

impl SCrypt {
    /// New.
    pub const fn new(c: [u8; 16]) -> Self {
        Self { server_cryptogram: c }
    }

    /// Encode.
    pub fn encode(&self) -> Result<Vec<u8>, Error> {
        Ok(self.server_cryptogram.to_vec())
    }

    /// Decode.
    pub fn decode(data: &[u8]) -> Result<Self, Error> {
        if data.len() != 16 {
            return Err(Error::MalformedPayload {
                code: 0x77,
                reason: "SCRYPT requires 16-byte cryptogram",
            });
        }
        let mut c = [0u8; 16];
        c.copy_from_slice(data);
        Ok(Self { server_cryptogram: c })
    }
}
