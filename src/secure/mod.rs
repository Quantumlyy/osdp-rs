//! Secure Channel — Annex D.
//!
//! # Spec: Annex D
//!
//! Three layers:
//!
//! 1. [`crypto`] — primitives: AES-128, key derivation, cryptograms.
//! 2. [`mac`] — CBC-MAC with the S-MAC1/S-MAC2 swap on the final block.
//! 3. [`session`] — type-state machine wrapping the above into a usable API.
//!
//! Padding is described in [`pad`].

pub mod crypto;
pub mod mac;
pub mod pad;
pub mod session;

pub use session::{Disconnected, Cryptogrammed, Challenged, Secure, Session};

/// Default install key (`SCBK-D`): bytes `0x30..=0x3F`.
///
/// # Spec: Annex D.8
pub const SCBK_D: [u8; 16] = [
    0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3A, 0x3B, 0x3C, 0x3D, 0x3E,
    0x3F,
];
