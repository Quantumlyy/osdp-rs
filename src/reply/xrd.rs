//! `osdp_XRD` (`0xB1`) — extended-read response.
//!
//! # Spec: §7.26
//!
//! Body's first byte is `XRW_MODE` (matches the request); the remainder is
//! mode-specific.

use crate::error::Error;
use alloc::vec::Vec;

/// `osdp_XRD` body — opaque container.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Xrd {
    /// XRW_MODE byte.
    pub mode: u8,
    /// Mode-specific reply (sub-reply code + data).
    pub payload: Vec<u8>,
}

impl Xrd {
    /// Encode.
    pub fn encode(&self) -> Result<Vec<u8>, Error> {
        let mut out = Vec::with_capacity(1 + self.payload.len());
        out.push(self.mode);
        out.extend_from_slice(&self.payload);
        Ok(out)
    }

    /// Decode.
    pub fn decode(data: &[u8]) -> Result<Self, Error> {
        if data.is_empty() {
            return Err(Error::MalformedPayload {
                code: 0xB1,
                reason: "XRD requires XRW_MODE byte",
            });
        }
        Ok(Self {
            mode: data[0],
            payload: data[1..].to_vec(),
        })
    }
}
