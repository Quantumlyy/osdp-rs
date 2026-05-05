//! `osdp_FTSTAT` (`0x7A`) — file-transfer status.
//!
//! # Spec: §7.19
//!
//! Body: `status (1) || delay (2 LE) || preferred_size (2 LE) || flags (2 LE)`.

use crate::error::Error;
use crate::payload_util::require_exact_len;
use alloc::vec::Vec;

/// `osdp_FTSTAT` body.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FtStat {
    /// Status code.
    pub status: u8,
    /// Update delay (ms) before next fragment is welcome.
    pub delay_ms: u16,
    /// Preferred fragment size in bytes.
    pub preferred_size: u16,
    /// Bit-flags (vendor-specific extensions).
    pub flags: u16,
}

impl FtStat {
    /// Encode.
    pub fn encode(&self) -> Result<Vec<u8>, Error> {
        let mut out = Vec::with_capacity(7);
        out.push(self.status);
        out.extend_from_slice(&self.delay_ms.to_le_bytes());
        out.extend_from_slice(&self.preferred_size.to_le_bytes());
        out.extend_from_slice(&self.flags.to_le_bytes());
        Ok(out)
    }

    /// Decode.
    pub fn decode(data: &[u8]) -> Result<Self, Error> {
        require_exact_len(data, 7, 0x7A)?;
        Ok(Self {
            status: data[0],
            delay_ms: u16::from_le_bytes([data[1], data[2]]),
            preferred_size: u16::from_le_bytes([data[3], data[4]]),
            flags: u16::from_le_bytes([data[5], data[6]]),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let body = FtStat {
            status: 0x01,
            delay_ms: 0x0064,
            preferred_size: 0x0100,
            flags: 0x0003,
        };
        let bytes = body.encode().unwrap();
        assert_eq!(bytes, [0x01, 0x64, 0x00, 0x00, 0x01, 0x03, 0x00]);
        assert_eq!(FtStat::decode(&bytes).unwrap(), body);
    }

    #[test]
    fn decode_rejects_wrong_length() {
        assert!(matches!(
            FtStat::decode(&[0; 6]),
            Err(Error::PayloadLength { code: 0x7A, .. })
        ));
    }
}
