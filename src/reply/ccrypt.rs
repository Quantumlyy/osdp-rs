//! `osdp_CCRYPT` (`0x76`) — client cryptogram (PD response to `osdp_CHLNG`).
//!
//! # Spec: §7.16, Annex D.4
//!
//! Body: `cUID (8) || RND.B (8) || ClientCryptogram (16)`.

use crate::error::Error;
use crate::payload_util::require_exact_len;
use alloc::vec::Vec;

/// `osdp_CCRYPT` body.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CCrypt {
    /// PD client identifier (8 bytes).
    pub cuid: [u8; 8],
    /// `RND.B` from the PD.
    pub rnd_b: [u8; 8],
    /// Client cryptogram = AES_S-ENC( RND.A || RND.B ).
    pub client_cryptogram: [u8; 16],
}

impl CCrypt {
    /// Encode.
    pub fn encode(&self) -> Result<Vec<u8>, Error> {
        let mut out = Vec::with_capacity(32);
        out.extend_from_slice(&self.cuid);
        out.extend_from_slice(&self.rnd_b);
        out.extend_from_slice(&self.client_cryptogram);
        Ok(out)
    }

    /// Decode.
    pub fn decode(data: &[u8]) -> Result<Self, Error> {
        require_exact_len(data, 32, 0x76)?;
        let mut cuid = [0u8; 8];
        cuid.copy_from_slice(&data[..8]);
        let mut rnd_b = [0u8; 8];
        rnd_b.copy_from_slice(&data[8..16]);
        let mut client_cryptogram = [0u8; 16];
        client_cryptogram.copy_from_slice(&data[16..32]);
        Ok(Self {
            cuid,
            rnd_b,
            client_cryptogram,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let body = CCrypt {
            cuid: [0xAA; 8],
            rnd_b: [0xBB; 8],
            client_cryptogram: [0xCC; 16],
        };
        let bytes = body.encode().unwrap();
        assert_eq!(bytes.len(), 32);
        assert_eq!(&bytes[..8], &[0xAA; 8]);
        assert_eq!(&bytes[8..16], &[0xBB; 8]);
        assert_eq!(&bytes[16..32], &[0xCC; 16]);
        assert_eq!(CCrypt::decode(&bytes).unwrap(), body);
    }

    #[test]
    fn decode_rejects_wrong_length() {
        assert!(matches!(
            CCrypt::decode(&[0; 31]),
            Err(Error::PayloadLength { code: 0x76, .. })
        ));
    }
}
