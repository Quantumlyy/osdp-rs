//! 8-bit two's-complement checksum used for the weak frame trailer.
//!
//! # Spec: §5.9
//!
//! The transmitted byte is the negation of the modulo-256 sum so that the sum
//! of all bytes including the trailer is `0`.

/// Compute the 8-bit OSDP checksum over `bytes`.
#[inline]
pub fn checksum8(bytes: &[u8]) -> u8 {
    let sum: u32 = bytes.iter().map(|&b| u32::from(b)).sum();
    (!(sum as u8)).wrapping_add(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `osdp_COMSET` packet trailer.
    #[test]
    fn comset_vector() {
        let buf = [0x53, 0x7F, 0x0C, 0x00, 0x00, 0x6E, 0x00, 0x80, 0x25, 0x00];
        assert_eq!(checksum8(&buf), 0x0F);
    }

    /// `osdp_ID` packet trailer.
    #[test]
    fn id_vector() {
        let buf = [0x53, 0x00, 0x08, 0x00, 0x00, 0x61];
        assert_eq!(checksum8(&buf), 0x44);
    }

    /// Trailer is its own inverse: sum of all bytes including the checksum is 0.
    #[test]
    fn trailer_closes_to_zero() {
        let buf = [0x53, 0x7F, 0x0C, 0x00, 0x00, 0x6E, 0x00, 0x80, 0x25, 0x00];
        let trailer = checksum8(&buf);
        let total: u32 = buf
            .iter()
            .copied()
            .chain(core::iter::once(trailer))
            .map(u32::from)
            .sum();
        assert_eq!(total & 0xFF, 0);
    }
}
