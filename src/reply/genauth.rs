//! `osdp_GENAUTHR` (`0x81`) — generic authenticate response (PIV).
//!
//! # Spec: §7.21

use crate::error::Error;
use alloc::vec::Vec;

/// `osdp_GENAUTHR` body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenAuthR {
    /// Authentication response template (TLV).
    pub response: Vec<u8>,
}

impl GenAuthR {
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
        let body = GenAuthR {
            response: alloc::vec![0x7C, 0x02, 0x82, 0x00],
        };
        let bytes = body.encode().unwrap();
        assert_eq!(bytes, [0x7C, 0x02, 0x82, 0x00]);
        assert_eq!(GenAuthR::decode(&bytes).unwrap(), body);
    }

    #[test]
    fn empty_response_is_valid() {
        assert_eq!(
            GenAuthR::decode(&[]).unwrap(),
            GenAuthR {
                response: Vec::new()
            }
        );
    }
}
