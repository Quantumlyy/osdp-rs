//! CRC-16/KERMIT (CCITT) used as the strong frame trailer.
//!
//! # Spec: Annex C
//!
//! Polynomial `0x1021`, initial value `0x1D0F`, no input/output reflect, no
//! final XOR. Transmitted little-endian.

/// Pre-computed table for CRC-16/KERMIT with seed `0x1D0F`.
const TABLE: [u16; 256] = {
    let mut table = [0u16; 256];
    let mut i = 0u16;
    while i < 256 {
        let mut crc = i << 8;
        let mut bit = 0;
        while bit < 8 {
            if (crc & 0x8000) != 0 {
                crc = (crc << 1) ^ 0x1021;
            } else {
                crc <<= 1;
            }
            bit += 1;
        }
        table[i as usize] = crc;
        i += 1;
    }
    table
};

/// Compute the CRC-16/KERMIT over `bytes`.
///
/// The OSDP variant uses initial value `0x1D0F`. The result is transmitted
/// little-endian after the payload.
#[inline]
pub fn crc16(bytes: &[u8]) -> u16 {
    let mut crc: u16 = 0x1D0F;
    for &b in bytes {
        let idx = ((crc >> 8) ^ u16::from(b)) as u8 as usize;
        crc = (crc << 8) ^ TABLE[idx];
    }
    crc
}

/// Compute the CRC-16/KERMIT and return it as little-endian bytes.
#[inline]
pub fn crc16_le(bytes: &[u8]) -> [u8; 2] {
    crc16(bytes).to_le_bytes()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Annex E example: `osdp_COMSET` to broadcast.
    #[test]
    fn comset_broadcast_vector() {
        let buf = [
            0x53, 0x7F, 0x0D, 0x00, 0x04, 0x6E, 0x00, 0x80, 0x25, 0x00, 0x00,
        ];
        assert_eq!(crc16_le(&buf), [0x6E, 0x38]);
    }

    /// Annex E example: `osdp_ID` request.
    #[test]
    fn id_vector() {
        let buf = [0x53, 0x00, 0x09, 0x00, 0x04, 0x61, 0x00];
        assert_eq!(crc16_le(&buf), [0xC0, 0x66]);
    }
}
