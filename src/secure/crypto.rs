//! AES-128 primitives, key derivation, cryptograms.
//!
//! # Spec: Annex D.4
//!
//! Session keys are derived by AES-128 encrypting fixed templates with the
//! SCBK:
//!
//! - `S-ENC  = AES_SCBK([0x01, 0x82, RND.A[0..6], 0,0,0,0,0,0,0,0])`
//! - `S-MAC1 = AES_SCBK([0x01, 0x01, RND.A[0..6], 0,0,0,0,0,0,0,0])`
//! - `S-MAC2 = AES_SCBK([0x01, 0x02, RND.A[0..6], 0,0,0,0,0,0,0,0])`
//!
//! Cryptograms:
//!
//! - `ClientCryptogram = AES_S-ENC(RND.A || RND.B)`
//! - `ServerCryptogram = AES_S-ENC(RND.B || RND.A)`

use aes::Aes128;
use aes::cipher::generic_array::GenericArray;
use aes::cipher::{BlockDecrypt, BlockEncrypt, KeyInit};

/// 128-bit AES block.
pub type Block = [u8; 16];

/// AES-128 encrypt of a single block.
#[inline]
pub fn aes128_encrypt(key: &[u8; 16], block: &Block) -> Block {
    let cipher = Aes128::new(GenericArray::from_slice(key));
    let mut buf = *GenericArray::from_slice(block);
    cipher.encrypt_block(&mut buf);
    let mut out = [0u8; 16];
    out.copy_from_slice(&buf);
    out
}

/// AES-128 decrypt of a single block.
#[inline]
pub fn aes128_decrypt(key: &[u8; 16], block: &Block) -> Block {
    let cipher = Aes128::new(GenericArray::from_slice(key));
    let mut buf = *GenericArray::from_slice(block);
    cipher.decrypt_block(&mut buf);
    let mut out = [0u8; 16];
    out.copy_from_slice(&buf);
    out
}

/// Build the 16-byte template: `[tag1, tag2, RND.A[0..6], 0,0,0,0,0,0,0,0]`.
fn key_template(tag1: u8, tag2: u8, rnd_a: &[u8; 8]) -> Block {
    let mut t = [0u8; 16];
    t[0] = tag1;
    t[1] = tag2;
    t[2..8].copy_from_slice(&rnd_a[0..6]);
    t
}

/// Derived session keys.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SessionKeys {
    /// `S-ENC` — encryption key.
    pub s_enc: [u8; 16],
    /// `S-MAC1` — MAC key for all blocks except the last.
    pub s_mac1: [u8; 16],
    /// `S-MAC2` — MAC key for the last block.
    pub s_mac2: [u8; 16],
}

impl SessionKeys {
    /// Derive `S-ENC`/`S-MAC1`/`S-MAC2` from `scbk` and `RND.A`.
    pub fn derive(scbk: &[u8; 16], rnd_a: &[u8; 8]) -> Self {
        Self {
            s_enc: aes128_encrypt(scbk, &key_template(0x01, 0x82, rnd_a)),
            s_mac1: aes128_encrypt(scbk, &key_template(0x01, 0x01, rnd_a)),
            s_mac2: aes128_encrypt(scbk, &key_template(0x01, 0x02, rnd_a)),
        }
    }
}

/// `ClientCryptogram = AES_S-ENC(RND.A || RND.B)`.
pub fn client_cryptogram(s_enc: &[u8; 16], rnd_a: &[u8; 8], rnd_b: &[u8; 8]) -> Block {
    let mut block = [0u8; 16];
    block[..8].copy_from_slice(rnd_a);
    block[8..].copy_from_slice(rnd_b);
    aes128_encrypt(s_enc, &block)
}

/// `ServerCryptogram = AES_S-ENC(RND.B || RND.A)`.
pub fn server_cryptogram(s_enc: &[u8; 16], rnd_a: &[u8; 8], rnd_b: &[u8; 8]) -> Block {
    let mut block = [0u8; 16];
    block[..8].copy_from_slice(rnd_b);
    block[8..].copy_from_slice(rnd_a);
    aes128_encrypt(s_enc, &block)
}

/// `Initial R-MAC = AES_S-MAC2( AES_S-MAC1( ServerCryptogram ) )`.
///
/// # Spec: Annex D.4
pub fn initial_rmac(s_mac1: &[u8; 16], s_mac2: &[u8; 16], server_cryptogram: &Block) -> Block {
    let first = aes128_encrypt(s_mac1, server_cryptogram);
    aes128_encrypt(s_mac2, &first)
}

/// Legacy key diversification: `SCBK = AES_MK( cUID || ~cUID )`.
///
/// # Spec: Annex D.7 (deprecated)
pub fn diversify_scbk_legacy(mk: &[u8; 16], cuid: &[u8; 8]) -> [u8; 16] {
    let mut block = [0u8; 16];
    block[..8].copy_from_slice(cuid);
    for (i, &b) in cuid.iter().enumerate() {
        block[8 + i] = !b;
    }
    aes128_encrypt(mk, &block)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// AES-128 inversion.
    #[test]
    fn enc_then_dec_identity() {
        let key = [0u8; 16];
        let plain = [0xABu8; 16];
        let ct = aes128_encrypt(&key, &plain);
        let pt = aes128_decrypt(&key, &ct);
        assert_eq!(pt, plain);
    }

    /// FIPS-197 known answer — AES-128 with all-zero key + plaintext.
    #[test]
    fn fips_known_answer() {
        let key = [0u8; 16];
        let plain = [0u8; 16];
        let ct = aes128_encrypt(&key, &plain);
        let expected = [
            0x66, 0xE9, 0x4B, 0xD4, 0xEF, 0x8A, 0x2C, 0x3B, 0x88, 0x4C, 0xFA, 0x59, 0xCA, 0x34,
            0x2B, 0x2E,
        ];
        assert_eq!(ct, expected);
    }

    #[test]
    fn keys_differ() {
        let scbk = [0xAA; 16];
        let rnd_a = [0x55u8; 8];
        let k = SessionKeys::derive(&scbk, &rnd_a);
        assert_ne!(k.s_enc, k.s_mac1);
        assert_ne!(k.s_mac1, k.s_mac2);
        assert_ne!(k.s_enc, k.s_mac2);
    }

    #[test]
    fn cryptograms_distinct() {
        let s_enc = [0; 16];
        let rnd_a = [1u8; 8];
        let rnd_b = [2u8; 8];
        let c = client_cryptogram(&s_enc, &rnd_a, &rnd_b);
        let s = server_cryptogram(&s_enc, &rnd_a, &rnd_b);
        assert_ne!(c, s);
    }
}
