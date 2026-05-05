//! `osdp_XWR` (`0xA1`) — extended write.
//!
//! # Spec: §5.11, §6.20, Table 5
//!
//! Body's first byte is `XRW_MODE`. Subsequent layout depends on the mode:
//!
//! - `0x00` (default): generic mode-control sub-commands.
//! - `0x01` (transparent smart card): APDU transmission, secure PIN entry.

use crate::error::Error;
use alloc::vec::Vec;

/// Mode-00 sub-commands.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[allow(missing_docs)]
pub enum XwrMode00 {
    ReadMode = 0x01,
    SetMode = 0x02,
}

/// Mode-01 (transparent smart card) sub-commands.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[allow(missing_docs)]
pub enum XwrMode01 {
    TransmitApdu = 0x01,
    ConnectionDone = 0x02,
    SecurePinEntry = 0x03,
    SmartCardScan = 0x04,
}

/// `osdp_XWR` body — opaque container preserving mode + payload.
///
/// Higher-level helpers can interpret the payload according to `mode`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XWrite {
    /// XRW_MODE byte.
    pub mode: u8,
    /// Mode-specific payload (sub-command + data).
    pub payload: Vec<u8>,
}

impl XWrite {
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
                code: 0xA1,
                reason: "XWR requires XRW_MODE byte",
            });
        }
        Ok(Self {
            mode: data[0],
            payload: data[1..].to_vec(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let body = XWrite {
            mode: 0x01,
            payload: alloc::vec![0x02, 0xDE, 0xAD],
        };
        let bytes = body.encode().unwrap();
        assert_eq!(bytes, [0x01, 0x02, 0xDE, 0xAD]);
        assert_eq!(XWrite::decode(&bytes).unwrap(), body);
    }

    #[test]
    fn mode_only_is_valid() {
        let body = XWrite {
            mode: 0x00,
            payload: Vec::new(),
        };
        let bytes = body.encode().unwrap();
        assert_eq!(bytes, [0x00]);
        assert_eq!(XWrite::decode(&bytes).unwrap(), body);
    }

    #[test]
    fn empty_decode_rejected() {
        assert!(matches!(
            XWrite::decode(&[]),
            Err(Error::MalformedPayload { code: 0xA1, .. })
        ));
    }
}
