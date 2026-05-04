//! `osdp_GENAUTHR` (`0x81`) — generic authenticate response (PIV).
//!
//! # Spec: §7.21

use crate::error::Error;
use alloc::vec::Vec;

/// `osdp_GENAUTHR` body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenAuthR {
    /// Authentication response template (TLV).
    pub response: Vec<u8>,
}

impl GenAuthR {
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
