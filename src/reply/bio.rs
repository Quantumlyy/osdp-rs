//! `osdp_BIOREADR` (`0x57`) and `osdp_BIOMATCHR` (`0x58`).
//!
//! # Spec: §7.14, §7.15

use crate::command::{BioFormat, BioType};
use crate::error::Error;
use crate::payload_util::{require_at_least, require_exact_len};
use alloc::vec::Vec;

/// `osdp_BIOREADR` body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BioReadR {
    /// Reader number.
    pub reader: u8,
    /// Biometric type captured.
    pub bio_type: BioType,
    /// Biometric format used.
    pub bio_format: BioFormat,
    /// Quality score (0..100) of the captured sample.
    pub quality: u8,
    /// Captured biometric data (template or raw).
    pub data: Vec<u8>,
}

impl BioReadR {
    /// Encode.
    pub fn encode(&self) -> Result<Vec<u8>, Error> {
        if self.data.len() > u16::MAX as usize {
            return Err(Error::MalformedPayload {
                code: 0x57,
                reason: "BIOREADR data > 65535 bytes",
            });
        }
        let mut out = Vec::with_capacity(6 + self.data.len());
        out.push(self.reader);
        out.push(self.bio_type.as_byte());
        out.push(self.bio_format.as_byte());
        out.push(self.quality);
        out.extend_from_slice(&(self.data.len() as u16).to_le_bytes());
        out.extend_from_slice(&self.data);
        Ok(out)
    }

    /// Decode.
    pub fn decode(data: &[u8]) -> Result<Self, Error> {
        require_at_least(data, 6, 0x57)?;
        let length = u16::from_le_bytes([data[4], data[5]]) as usize;
        if data.len() != 6 + length {
            return Err(Error::MalformedPayload {
                code: 0x57,
                reason: "BIOREADR length disagrees with payload",
            });
        }
        Ok(Self {
            reader: data[0],
            bio_type: BioType::from_byte(data[1]),
            bio_format: BioFormat::from_byte(data[2]),
            quality: data[3],
            data: data[6..6 + length].to_vec(),
        })
    }
}

/// `osdp_BIOMATCHR` body.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BioMatchR {
    /// Reader number.
    pub reader: u8,
    /// Match outcome (`0` = no match, `1` = match).
    pub result: u8,
    /// Score (0..100).
    pub score: u8,
}

impl BioMatchR {
    /// Encode.
    pub fn encode(&self) -> Result<Vec<u8>, Error> {
        Ok(alloc::vec![self.reader, self.result, self.score])
    }

    /// Decode.
    pub fn decode(data: &[u8]) -> Result<Self, Error> {
        require_exact_len(data, 3, 0x58)?;
        Ok(Self {
            reader: data[0],
            result: data[1],
            score: data[2],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bioreadr_roundtrip() {
        let body = BioReadR {
            reader: 0x01,
            bio_type: BioType::RightThumb,
            bio_format: BioFormat::FingerprintAnsi378,
            quality: 90,
            data: alloc::vec![0xDE, 0xAD],
        };
        let bytes = body.encode().unwrap();
        assert_eq!(
            bytes,
            [0x01, 0x01, 0x02, 90, 0x02, 0x00, 0xDE, 0xAD]
        );
        assert_eq!(BioReadR::decode(&bytes).unwrap(), body);
    }

    #[test]
    fn bioreadr_rejects_length_mismatch() {
        assert!(matches!(
            BioReadR::decode(&[0x01, 0x01, 0x02, 90, 0x05, 0x00, 0xAA]),
            Err(Error::MalformedPayload { code: 0x57, .. })
        ));
    }

    #[test]
    fn biomatchr_roundtrip() {
        let body = BioMatchR {
            reader: 0x00,
            result: 0x01,
            score: 95,
        };
        let bytes = body.encode().unwrap();
        assert_eq!(bytes, [0x00, 0x01, 95]);
        assert_eq!(BioMatchR::decode(&bytes).unwrap(), body);
    }

    #[test]
    fn biomatchr_rejects_wrong_length() {
        assert!(matches!(
            BioMatchR::decode(&[0x00, 0x01]),
            Err(Error::PayloadLength { code: 0x58, .. })
        ));
    }
}
