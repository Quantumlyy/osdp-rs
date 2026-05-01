//! Packet codec — framing, header, security control block, MAC, trailer.
//!
//! # Spec: §5.9 Tables 1–3, Annex C

mod checksum;
mod codec;
mod crc;
mod header;
mod scb;
mod trailer;

pub use checksum::checksum8;
pub use codec::{ParsedPacket, HEADER_LEN, MAC_LEN};
pub use crc::{crc16, crc16_le};
pub use header::{Address, ControlByte, CtrlFlags, Sqn};
pub use scb::{ScbView, ScsType};
pub use trailer::Trailer;

#[cfg(feature = "alloc")]
pub use codec::PacketBuilder;
#[cfg(feature = "alloc")]
pub use scb::Scb;

/// Re-export of the high-level [`PacketBuilder`] under a friendlier name.
#[cfg(feature = "alloc")]
pub type Packet = PacketBuilder;
