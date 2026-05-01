//! `osdp_MFG` (`0x80`) — manufacturer-specific command.
//!
//! # Spec: §6.22
//!
//! Body is `OUI (3 bytes)` + opaque vendor data.

use crate::error::Error;
use alloc::vec::Vec;

/// `osdp_MFG` body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mfg {
    /// IEEE-assigned OUI for the vendor.
    pub oui: [u8; 3],
    /// Vendor-specific payload.
    pub payload: Vec<u8>,
}

impl Mfg {
    /// Encode.
    pub fn encode(&self) -> Result<Vec<u8>, Error> {
        let mut out = Vec::with_capacity(3 + self.payload.len());
        out.extend_from_slice(&self.oui);
        out.extend_from_slice(&self.payload);
        Ok(out)
    }

    /// Decode.
    pub fn decode(data: &[u8]) -> Result<Self, Error> {
        if data.len() < 3 {
            return Err(Error::MalformedPayload {
                code: 0x80,
                reason: "MFG requires 3-byte OUI",
            });
        }
        let mut oui = [0u8; 3];
        oui.copy_from_slice(&data[..3]);
        Ok(Self {
            oui,
            payload: data[3..].to_vec(),
        })
    }
}
