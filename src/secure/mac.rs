//! CBC-MAC with rolling ICV and the S-MAC1/S-MAC2 swap on the final block.
//!
//! # Spec: Annex D.5
//!
//! ```text
//! tmp = ICV
//! for each 16-byte block of padded input except the last:
//!     tmp = AES_S-MAC1(tmp XOR block)
//! tmp = AES_S-MAC2(tmp XOR last_block)
//! MAC = tmp        // first 4 bytes are sent on the wire
//! ```

use crate::secure::crypto::aes128_encrypt;
use crate::secure::pad::pad_mac;
use alloc::vec::Vec;

/// XOR `dst` with `src` in place.
#[inline]
fn xor_in_place(dst: &mut [u8; 16], src: &[u8]) {
    for (d, s) in dst.iter_mut().zip(src.iter()) {
        *d ^= *s;
    }
}

/// Compute the OSDP CBC-MAC.
///
/// `icv` is the previous MAC from the *other* direction (R-MAC ↔ C-MAC roll
/// per Annex D). For the very first MAC after handshake, the ICV is the
/// initial R-MAC computed by [`super::crypto::initial_rmac`].
///
/// Returns the full 16-byte MAC; only the first 4 are transmitted.
pub fn cbc_mac(input: &[u8], icv: &[u8; 16], s_mac1: &[u8; 16], s_mac2: &[u8; 16]) -> [u8; 16] {
    let mut padded = Vec::with_capacity(input.len() + 16);
    padded.extend_from_slice(input);
    pad_mac(&mut padded);
    debug_assert_eq!(padded.len() % 16, 0);
    debug_assert!(!padded.is_empty());

    let mut state = *icv;
    let blocks = padded.len() / 16;
    for (i, chunk) in padded.chunks_exact(16).enumerate() {
        xor_in_place(&mut state, chunk);
        let key = if i + 1 == blocks { s_mac2 } else { s_mac1 };
        state = aes128_encrypt(key, &state);
    }
    state
}

#[cfg(test)]
mod tests {
    use super::*;

    /// MAC is deterministic.
    #[test]
    fn deterministic() {
        let icv = [0u8; 16];
        let s1 = [1u8; 16];
        let s2 = [2u8; 16];
        let m1 = cbc_mac(&[1, 2, 3], &icv, &s1, &s2);
        let m2 = cbc_mac(&[1, 2, 3], &icv, &s1, &s2);
        assert_eq!(m1, m2);
    }

    /// Different inputs produce different MACs.
    #[test]
    fn distinct_inputs_distinct_macs() {
        let icv = [0u8; 16];
        let s1 = [1u8; 16];
        let s2 = [2u8; 16];
        let m1 = cbc_mac(&[1], &icv, &s1, &s2);
        let m2 = cbc_mac(&[2], &icv, &s1, &s2);
        assert_ne!(m1, m2);
    }

    /// ICV change shifts the MAC.
    #[test]
    fn icv_matters() {
        let s1 = [1u8; 16];
        let s2 = [2u8; 16];
        let m1 = cbc_mac(&[7, 8, 9], &[0u8; 16], &s1, &s2);
        let m2 = cbc_mac(&[7, 8, 9], &[1u8; 16], &s1, &s2);
        assert_ne!(m1, m2);
    }

    /// Multi-block input uses S-MAC1 then S-MAC2.
    #[test]
    fn key_swap_observable() {
        let icv = [0u8; 16];
        let s1 = [1u8; 16];
        let s2_a = [2u8; 16];
        let s2_b = [3u8; 16];
        let input = [0u8; 32];
        let m1 = cbc_mac(&input, &icv, &s1, &s2_a);
        let m2 = cbc_mac(&input, &icv, &s1, &s2_b);
        assert_ne!(m1, m2);
    }
}
