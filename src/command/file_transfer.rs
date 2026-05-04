//! `osdp_FILETRANSFER` (`0x7C`) — chunked file upload to the PD.
//!
//! # Spec: §6.21

use crate::error::Error;
use alloc::vec::Vec;

/// `osdp_FILETRANSFER` body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileTransfer {
    /// Vendor-specific file type.
    pub file_type: u8,
    /// Total file size, in bytes.
    pub total_size: u32,
    /// Byte offset of this fragment.
    pub offset: u32,
    /// File fragment.
    pub fragment: Vec<u8>,
}

impl FileTransfer {
    /// Encode.
    pub fn encode(&self) -> Result<Vec<u8>, Error> {
        if self.fragment.len() > u16::MAX as usize {
            return Err(Error::MalformedPayload {
                code: 0x7C,
                reason: "FILETRANSFER fragment > 65535 bytes",
            });
        }
        let mut out = Vec::with_capacity(11 + self.fragment.len());
        out.push(self.file_type);
        out.extend_from_slice(&self.total_size.to_le_bytes());
        out.extend_from_slice(&self.offset.to_le_bytes());
        out.extend_from_slice(&(self.fragment.len() as u16).to_le_bytes());
        out.extend_from_slice(&self.fragment);
        Ok(out)
    }

    /// Decode.
    pub fn decode(data: &[u8]) -> Result<Self, Error> {
        if data.len() < 11 {
            return Err(Error::MalformedPayload {
                code: 0x7C,
                reason: "FILETRANSFER requires at least 11 bytes",
            });
        }
        let total_size = u32::from_le_bytes([data[1], data[2], data[3], data[4]]);
        let offset = u32::from_le_bytes([data[5], data[6], data[7], data[8]]);
        let frag_len = u16::from_le_bytes([data[9], data[10]]) as usize;
        if data.len() != 11 + frag_len {
            return Err(Error::MalformedPayload {
                code: 0x7C,
                reason: "FILETRANSFER fragment length disagrees with payload",
            });
        }
        Ok(Self {
            file_type: data[0],
            total_size,
            offset,
            fragment: data[11..11 + frag_len].to_vec(),
        })
    }
}
