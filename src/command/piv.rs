//! PIV-related commands: `osdp_PIVDATA` (`0xA3`), `osdp_GENAUTH` (`0xA4`),
//! `osdp_CRAUTH` (`0xA5`).
//!
//! # Spec: §6.23 (PIVDATA), §6.24 (GENAUTH), §6.25 (CRAUTH)
//!
//! These commands typically use the multi-part envelope (Annex E examples)
//! when their payload exceeds a single packet's RX size.

use crate::error::Error;
use alloc::vec::Vec;

/// `osdp_PIVDATA` body.
///
/// Format follows Annex F of the spec; we treat the body as a typed selector
/// plus opaque payload for forward-compatibility with PIV variants.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PivData {
    /// PIV object tag (e.g. CHUID, CCC, PHOTO).
    pub object_id: [u8; 3],
    /// Element ID within the object.
    pub element_id: u8,
    /// Data offset (when fragmented).
    pub offset: u16,
    /// Trailing payload (often empty for read requests).
    pub data: Vec<u8>,
}

impl PivData {
    /// Encode.
    pub fn encode(&self) -> Result<Vec<u8>, Error> {
        let mut out = Vec::with_capacity(6 + self.data.len());
        out.extend_from_slice(&self.object_id);
        out.push(self.element_id);
        out.extend_from_slice(&self.offset.to_le_bytes());
        out.extend_from_slice(&self.data);
        Ok(out)
    }

    /// Decode.
    pub fn decode(data: &[u8]) -> Result<Self, Error> {
        if data.len() < 6 {
            return Err(Error::MalformedPayload {
                code: 0xA3,
                reason: "PIVDATA requires at least 6 bytes",
            });
        }
        let mut object_id = [0u8; 3];
        object_id.copy_from_slice(&data[..3]);
        Ok(Self {
            object_id,
            element_id: data[3],
            offset: u16::from_le_bytes([data[4], data[5]]),
            data: data[6..].to_vec(),
        })
    }
}

/// `osdp_GENAUTH` body. Generic authenticate sub-command for PIV.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenAuth {
    /// Algorithm reference (PIV §3.2.4).
    pub algorithm: u8,
    /// Key reference.
    pub key_ref: u8,
    /// Encoded TLV authentication template.
    pub auth_template: Vec<u8>,
}

impl GenAuth {
    /// Encode.
    pub fn encode(&self) -> Result<Vec<u8>, Error> {
        let mut out = Vec::with_capacity(2 + self.auth_template.len());
        out.push(self.algorithm);
        out.push(self.key_ref);
        out.extend_from_slice(&self.auth_template);
        Ok(out)
    }

    /// Decode.
    pub fn decode(data: &[u8]) -> Result<Self, Error> {
        if data.len() < 2 {
            return Err(Error::MalformedPayload {
                code: 0xA4,
                reason: "GENAUTH requires at least 2 bytes",
            });
        }
        Ok(Self {
            algorithm: data[0],
            key_ref: data[1],
            auth_template: data[2..].to_vec(),
        })
    }
}

/// `osdp_CRAUTH` body. Crypto challenge.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrAuth {
    /// Challenge nonce (typically 16 bytes).
    pub challenge: Vec<u8>,
}

impl CrAuth {
    /// Encode.
    pub fn encode(&self) -> Result<Vec<u8>, Error> {
        Ok(self.challenge.clone())
    }

    /// Decode.
    pub fn decode(data: &[u8]) -> Result<Self, Error> {
        Ok(Self {
            challenge: data.to_vec(),
        })
    }
}
