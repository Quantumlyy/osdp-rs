//! `osdp_TEXT` (`0x6B`) — reader text output.
//!
//! # Spec: §6.12, Tables 21–22
//!
//! Body layout:
//!
//! ```text
//! +--------+----------+--------+--------+--------+--------+-- ... --+
//! | reader | command  | temp_t | row    | column | length | text…   |
//! +--------+----------+--------+--------+--------+--------+-- ... --+
//! ```

use crate::error::Error;
use alloc::vec::Vec;

/// Text command codes (Table 21).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
#[allow(missing_docs)]
pub enum TextCommand {
    PermanentNoWrap = 0x01,
    PermanentWrap = 0x02,
    TemporaryNoWrap = 0x03,
    TemporaryWrap = 0x04,
}

impl TextCommand {
    /// Parse from byte.
    pub const fn from_byte(b: u8) -> Result<Self, Error> {
        Ok(match b {
            0x01 => Self::PermanentNoWrap,
            0x02 => Self::PermanentWrap,
            0x03 => Self::TemporaryNoWrap,
            0x04 => Self::TemporaryWrap,
            _ => return Err(Error::MalformedPayload {
                code: 0x6B,
                reason: "unknown TEXT command code",
            }),
        })
    }

    /// Raw byte.
    pub const fn as_byte(self) -> u8 {
        self as u8
    }
}

/// `osdp_TEXT` body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Text {
    /// Reader number.
    pub reader: u8,
    /// Text command code.
    pub command: TextCommand,
    /// Temporary on-time in seconds (1..=255). Set 0 for permanent commands.
    pub temp_time_s: u8,
    /// Display row (1..).
    pub row: u8,
    /// Display column (1..).
    pub column: u8,
    /// Printable ASCII text (`0x20..=0x7E`).
    pub text: Vec<u8>,
}

impl Text {
    /// Spec-compliant ASCII bound.
    fn validate_ascii(s: &[u8]) -> Result<(), Error> {
        for &b in s {
            if !(0x20..=0x7E).contains(&b) {
                return Err(Error::MalformedPayload {
                    code: 0x6B,
                    reason: "TEXT must be printable ASCII",
                });
            }
        }
        Ok(())
    }

    /// Encode.
    pub fn encode(&self) -> Result<Vec<u8>, Error> {
        Self::validate_ascii(&self.text)?;
        if self.text.len() > u8::MAX as usize {
            return Err(Error::MalformedPayload {
                code: 0x6B,
                reason: "TEXT length exceeds 255 bytes",
            });
        }
        let mut out = Vec::with_capacity(6 + self.text.len());
        out.push(self.reader);
        out.push(self.command.as_byte());
        out.push(self.temp_time_s);
        out.push(self.row);
        out.push(self.column);
        out.push(self.text.len() as u8);
        out.extend_from_slice(&self.text);
        Ok(out)
    }

    /// Decode.
    pub fn decode(data: &[u8]) -> Result<Self, Error> {
        if data.len() < 6 {
            return Err(Error::MalformedPayload {
                code: 0x6B,
                reason: "TEXT requires at least 6 bytes",
            });
        }
        let length = data[5] as usize;
        if data.len() != 6 + length {
            return Err(Error::MalformedPayload {
                code: 0x6B,
                reason: "TEXT length field disagrees with payload",
            });
        }
        let text = data[6..6 + length].to_vec();
        Self::validate_ascii(&text)?;
        Ok(Self {
            reader: data[0],
            command: TextCommand::from_byte(data[1])?,
            temp_time_s: data[2],
            row: data[3],
            column: data[4],
            text,
        })
    }
}
