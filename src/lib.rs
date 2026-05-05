//! Pure-Rust, `no_std`-friendly implementation of the SIA OSDP v2.2 protocol.
//!
//! # Crate features
//!
//! - `std` *(default)*: enables [`alloc`], plus desktop convenience.
//! - `alloc` *(default via `std`)*: enables [`alloc::vec::Vec`]-based codec paths.
//! - `secure-channel` *(default)*: enables Annex D (AES-128, MAC, encryption).
//! - `embedded-io` / `embedded-io-async`: transport adapters.
//! - `defmt`: structured logging on embedded targets.
//!
//! # Layering
//!
//! - [`packet`] — wire framing (SOM, header, SCB, MAC, trailer)
//! - [`command`] / [`reply`] — typed messages
//! - [`multipart`] — RFC §5.10 multi-part assembly / disassembly
//! - [`secure`] — Annex D secure channel
//! - [`transport`] — byte-stream abstraction
//! - [`driver`] — ACU and PD state machines
//! - [`caps`] — Annex B function codes
//!
//! # Specification cross-references
//!
//! All spec citations refer to *SIA OSDP v2.2* (©2020 Security Industry
//! Association). Where an item maps directly onto a spec section it is noted
//! in the docs as `# Spec: §X.Y`.

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(missing_docs)]
#![deny(unsafe_code)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod caps;
pub mod clock;
pub mod error;
pub mod packet;
pub mod transport;

#[cfg(feature = "alloc")]
mod payload_util;

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub mod command;
#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub mod multipart;
#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub mod reply;

#[cfg(all(feature = "alloc", feature = "secure-channel"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "alloc", feature = "secure-channel"))))]
pub mod secure;

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub mod driver;

pub use error::{Error, Result};
pub use packet::{Address, ControlByte, ParsedPacket, Sqn, Trailer};

#[cfg(feature = "alloc")]
pub use packet::Packet;

/// Start of message marker — begins every OSDP packet header.
///
/// # Spec: §5.9
pub const SOM: u8 = 0x53;

/// Broadcast address.
///
/// # Spec: §5.4
///
/// Address `0x7F` is reserved as a special "BROADCAST" address that each PD
/// will accept and respond to. The reply uses `0x7F | 0x80 = 0xFF` in its
/// address field.
pub const BROADCAST_ADDR: u8 = 0x7F;

/// Largest legal PD unicast address.
///
/// # Spec: §5.4
pub const MAX_PD_ADDR: u8 = 0x7E;

/// Reply flag set in [`Address`] when sent from PD to ACU.
///
/// # Spec: §5.9, Table 1
pub const REPLY_FLAG: u8 = 0x80;

/// Minimum receive-buffer size every PD must support.
///
/// # Spec: §5.6
pub const MIN_RX_SIZE: usize = 128;

/// Maximum packet length that any device must tolerate on the wire (even if
/// addressed elsewhere).
///
/// # Spec: §5.6
pub const MAX_BUS_PACKET: usize = 1440;

/// Default ACU reply-delay budget.
///
/// # Spec: §5.7
pub const REPLY_DELAY_MS: u32 = 200;

/// Off-line declaration threshold.
///
/// # Spec: §5.7
pub const OFFLINE_THRESHOLD_MS: u32 = 8_000;
