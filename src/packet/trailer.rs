//! Frame trailer — either an 8-bit checksum or a 16-bit CRC, controlled by
//! the `USE_CRC` bit of the [`super::ControlByte`].

/// Decoded packet trailer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Trailer {
    /// 8-bit two's-complement checksum.
    Checksum(u8),
    /// CRC-16/KERMIT (Annex C).
    Crc(u16),
}

impl Trailer {
    /// `true` if this is a CRC trailer.
    pub const fn is_crc(self) -> bool {
        matches!(self, Self::Crc(_))
    }
}
