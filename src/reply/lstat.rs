//! `osdp_LSTATR` (`0x48`) — local status report.
//!
//! # Spec: §7.6
//!
//! Body is exactly 2 bytes: `tamper`, `power`. `0` means normal, `1` means a
//! fault.

use crate::error::Error;
use alloc::vec::Vec;

/// `osdp_LSTATR` body.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LStatR {
    /// Tamper switch state. `0` = normal, `1` = tamper.
    pub tamper: u8,
    /// Power state. `0` = normal, `1` = power failure.
    pub power: u8,
}

impl LStatR {
    /// Encode.
    pub fn encode(&self) -> Result<Vec<u8>, Error> {
        Ok(alloc::vec![self.tamper, self.power])
    }

    /// Decode.
    pub fn decode(data: &[u8]) -> Result<Self, Error> {
        if data.len() != 2 {
            return Err(Error::MalformedPayload {
                code: 0x48,
                reason: "LSTATR requires 2 bytes",
            });
        }
        Ok(Self {
            tamper: data[0],
            power: data[1],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let body = LStatR {
            tamper: 0,
            power: 1,
        };
        let bytes = body.encode().unwrap();
        assert_eq!(bytes, [0, 1]);
        assert_eq!(LStatR::decode(&bytes).unwrap(), body);
    }

    #[test]
    fn decode_rejects_wrong_length() {
        assert!(matches!(
            LStatR::decode(&[0]),
            Err(Error::MalformedPayload { code: 0x48, .. })
        ));
    }
}
