//! AES-128-CBC encryption / decryption of `SCS_17`/`SCS_18` DATA payloads.
//!
//! # Spec: Annex D.5
//!
//! - DATA is **always** 0x80-padded (per [`super::pad::pad_data`]).
//! - The CBC ICV is the **one's complement** of the last MAC received from
//!   the *other* device.
//! - Key is `S-ENC`.

use crate::error::SecureSessionError;
use crate::secure::pad::{pad_data, unpad_data};
use aes::cipher::generic_array::GenericArray;
use aes::cipher::{BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use alloc::vec::Vec;

type Encryptor = cbc::Encryptor<aes::Aes128>;
type Decryptor = cbc::Decryptor<aes::Aes128>;

/// Annex D.5 ICV: one's complement of the last MAC from the other side.
#[inline]
pub fn complement_icv(other_mac: &[u8; 16]) -> [u8; 16] {
    let mut iv = [0u8; 16];
    for (d, s) in iv.iter_mut().zip(other_mac.iter()) {
        *d = !*s;
    }
    iv
}

/// Encrypt a DATA payload with AES-128-CBC, applying the always-0x80 padding
/// rule. `s_enc` is the session encryption key, `iv` is the complemented MAC
/// of the last frame received from the other side.
///
/// Output length is `((data.len() + 1).div_ceil(16)) * 16`.
pub fn encrypt_data(s_enc: &[u8; 16], iv: &[u8; 16], data: &[u8]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(data.len() + 16);
    buf.extend_from_slice(data);
    pad_data(&mut buf);
    debug_assert_eq!(buf.len() % 16, 0);

    let mut enc = Encryptor::new(GenericArray::from_slice(s_enc), GenericArray::from_slice(iv));
    for chunk in buf.chunks_exact_mut(16) {
        let mut block = *GenericArray::from_slice(chunk);
        enc.encrypt_block_mut(&mut block);
        chunk.copy_from_slice(&block);
    }
    buf
}

/// Decrypt a DATA payload and strip the 0x80 padding.
///
/// Errors if `ct.len()` is not a positive multiple of 16, or if padding is
/// malformed after decryption.
pub fn decrypt_data(
    s_enc: &[u8; 16],
    iv: &[u8; 16],
    ct: &[u8],
) -> Result<Vec<u8>, SecureSessionError> {
    if ct.is_empty() || ct.len() % 16 != 0 {
        return Err(SecureSessionError::BadPadding);
    }
    let mut buf = ct.to_vec();
    let mut dec = Decryptor::new(GenericArray::from_slice(s_enc), GenericArray::from_slice(iv));
    for chunk in buf.chunks_exact_mut(16) {
        let mut block = *GenericArray::from_slice(chunk);
        dec.decrypt_block_mut(&mut block);
        chunk.copy_from_slice(&block);
    }
    let stripped = unpad_data(&buf)?;
    Ok(stripped.to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_short() {
        let s_enc = [0x42u8; 16];
        let iv = [0xAAu8; 16];
        for n in 0..40usize {
            let plain: Vec<u8> = (0u8..(n as u8)).collect();
            let ct = encrypt_data(&s_enc, &iv, &plain);
            assert_eq!(ct.len() % 16, 0);
            assert!(ct.len() > plain.len());
            let pt = decrypt_data(&s_enc, &iv, &ct).unwrap();
            assert_eq!(pt, plain);
        }
    }

    #[test]
    fn complement_icv_is_xor_ff() {
        let mac = [0xA5u8; 16];
        let iv = complement_icv(&mac);
        assert!(iv.iter().all(|&b| b == 0x5A));
    }

    #[test]
    fn decrypt_rejects_unaligned() {
        let key = [0u8; 16];
        let iv = [0u8; 16];
        assert!(decrypt_data(&key, &iv, &[0u8; 15]).is_err());
        assert!(decrypt_data(&key, &iv, &[]).is_err());
    }

    /// Wrong IV → padding will not validate → error.
    #[test]
    fn wrong_iv_fails_padding() {
        let s_enc = [0x42u8; 16];
        let iv = [0xAAu8; 16];
        let ct = encrypt_data(&s_enc, &iv, &[1, 2, 3, 4]);
        let bad_iv = [0xFFu8; 16];
        assert!(decrypt_data(&s_enc, &bad_iv, &ct).is_err());
    }
}
