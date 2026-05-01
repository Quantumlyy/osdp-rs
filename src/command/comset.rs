//! `osdp_COMSET` (`0x6E`) — set PD address and baud rate.
//!
//! # Spec: §6.13
//!
//! Body is 5 bytes: `addr (1)` + `baud (4 LE)`.

use crate::error::Error;
use alloc::vec::Vec;

/// `osdp_COMSET` body.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ComSet {
    /// New PD address (`0x00..=0x7E`).
    pub address: u8,
    /// New bit rate (e.g. 9600, 115200).
    pub baud: u32,
}

impl ComSet {
    /// Encode.
    pub fn encode(&self) -> Result<Vec<u8>, Error> {
        if self.address > crate::MAX_PD_ADDR {
            return Err(Error::BadAddress(self.address));
        }
        let mut out = Vec::with_capacity(5);
        out.push(self.address);
        out.extend_from_slice(&self.baud.to_le_bytes());
        Ok(out)
    }

    /// Decode.
    pub fn decode(data: &[u8]) -> Result<Self, Error> {
        if data.len() != 5 {
            return Err(Error::MalformedPayload {
                code: 0x6E,
                reason: "COMSET requires 5 bytes",
            });
        }
        Ok(Self {
            address: data[0],
            baud: u32::from_le_bytes([data[1], data[2], data[3], data[4]]),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    /// Annex E vector: COMSET broadcast → `addr=0x00`, `baud=9600 (0x2580)`.
    #[test]
    fn annex_e_vector() {
        let body = ComSet {
            address: 0x00,
            baud: 9600,
        };
        let bytes = body.encode().unwrap();
        assert_eq!(bytes, [0x00, 0x80, 0x25, 0x00, 0x00]);
    }
}
