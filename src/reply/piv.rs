//! `osdp_PIVDATAR` (`0x80`) — PIV data response. Opaque body.
//!
//! # Spec: §7.20

use crate::error::Error;
use alloc::vec::Vec;

/// `osdp_PIVDATAR` body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PivDataR {
    /// Raw PIV data (often multi-part).
    pub data: Vec<u8>,
}

impl PivDataR {
    /// Encode.
    pub fn encode(&self) -> Result<Vec<u8>, Error> {
        Ok(self.data.clone())
    }

    /// Decode.
    pub fn decode(data: &[u8]) -> Result<Self, Error> {
        Ok(Self {
            data: data.to_vec(),
        })
    }
}
