//! `osdp_ACURXSIZE` (`0x7B`) — inform the PD of the ACU's max receive size.
//!
//! # Spec: §6.19
//!
//! Body is a 16-bit little-endian byte count.

use crate::error::Error;
use crate::payload_util::require_exact_len;
use alloc::vec::Vec;

/// `osdp_ACURXSIZE` body.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AcuRxSize {
    /// Bytes the ACU can receive in one packet.
    pub max_size: u16,
}

impl AcuRxSize {
    /// Encode.
    pub fn encode(&self) -> Result<Vec<u8>, Error> {
        Ok(self.max_size.to_le_bytes().to_vec())
    }

    /// Decode.
    pub fn decode(data: &[u8]) -> Result<Self, Error> {
        require_exact_len(data, 2, 0x7B)?;
        Ok(Self {
            max_size: u16::from_le_bytes([data[0], data[1]]),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let body = AcuRxSize { max_size: 0x0123 };
        let bytes = body.encode().unwrap();
        assert_eq!(bytes, [0x23, 0x01]);
        assert_eq!(AcuRxSize::decode(&bytes).unwrap(), body);
    }

    #[test]
    fn decode_rejects_short() {
        assert!(matches!(
            AcuRxSize::decode(&[0x10]),
            Err(Error::PayloadLength { code: 0x7B, .. })
        ));
    }
}
