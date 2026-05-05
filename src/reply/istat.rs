//! `osdp_ISTATR` (`0x49`) — input status report.
//!
//! # Spec: §7.7
//!
//! One byte per input, where each byte is `0` (inactive) or `1` (active).

use crate::error::Error;
use alloc::vec::Vec;

/// `osdp_ISTATR` body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IStatR {
    /// Status of each input, ordered by input number.
    pub inputs: Vec<u8>,
}

impl IStatR {
    /// Encode.
    pub fn encode(&self) -> Result<Vec<u8>, Error> {
        Ok(self.inputs.clone())
    }

    /// Decode.
    pub fn decode(data: &[u8]) -> Result<Self, Error> {
        Ok(Self {
            inputs: data.to_vec(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let body = IStatR {
            inputs: alloc::vec![0, 1, 0, 1],
        };
        let bytes = body.encode().unwrap();
        assert_eq!(bytes, [0, 1, 0, 1]);
        assert_eq!(IStatR::decode(&bytes).unwrap(), body);
    }

    #[test]
    fn empty_inputs_is_valid() {
        assert_eq!(IStatR::decode(&[]).unwrap(), IStatR { inputs: Vec::new() });
    }
}
