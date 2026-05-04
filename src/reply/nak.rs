//! `osdp_NAK` (`0x41`).
//!
//! # Spec: §7.3, Table 47
//!
//! Body is `error_code (1 byte)` followed by an *optional* per-record
//! completion-code array (only when error code is `0x09`).

use crate::error::Error;
use alloc::vec::Vec;

/// NAK error code (Table 47).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
#[allow(missing_docs)]
pub enum NakErrorCode {
    NoError = 0x00,
    BadCrcOrChecksum = 0x01,
    CommandLengthError = 0x02,
    UnknownCommandCode = 0x03,
    UnexpectedSequenceNumber = 0x04,
    SecurityBlockTypeNotSupported = 0x05,
    EncryptedCommunicationRequired = 0x06,
    BioTypeNotSupported = 0x07,
    BioFormatNotSupported = 0x08,
    UnableToProcessCommandRecord = 0x09,
}

impl NakErrorCode {
    /// Parse from byte.
    pub const fn from_byte(b: u8) -> Result<Self, Error> {
        Ok(match b {
            0x00 => Self::NoError,
            0x01 => Self::BadCrcOrChecksum,
            0x02 => Self::CommandLengthError,
            0x03 => Self::UnknownCommandCode,
            0x04 => Self::UnexpectedSequenceNumber,
            0x05 => Self::SecurityBlockTypeNotSupported,
            0x06 => Self::EncryptedCommunicationRequired,
            0x07 => Self::BioTypeNotSupported,
            0x08 => Self::BioFormatNotSupported,
            0x09 => Self::UnableToProcessCommandRecord,
            _ => {
                return Err(Error::MalformedPayload {
                    code: 0x41,
                    reason: "unknown NAK error code",
                });
            }
        })
    }

    /// Raw byte.
    pub const fn as_byte(self) -> u8 {
        self as u8
    }
}

/// `osdp_NAK` body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Nak {
    /// Top-level error code.
    pub error: NakErrorCode,
    /// Per-record completion codes (only when `error == UnableToProcessCommandRecord`).
    pub completion_codes: Vec<u8>,
}

impl Nak {
    /// Build a simple NAK with no completion array.
    pub fn simple(err: NakErrorCode) -> Self {
        Self {
            error: err,
            completion_codes: Vec::new(),
        }
    }

    /// Encode.
    pub fn encode(&self) -> Result<Vec<u8>, Error> {
        let mut out = Vec::with_capacity(1 + self.completion_codes.len());
        out.push(self.error.as_byte());
        out.extend_from_slice(&self.completion_codes);
        Ok(out)
    }

    /// Decode. Treats unknown error codes as a parse failure rather than
    /// silently dropping the byte (NAK arriving with an unknown code is
    /// usually a sign of bus chaos worth surfacing).
    pub fn decode(data: &[u8]) -> Result<Self, Error> {
        if data.is_empty() {
            return Err(Error::MalformedPayload {
                code: 0x41,
                reason: "NAK requires at least 1 byte",
            });
        }
        let error = NakErrorCode::from_byte(data[0])?;
        let completion_codes = if error == NakErrorCode::UnableToProcessCommandRecord {
            data[1..].to_vec()
        } else {
            Vec::new()
        };
        Ok(Self {
            error,
            completion_codes,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_roundtrip() {
        let n = Nak::simple(NakErrorCode::BadCrcOrChecksum);
        let bytes = n.encode().unwrap();
        let parsed = Nak::decode(&bytes).unwrap();
        assert_eq!(parsed, n);
    }

    #[test]
    fn record_completion_codes() {
        let n = Nak {
            error: NakErrorCode::UnableToProcessCommandRecord,
            completion_codes: alloc::vec![0xAA, 0xBB, 0xCC],
        };
        let bytes = n.encode().unwrap();
        let parsed = Nak::decode(&bytes).unwrap();
        assert_eq!(parsed, n);
    }

    #[test]
    fn rejects_unknown_code() {
        assert!(Nak::decode(&[0xFE]).is_err());
    }
}
