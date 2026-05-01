//! `osdp_RSTATR` (`0x4B`) — reader (tamper) status report.
//!
//! # Spec: §7.9
//!
//! One byte per reader (`0` = normal, `1` = tamper, `2` = disconnected).

use crate::error::Error;
use alloc::vec::Vec;

/// `osdp_RSTATR` body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RStatR {
    /// Status of each reader.
    pub readers: Vec<u8>,
}

impl RStatR {
    /// Encode.
    pub fn encode(&self) -> Result<Vec<u8>, Error> {
        Ok(self.readers.clone())
    }

    /// Decode.
    pub fn decode(data: &[u8]) -> Result<Self, Error> {
        Ok(Self {
            readers: data.to_vec(),
        })
    }
}
