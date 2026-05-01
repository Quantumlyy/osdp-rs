//! 0x80-padding rules used by the secure channel.
//!
//! # Spec: Annex D.5
//!
//! - **MAC**: pad input with `0x80` then zeros only if it isn't already a
//!   multiple of 16.
//! - **DATA encryption (SCS_17/18)**: pad with `0x80` then zeros, *always*
//!   (even when the input is already a multiple of 16).

/// Append 0x80-padding for MAC inputs ("only if needed").
pub fn pad_mac(buf: &mut alloc::vec::Vec<u8>) {
    let rem = buf.len() % 16;
    if rem == 0 {
        return;
    }
    buf.push(0x80);
    let need = 16 - ((rem + 1) % 16);
    if need != 16 {
        buf.extend(core::iter::repeat_n(0u8, need));
    }
}

/// Append 0x80-padding for encrypted DATA ("always").
pub fn pad_data(buf: &mut alloc::vec::Vec<u8>) {
    buf.push(0x80);
    let rem = buf.len() % 16;
    if rem != 0 {
        buf.extend(core::iter::repeat_n(0u8, 16 - rem));
    }
}

/// Strip 0x80-padding from decrypted DATA. Returns the stripped slice.
///
/// Errors if the trailing bytes do not match `0x80 [0x00]*`.
pub fn unpad_data(buf: &[u8]) -> Result<&[u8], crate::error::SecureSessionError> {
    let mut i = buf.len();
    while i > 0 {
        i -= 1;
        match buf[i] {
            0x00 => continue,
            0x80 => return Ok(&buf[..i]),
            _ => return Err(crate::error::SecureSessionError::BadPadding),
        }
    }
    Err(crate::error::SecureSessionError::BadPadding)
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec::Vec;

    #[test]
    fn mac_padding_skipped_when_aligned() {
        let mut buf: Vec<u8> = (0..16).collect();
        pad_mac(&mut buf);
        assert_eq!(buf.len(), 16);
    }

    #[test]
    fn mac_padding_when_unaligned() {
        let mut buf: Vec<u8> = (0..3).collect();
        pad_mac(&mut buf);
        assert_eq!(buf.len(), 16);
        assert_eq!(buf[3], 0x80);
        assert!(buf[4..].iter().all(|&b| b == 0));
    }

    #[test]
    fn data_padding_always_applied() {
        let mut buf: Vec<u8> = (0..16).collect();
        pad_data(&mut buf);
        assert_eq!(buf.len(), 32);
        assert_eq!(buf[16], 0x80);
    }

    #[test]
    fn unpad_roundtrip() {
        for n in 0..32usize {
            let mut buf: Vec<u8> = (0..n as u8).collect();
            let original = buf.clone();
            pad_data(&mut buf);
            let stripped = unpad_data(&buf).unwrap();
            assert_eq!(stripped, original);
        }
    }
}
