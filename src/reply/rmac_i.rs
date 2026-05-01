//! `osdp_RMAC_I` (`0x78`) — initial R-MAC.
//!
//! # Spec: §7.17, Annex D.4

use crate::error::Error;
use alloc::vec::Vec;

/// `osdp_RMAC_I` body.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RMacI {
    /// Initial R-MAC seed (16 bytes).
    pub r_mac_i: [u8; 16],
}

impl RMacI {
    /// Encode.
    pub fn encode(&self) -> Result<Vec<u8>, Error> {
        Ok(self.r_mac_i.to_vec())
    }

    /// Decode.
    pub fn decode(data: &[u8]) -> Result<Self, Error> {
        if data.len() != 16 {
            return Err(Error::MalformedPayload {
                code: 0x78,
                reason: "RMAC_I requires 16 bytes",
            });
        }
        let mut r_mac_i = [0u8; 16];
        r_mac_i.copy_from_slice(data);
        Ok(Self { r_mac_i })
    }
}
