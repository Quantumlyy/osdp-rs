//! `osdp_CRAUTHR` (`0x82`) — challenge response.
//!
//! # Spec: §7.22

use crate::error::Error;
use alloc::vec::Vec;

/// `osdp_CRAUTHR` body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrAuthR {
    /// Challenge response payload.
    pub response: Vec<u8>,
}

impl CrAuthR {
    /// Encode.
    pub fn encode(&self) -> Result<Vec<u8>, Error> {
        Ok(self.response.clone())
    }

    /// Decode.
    pub fn decode(data: &[u8]) -> Result<Self, Error> {
        Ok(Self {
            response: data.to_vec(),
        })
    }
}
