//! `osdp_OSTATR` (`0x4A`) — output status report.
//!
//! # Spec: §7.8
//!
//! One byte per output (`0` = OFF, `1` = ON).

use crate::error::Error;
use alloc::vec::Vec;

/// `osdp_OSTATR` body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OStatR {
    /// Status of each output.
    pub outputs: Vec<u8>,
}

impl OStatR {
    /// Encode.
    pub fn encode(&self) -> Result<Vec<u8>, Error> {
        Ok(self.outputs.clone())
    }

    /// Decode.
    pub fn decode(data: &[u8]) -> Result<Self, Error> {
        Ok(Self {
            outputs: data.to_vec(),
        })
    }
}
