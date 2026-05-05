//! `osdp_CRAUTHR` (`0x82`) — challenge response.
//!
//! # Spec: §7.22

use crate::error::Error;
use alloc::vec::Vec;

/// `osdp_CRAUTHR` body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrAuthR {
    /// Challenge response payload.
    pub response: Vec<u8>,
}

impl CrAuthR {
    /// Encode.
    pub fn encode(&self) -> Result<Vec<u8>, Error> {
        Ok(self.response.clone())
    }

    /// Decode.
    pub fn decode(data: &[u8]) -> Result<Self, Error> {
        Ok(Self {
            response: data.to_vec(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let body = CrAuthR {
            response: alloc::vec![0xDE, 0xAD, 0xBE, 0xEF],
        };
        let bytes = body.encode().unwrap();
        assert_eq!(bytes, [0xDE, 0xAD, 0xBE, 0xEF]);
        assert_eq!(CrAuthR::decode(&bytes).unwrap(), body);
    }

    #[test]
    fn empty_response_is_valid() {
        assert_eq!(
            CrAuthR::decode(&[]).unwrap(),
            CrAuthR {
                response: Vec::new()
            }
        );
    }
}
