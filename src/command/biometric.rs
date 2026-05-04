//! `osdp_BIOREAD` (`0x73`) and `osdp_BIOMATCH` (`0x74`).
//!
//! # Spec: §6.14, §6.15, Tables 24–25

use crate::error::Error;
use alloc::vec::Vec;

/// Biometric type code (Table 24).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
#[allow(missing_docs)]
pub enum BioType {
    NotSpecified = 0x00,
    RightThumb = 0x01,
    RightIndex = 0x02,
    RightMiddle = 0x03,
    RightRing = 0x04,
    RightLittle = 0x05,
    LeftThumb = 0x06,
    LeftIndex = 0x07,
    LeftMiddle = 0x08,
    LeftRing = 0x09,
    LeftLittle = 0x0A,
    RightIris = 0x0B,
    RightRetina = 0x0C,
    LeftIris = 0x0D,
    LeftRetina = 0x0E,
    Face = 0x0F,
    RightHandGeometry = 0x10,
    LeftHandGeometry = 0x11,
}

impl BioType {
    /// Parse from byte; preserves "NotSpecified" for unknowns to avoid losing
    /// data that the receiver ought to NAK with `0x07`.
    pub const fn from_byte(b: u8) -> Self {
        match b {
            0x00 => Self::NotSpecified,
            0x01 => Self::RightThumb,
            0x02 => Self::RightIndex,
            0x03 => Self::RightMiddle,
            0x04 => Self::RightRing,
            0x05 => Self::RightLittle,
            0x06 => Self::LeftThumb,
            0x07 => Self::LeftIndex,
            0x08 => Self::LeftMiddle,
            0x09 => Self::LeftRing,
            0x0A => Self::LeftLittle,
            0x0B => Self::RightIris,
            0x0C => Self::RightRetina,
            0x0D => Self::LeftIris,
            0x0E => Self::LeftRetina,
            0x0F => Self::Face,
            0x10 => Self::RightHandGeometry,
            0x11 => Self::LeftHandGeometry,
            _ => Self::NotSpecified,
        }
    }

    /// Raw byte.
    pub const fn as_byte(self) -> u8 {
        self as u8
    }
}

/// Biometric data format (Table 25).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
#[allow(missing_docs)]
pub enum BioFormat {
    NotSpecified = 0x00,
    FingerprintRawPgm = 0x01,
    FingerprintAnsi378 = 0x02,
}

impl BioFormat {
    /// Parse.
    pub const fn from_byte(b: u8) -> Self {
        match b {
            0x01 => Self::FingerprintRawPgm,
            0x02 => Self::FingerprintAnsi378,
            _ => Self::NotSpecified,
        }
    }

    /// Raw byte.
    pub const fn as_byte(self) -> u8 {
        self as u8
    }
}

/// `osdp_BIOREAD` body.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BioRead {
    /// Reader number.
    pub reader: u8,
    /// Biometric type to capture.
    pub bio_type: BioType,
    /// Format the PD should respond with.
    pub bio_format: BioFormat,
    /// Capture quality (0..=100).
    pub quality: u8,
}

impl BioRead {
    /// Encode.
    pub fn encode(&self) -> Result<Vec<u8>, Error> {
        Ok(alloc::vec![
            self.reader,
            self.bio_type.as_byte(),
            self.bio_format.as_byte(),
            self.quality,
        ])
    }

    /// Decode.
    pub fn decode(data: &[u8]) -> Result<Self, Error> {
        if data.len() != 4 {
            return Err(Error::MalformedPayload {
                code: 0x73,
                reason: "BIOREAD requires 4 bytes",
            });
        }
        Ok(Self {
            reader: data[0],
            bio_type: BioType::from_byte(data[1]),
            bio_format: BioFormat::from_byte(data[2]),
            quality: data[3],
        })
    }
}

/// `osdp_BIOMATCH` body. Carries a template to match against.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BioMatch {
    /// Reader number.
    pub reader: u8,
    /// Biometric type to capture.
    pub bio_type: BioType,
    /// Format of the template.
    pub bio_format: BioFormat,
    /// Quality threshold for the match.
    pub quality: u8,
    /// Template payload.
    pub template: Vec<u8>,
}

impl BioMatch {
    /// Encode.
    pub fn encode(&self) -> Result<Vec<u8>, Error> {
        if self.template.len() > u16::MAX as usize {
            return Err(Error::MalformedPayload {
                code: 0x74,
                reason: "BIOMATCH template > 65535 bytes",
            });
        }
        let mut out = Vec::with_capacity(6 + self.template.len());
        out.push(self.reader);
        out.push(self.bio_type.as_byte());
        out.push(self.bio_format.as_byte());
        out.push(self.quality);
        out.extend_from_slice(&(self.template.len() as u16).to_le_bytes());
        out.extend_from_slice(&self.template);
        Ok(out)
    }

    /// Decode.
    pub fn decode(data: &[u8]) -> Result<Self, Error> {
        if data.len() < 6 {
            return Err(Error::MalformedPayload {
                code: 0x74,
                reason: "BIOMATCH requires at least 6 bytes",
            });
        }
        let length = u16::from_le_bytes([data[4], data[5]]) as usize;
        if data.len() != 6 + length {
            return Err(Error::MalformedPayload {
                code: 0x74,
                reason: "BIOMATCH length disagrees with payload",
            });
        }
        Ok(Self {
            reader: data[0],
            bio_type: BioType::from_byte(data[1]),
            bio_format: BioFormat::from_byte(data[2]),
            quality: data[3],
            template: data[6..6 + length].to_vec(),
        })
    }
}
