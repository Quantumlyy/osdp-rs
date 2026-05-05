//! `osdp_RAW` (`0x50`) and `osdp_FMT` (`0x51`) — card data reports.
//!
//! # Spec: §7.10, §7.11
//!
//! `RAW` carries a raw bit array: `reader (1)` + `format_code (1)` +
//! `bit_count (2 LE)` + `bytes`. `bytes` is `ceil(bit_count / 8)` bytes;
//! trailing bits in the final byte beyond `bit_count` are reserved and must
//! be ignored.

use crate::error::Error;
use crate::payload_util::require_at_least;
use alloc::vec::Vec;

/// `osdp_RAW` body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Raw {
    /// Reader number.
    pub reader: u8,
    /// Format code (vendor-specific; see Annex F).
    pub format_code: u8,
    /// Number of bits actually present in `bits`.
    pub bit_count: u16,
    /// `ceil(bit_count / 8)` bytes; high-order bits first.
    pub bits: Vec<u8>,
}

impl Raw {
    /// Encode.
    pub fn encode(&self) -> Result<Vec<u8>, Error> {
        let need = (self.bit_count as usize).div_ceil(8);
        if self.bits.len() != need {
            return Err(Error::MalformedPayload {
                code: 0x50,
                reason: "RAW bit count does not match byte length",
            });
        }
        let mut out = Vec::with_capacity(4 + self.bits.len());
        out.push(self.reader);
        out.push(self.format_code);
        out.extend_from_slice(&self.bit_count.to_le_bytes());
        out.extend_from_slice(&self.bits);
        Ok(out)
    }

    /// Decode.
    pub fn decode(data: &[u8]) -> Result<Self, Error> {
        require_at_least(data, 4, 0x50)?;
        let bit_count = u16::from_le_bytes([data[2], data[3]]);
        let need = (bit_count as usize).div_ceil(8);
        if data.len() != 4 + need {
            return Err(Error::MalformedPayload {
                code: 0x50,
                reason: "RAW bit count does not match payload length",
            });
        }
        Ok(Self {
            reader: data[0],
            format_code: data[1],
            bit_count,
            bits: data[4..].to_vec(),
        })
    }

    /// Iterate over the active card bits, MSB-first within each byte,
    /// stopping at `bit_count`.
    pub fn iter_bits(&self) -> impl Iterator<Item = bool> + '_ {
        let n = self.bit_count as usize;
        self.bits
            .iter()
            .enumerate()
            .flat_map(move |(byte_idx, &b)| {
                (0..8).filter_map(move |bit| {
                    let pos = byte_idx * 8 + bit;
                    if pos >= n {
                        None
                    } else {
                        Some(b & (0x80 >> bit) != 0)
                    }
                })
            })
    }
}

/// `osdp_FMT` body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fmt {
    /// Reader number.
    pub reader: u8,
    /// Read direction (0 = forward, 1 = reverse).
    pub direction: u8,
    /// Length of `chars`.
    pub char_count: u8,
    /// ASCII chars from the formatted card data.
    pub chars: Vec<u8>,
}

impl Fmt {
    /// Encode.
    pub fn encode(&self) -> Result<Vec<u8>, Error> {
        if self.chars.len() != self.char_count as usize {
            return Err(Error::MalformedPayload {
                code: 0x51,
                reason: "FMT char_count disagrees with chars",
            });
        }
        let mut out = Vec::with_capacity(3 + self.chars.len());
        out.push(self.reader);
        out.push(self.direction);
        out.push(self.char_count);
        out.extend_from_slice(&self.chars);
        Ok(out)
    }

    /// Decode.
    pub fn decode(data: &[u8]) -> Result<Self, Error> {
        require_at_least(data, 3, 0x51)?;
        let char_count = data[2];
        if data.len() != 3 + char_count as usize {
            return Err(Error::MalformedPayload {
                code: 0x51,
                reason: "FMT char_count disagrees with payload",
            });
        }
        Ok(Self {
            reader: data[0],
            direction: data[1],
            char_count,
            chars: data[3..].to_vec(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_iter_bits_truncates() {
        // 26-bit Wiegand card data
        let raw = Raw {
            reader: 0,
            format_code: 0,
            bit_count: 26,
            bits: alloc::vec![0xAA, 0xBB, 0xCC, 0x80],
        };
        let bits: alloc::vec::Vec<bool> = raw.iter_bits().collect();
        assert_eq!(bits.len(), 26);
    }

    #[test]
    fn raw_roundtrip() {
        let r = Raw {
            reader: 1,
            format_code: 0,
            bit_count: 26,
            bits: alloc::vec![0xAA, 0xBB, 0xCC, 0x80],
        };
        let bytes = r.encode().unwrap();
        let parsed = Raw::decode(&bytes).unwrap();
        assert_eq!(parsed, r);
    }
}
