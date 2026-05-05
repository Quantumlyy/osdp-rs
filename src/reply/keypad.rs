//! `osdp_KEYPAD` (`0x53`) — keypad data.
//!
//! # Spec: §7.12

use crate::error::Error;
use crate::payload_util::require_at_least;
use alloc::vec::Vec;

/// `osdp_KEYPAD` body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Keypad {
    /// Reader number.
    pub reader: u8,
    /// Number of digits in `digits`.
    pub digit_count: u8,
    /// Digits (ASCII; `0x7F` = clear, `0x0D` = enter).
    pub digits: Vec<u8>,
}

impl Keypad {
    /// Encode.
    pub fn encode(&self) -> Result<Vec<u8>, Error> {
        if self.digits.len() != self.digit_count as usize {
            return Err(Error::MalformedPayload {
                code: 0x53,
                reason: "KEYPAD digit_count disagrees with digits",
            });
        }
        let mut out = Vec::with_capacity(2 + self.digits.len());
        out.push(self.reader);
        out.push(self.digit_count);
        out.extend_from_slice(&self.digits);
        Ok(out)
    }

    /// Decode.
    pub fn decode(data: &[u8]) -> Result<Self, Error> {
        require_at_least(data, 2, 0x53)?;
        let digit_count = data[1];
        if data.len() != 2 + digit_count as usize {
            return Err(Error::MalformedPayload {
                code: 0x53,
                reason: "KEYPAD digit count disagrees with payload",
            });
        }
        Ok(Self {
            reader: data[0],
            digit_count,
            digits: data[2..].to_vec(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let body = Keypad {
            reader: 0x00,
            digit_count: 3,
            digits: alloc::vec![b'1', b'2', b'3'],
        };
        let bytes = body.encode().unwrap();
        assert_eq!(bytes, [0x00, 3, b'1', b'2', b'3']);
        assert_eq!(Keypad::decode(&bytes).unwrap(), body);
    }

    #[test]
    fn encode_rejects_count_disagreement() {
        let body = Keypad {
            reader: 0,
            digit_count: 5, // claims 5
            digits: alloc::vec![b'1', b'2'], // but only carries 2
        };
        assert!(matches!(
            body.encode(),
            Err(Error::MalformedPayload { code: 0x53, .. })
        ));
    }

    #[test]
    fn decode_rejects_count_disagreement() {
        // count=4 but only 1 digit byte follows.
        assert!(matches!(
            Keypad::decode(&[0x00, 4, b'1']),
            Err(Error::MalformedPayload { code: 0x53, .. })
        ));
    }
}
