//! `osdp_OUT` (`0x68`) — output control.
//!
//! # Spec: §6.8, Table 14
//!
//! Body is a sequence of 4-byte records, one per output to drive:
//!
//! ```text
//! +----------+-----------+--------------+--------------+
//! | output#  | code      | timer LSB    | timer MSB    |
//! +----------+-----------+--------------+--------------+
//! ```

use crate::error::Error;
use alloc::vec::Vec;

/// Output control codes — Table 14.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
#[allow(missing_docs)]
pub enum OutputControlCode {
    Nop = 0x00,
    PermanentOffAbortTimed = 0x01,
    PermanentOnAbortTimed = 0x02,
    PermanentOffAllowTimed = 0x03,
    PermanentOnAllowTimed = 0x04,
    TemporaryOnResume = 0x05,
    TemporaryOffResume = 0x06,
}

impl OutputControlCode {
    /// Parse from raw byte.
    pub const fn from_byte(b: u8) -> Result<Self, Error> {
        Ok(match b {
            0x00 => Self::Nop,
            0x01 => Self::PermanentOffAbortTimed,
            0x02 => Self::PermanentOnAbortTimed,
            0x03 => Self::PermanentOffAllowTimed,
            0x04 => Self::PermanentOnAllowTimed,
            0x05 => Self::TemporaryOnResume,
            0x06 => Self::TemporaryOffResume,
            _ => return Err(Error::MalformedPayload {
                code: 0x68,
                reason: "unknown output control code",
            }),
        })
    }

    /// Raw byte value.
    pub const fn as_byte(self) -> u8 {
        self as u8
    }
}

/// Single output record.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OutputRecord {
    /// Output number on the PD.
    pub output: u8,
    /// Control code.
    pub code: OutputControlCode,
    /// Timer in 100 ms increments (16-bit LE).
    pub timer: u16,
}

impl OutputRecord {
    /// Wire size of one record.
    pub const WIRE_LEN: usize = 4;
}

/// `osdp_OUT` body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputControl {
    /// Records (one per output).
    pub records: Vec<OutputRecord>,
}

impl OutputControl {
    /// New, with the given records.
    pub fn new(records: Vec<OutputRecord>) -> Self {
        Self { records }
    }

    /// Encode.
    pub fn encode(&self) -> Result<Vec<u8>, Error> {
        if self.records.is_empty() {
            return Err(Error::MalformedPayload {
                code: 0x68,
                reason: "OUT requires at least one record",
            });
        }
        let mut out = Vec::with_capacity(self.records.len() * OutputRecord::WIRE_LEN);
        for r in &self.records {
            out.push(r.output);
            out.push(r.code.as_byte());
            out.extend_from_slice(&r.timer.to_le_bytes());
        }
        Ok(out)
    }

    /// Decode.
    pub fn decode(data: &[u8]) -> Result<Self, Error> {
        if data.is_empty() || data.len() % OutputRecord::WIRE_LEN != 0 {
            return Err(Error::MalformedPayload {
                code: 0x68,
                reason: "OUT payload must be a multiple of 4 bytes",
            });
        }
        let mut records = Vec::with_capacity(data.len() / OutputRecord::WIRE_LEN);
        for chunk in data.chunks_exact(OutputRecord::WIRE_LEN) {
            records.push(OutputRecord {
                output: chunk[0],
                code: OutputControlCode::from_byte(chunk[1])?,
                timer: u16::from_le_bytes([chunk[2], chunk[3]]),
            });
        }
        Ok(Self { records })
    }
}
