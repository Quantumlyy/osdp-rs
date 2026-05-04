//! `osdp_LED` (`0x69`) — reader LED control.
//!
//! # Spec: §6.9, Tables 16–18
//!
//! Body is a sequence of 14-byte records (Reader#, LED#, then a 6-byte
//! "temporary" control, then a 6-byte "permanent" control).

use crate::error::Error;
use alloc::vec::Vec;

/// LED color values — Table 18.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
#[allow(missing_docs)]
pub enum LedColor {
    Black = 0,
    Red = 1,
    Green = 2,
    Amber = 3,
    Blue = 4,
    Magenta = 5,
    Cyan = 6,
    White = 7,
}

impl LedColor {
    /// Parse from byte (preserving "unknown" as Black per spec leniency).
    pub const fn from_byte(b: u8) -> Self {
        match b {
            0 => Self::Black,
            1 => Self::Red,
            2 => Self::Green,
            3 => Self::Amber,
            4 => Self::Blue,
            5 => Self::Magenta,
            6 => Self::Cyan,
            7 => Self::White,
            _ => Self::Black,
        }
    }

    /// Raw byte value.
    pub const fn as_byte(self) -> u8 {
        self as u8
    }
}

/// Temporary LED control sub-block (6 bytes).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LedTemporary {
    /// Control code (Table 16): 0=NOP, 1=cancel temp, 2=set temp.
    pub control: u8,
    /// On time (units of 100 ms).
    pub on_time: u8,
    /// Off time (units of 100 ms).
    pub off_time: u8,
    /// Color while on.
    pub on_color: LedColor,
    /// Color while off.
    pub off_color: LedColor,
    /// Total run time, in units of 100 ms (16-bit LE).
    pub timer: u16,
}

/// Permanent LED control sub-block (6 bytes).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LedPermanent {
    /// Control code (Table 17): 0=NOP, 1=set permanent state.
    pub control: u8,
    /// On time.
    pub on_time: u8,
    /// Off time.
    pub off_time: u8,
    /// Color while on.
    pub on_color: LedColor,
    /// Color while off.
    pub off_color: LedColor,
}

/// One LED record (14 bytes).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LedRecord {
    /// Reader number on the PD.
    pub reader: u8,
    /// LED number on the reader.
    pub led: u8,
    /// Temporary control half.
    pub temporary: LedTemporary,
    /// Permanent control half.
    pub permanent: LedPermanent,
}

impl LedRecord {
    /// Wire size of one record.
    pub const WIRE_LEN: usize = 14;

    fn encode_into(&self, out: &mut Vec<u8>) {
        out.push(self.reader);
        out.push(self.led);

        out.push(self.temporary.control);
        out.push(self.temporary.on_time);
        out.push(self.temporary.off_time);
        out.push(self.temporary.on_color.as_byte());
        out.push(self.temporary.off_color.as_byte());
        out.extend_from_slice(&self.temporary.timer.to_le_bytes());

        out.push(self.permanent.control);
        out.push(self.permanent.on_time);
        out.push(self.permanent.off_time);
        out.push(self.permanent.on_color.as_byte());
        out.push(self.permanent.off_color.as_byte());
    }

    fn decode(buf: &[u8]) -> Self {
        Self {
            reader: buf[0],
            led: buf[1],
            temporary: LedTemporary {
                control: buf[2],
                on_time: buf[3],
                off_time: buf[4],
                on_color: LedColor::from_byte(buf[5]),
                off_color: LedColor::from_byte(buf[6]),
                timer: u16::from_le_bytes([buf[7], buf[8]]),
            },
            permanent: LedPermanent {
                control: buf[9],
                on_time: buf[10],
                off_time: buf[11],
                on_color: LedColor::from_byte(buf[12]),
                off_color: LedColor::from_byte(buf[13]),
            },
        }
    }
}

/// `osdp_LED` body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LedControl {
    /// Records (one per LED).
    pub records: Vec<LedRecord>,
}

impl LedControl {
    /// New body.
    pub fn new(records: Vec<LedRecord>) -> Self {
        Self { records }
    }

    /// Encode.
    pub fn encode(&self) -> Result<Vec<u8>, Error> {
        if self.records.is_empty() {
            return Err(Error::MalformedPayload {
                code: 0x69,
                reason: "LED requires at least one record",
            });
        }
        let mut out = Vec::with_capacity(self.records.len() * LedRecord::WIRE_LEN);
        for r in &self.records {
            r.encode_into(&mut out);
        }
        Ok(out)
    }

    /// Decode.
    pub fn decode(data: &[u8]) -> Result<Self, Error> {
        if data.is_empty() || data.len() % LedRecord::WIRE_LEN != 0 {
            return Err(Error::MalformedPayload {
                code: 0x69,
                reason: "LED payload must be a multiple of 14 bytes",
            });
        }
        let records = data
            .chunks_exact(LedRecord::WIRE_LEN)
            .map(LedRecord::decode)
            .collect();
        Ok(Self { records })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let body = LedControl::new(alloc::vec![LedRecord {
            reader: 0,
            led: 1,
            temporary: LedTemporary {
                control: 2,
                on_time: 5,
                off_time: 5,
                on_color: LedColor::Green,
                off_color: LedColor::Black,
                timer: 50,
            },
            permanent: LedPermanent {
                control: 1,
                on_time: 0xFF,
                off_time: 0,
                on_color: LedColor::Red,
                off_color: LedColor::Black,
            },
        }]);
        let bytes = body.encode().unwrap();
        assert_eq!(bytes.len(), 14);
        let decoded = LedControl::decode(&bytes).unwrap();
        assert_eq!(decoded, body);
    }
}
