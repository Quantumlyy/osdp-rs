//! `osdp_OSTATR` (`0x4A`) — output status report.
//!
//! # Spec: §7.8
//!
//! One byte per output (`0` = OFF, `1` = ON).

use crate::error::Error;
use alloc::vec::Vec;

/// `osdp_OSTATR` body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OStatR {
    /// Status of each output.
    pub outputs: Vec<u8>,
}

impl OStatR {
    /// Encode.
    pub fn encode(&self) -> Result<Vec<u8>, Error> {
        Ok(self.outputs.clone())
    }

    /// Decode.
    pub fn decode(data: &[u8]) -> Result<Self, Error> {
        Ok(Self {
            outputs: data.to_vec(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let body = OStatR {
            outputs: alloc::vec![1, 0, 1, 0],
        };
        let bytes = body.encode().unwrap();
        assert_eq!(bytes, [1, 0, 1, 0]);
        assert_eq!(OStatR::decode(&bytes).unwrap(), body);
    }

    #[test]
    fn empty_outputs_is_valid() {
        assert_eq!(
            OStatR::decode(&[]).unwrap(),
            OStatR {
                outputs: Vec::new()
            }
        );
    }
}
