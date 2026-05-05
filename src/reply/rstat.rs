//! `osdp_RSTATR` (`0x4B`) — reader (tamper) status report.
//!
//! # Spec: §7.9
//!
//! One byte per reader (`0` = normal, `1` = tamper, `2` = disconnected).

use crate::error::Error;
use alloc::vec::Vec;

/// `osdp_RSTATR` body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RStatR {
    /// Status of each reader.
    pub readers: Vec<u8>,
}

impl RStatR {
    /// Encode.
    pub fn encode(&self) -> Result<Vec<u8>, Error> {
        Ok(self.readers.clone())
    }

    /// Decode.
    pub fn decode(data: &[u8]) -> Result<Self, Error> {
        Ok(Self {
            readers: data.to_vec(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let body = RStatR {
            readers: alloc::vec![0, 1, 2],
        };
        let bytes = body.encode().unwrap();
        assert_eq!(bytes, [0, 1, 2]);
        assert_eq!(RStatR::decode(&bytes).unwrap(), body);
    }

    #[test]
    fn empty_readers_is_valid() {
        assert_eq!(
            RStatR::decode(&[]).unwrap(),
            RStatR {
                readers: Vec::new()
            }
        );
    }
}
