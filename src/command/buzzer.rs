//! `osdp_BUZ` (`0x6A`) — reader buzzer control.
//!
//! # Spec: §6.10–§6.11, Table 19
//!
//! Body is a 5-byte record per buzzer.

use crate::error::Error;
use alloc::vec::Vec;

/// Buzzer tone code (Table 19).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
#[allow(missing_docs)]
pub enum BuzzerTone {
    /// Deprecated; treated as Off by modern firmware.
    None = 0x00,
    Off = 0x01,
    Default = 0x02,
}

impl BuzzerTone {
    /// Parse from byte (returns Off for unrecognized).
    pub const fn from_byte(b: u8) -> Self {
        match b {
            0x00 => Self::None,
            0x01 => Self::Off,
            _ => Self::Default,
        }
    }

    /// Raw byte.
    pub const fn as_byte(self) -> u8 {
        self as u8
    }
}

/// `osdp_BUZ` body.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BuzzerControl {
    /// Reader number.
    pub reader: u8,
    /// Tone code.
    pub tone: BuzzerTone,
    /// On time, in 100 ms units.
    pub on_time: u8,
    /// Off time, in 100 ms units.
    pub off_time: u8,
    /// Number of repetitions.
    pub count: u8,
}

impl BuzzerControl {
    /// Encode.
    pub fn encode(&self) -> Result<Vec<u8>, Error> {
        Ok(alloc::vec![
            self.reader,
            self.tone.as_byte(),
            self.on_time,
            self.off_time,
            self.count,
        ])
    }

    /// Decode.
    pub fn decode(data: &[u8]) -> Result<Self, Error> {
        if data.len() != 5 {
            return Err(Error::MalformedPayload {
                code: 0x6A,
                reason: "BUZ requires 5 bytes",
            });
        }
        Ok(Self {
            reader: data[0],
            tone: BuzzerTone::from_byte(data[1]),
            on_time: data[2],
            off_time: data[3],
            count: data[4],
        })
    }
}
