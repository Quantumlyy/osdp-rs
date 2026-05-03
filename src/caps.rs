//! PD capability function codes.
//!
//! # Spec: Annex B
//!
//! Each entry in an `osdp_PDCAP` reply is a 3-byte tuple
//! `(function_code, compliance, number_of)`. The `function_code` is one of
//! the values in [`FunctionCode`] and the meaning of `compliance` /
//! `number_of` is function-specific.

use crate::error::Error;

/// Capability function code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum FunctionCode {
    /// Contact-status monitoring.
    ContactStatus = 1,
    /// Output control.
    OutputControl = 2,
    /// Card-data format.
    CardDataFormat = 3,
    /// Reader LED control.
    LedControl = 4,
    /// Reader audible (buzzer) output.
    AudibleOutput = 5,
    /// Reader text output.
    TextOutput = 6,
    /// Time keeping (deprecated).
    TimeKeeping = 7,
    /// Check-character support: `compliance` 0 = checksum only, 1 = CRC.
    CheckCharacter = 8,
    /// Communication security: `compliance` bit 0 = AES-128.
    CommunicationSecurity = 9,
    /// Receive buffer size: 16-bit value with `compliance` = LSB,
    /// `number_of` = MSB.
    ReceiveBufferSize = 10,
    /// Largest combined message size: same little-endian split.
    LargestCombinedSize = 11,
    /// Smart-card support: `compliance` bit 0 = transparent, bit 1 = extended.
    SmartCardSupport = 12,
    /// Number of downstream readers.
    Readers = 13,
    /// Biometrics: `compliance` 1 = fp T1, 2 = fp T2, 3 = iris T1.
    Biometrics = 14,
    /// Secure PIN entry.
    SecurePinEntry = 15,
    /// OSDP version: 0 pre-IEC, 1 IEC 60839-11-5, 2 SIA OSDP 2.2.
    OsdpVersion = 16,
}

impl FunctionCode {
    /// Parse from byte.
    pub const fn from_byte(b: u8) -> Result<Self, Error> {
        Ok(match b {
            1 => Self::ContactStatus,
            2 => Self::OutputControl,
            3 => Self::CardDataFormat,
            4 => Self::LedControl,
            5 => Self::AudibleOutput,
            6 => Self::TextOutput,
            7 => Self::TimeKeeping,
            8 => Self::CheckCharacter,
            9 => Self::CommunicationSecurity,
            10 => Self::ReceiveBufferSize,
            11 => Self::LargestCombinedSize,
            12 => Self::SmartCardSupport,
            13 => Self::Readers,
            14 => Self::Biometrics,
            15 => Self::SecurePinEntry,
            16 => Self::OsdpVersion,
            other => {
                return Err(Error::MalformedPayload {
                    code: other,
                    reason: "unknown PDCAP function code",
                });
            }
        })
    }

    /// Raw byte value.
    pub const fn as_byte(self) -> u8 {
        self as u8
    }
}

/// One capability entry from `osdp_PDCAP`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Capability {
    /// Function code byte (may be unknown to us — keep raw).
    pub code: u8,
    /// Compliance / level. Meaning is per-function.
    pub compliance: u8,
    /// Quantity / index. Meaning is per-function.
    pub number_of: u8,
}

impl Capability {
    /// Wire size of one capability triplet.
    pub const WIRE_LEN: usize = 3;

    /// Encode to 3 bytes.
    pub const fn encode(self) -> [u8; 3] {
        [self.code, self.compliance, self.number_of]
    }

    /// Decode from 3 bytes.
    pub const fn decode(bytes: [u8; 3]) -> Self {
        Self {
            code: bytes[0],
            compliance: bytes[1],
            number_of: bytes[2],
        }
    }

    /// Recognized [`FunctionCode`], if any.
    pub fn function(self) -> Option<FunctionCode> {
        FunctionCode::from_byte(self.code).ok()
    }

    /// 16-bit value formed by `compliance | (number_of << 8)`.
    pub const fn u16_value(self) -> u16 {
        u16::from_le_bytes([self.compliance, self.number_of])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rx_size_combined() {
        let c = Capability {
            code: FunctionCode::ReceiveBufferSize.as_byte(),
            compliance: 0x80, // 128
            number_of: 0x00,
        };
        assert_eq!(c.u16_value(), 128);
    }

    #[test]
    fn function_roundtrip() {
        for raw in 1u8..=16 {
            let f = FunctionCode::from_byte(raw).unwrap();
            assert_eq!(f.as_byte(), raw);
        }
    }
}
