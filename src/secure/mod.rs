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
//! Padding is described in [`pad`]. The full handshake is rendered in
//! [`handshake`].

pub mod cipher;
pub mod crypto;
pub mod frame;
pub mod mac;
pub mod pad;
pub mod session;

/// Annex D.4 secure-channel handshake.
///
/// Both sides start out sharing only the SCBK (out-of-band install key, or a
/// previously-keyset session key). The handshake derives matching session
/// keys, proves possession to each end, and seeds the rolling-ICV chain that
/// every subsequent SCS_15..=18 frame uses.
///
#[cfg_attr(feature = "_docs", aquamarine::aquamarine)]
/// ```mermaid
/// sequenceDiagram
///     participant ACU
///     participant PD
///     Note over ACU,PD: Both share SCBK out-of-band
///     ACU->>PD: osdp_CHLNG (RND.A)
///     PD->>PD: derive S-ENC, S-MAC1, S-MAC2 from SCBK ⊕ RND.A
///     PD->>PD: pick RND.B; compute ClientCryptogram
///     PD->>ACU: osdp_CCRYPT (cUID, RND.B, ClientCryptogram)
///     ACU->>ACU: derive same keys; verify ClientCryptogram
///     ACU->>ACU: compute ServerCryptogram
///     ACU->>PD: osdp_SCRYPT (ServerCryptogram)
///     PD->>PD: verify ServerCryptogram; compute initial R-MAC
///     PD->>ACU: osdp_RMAC_I (initial R-MAC)
///     ACU->>ACU: verify R-MAC matches own
///     Note over ACU,PD: Session is Secure;<br/>SCS_15..=18 frames carry rolling ICV
/// ```
pub mod handshake {}

pub use frame::{Direction, seal, unseal};
pub use session::{Challenged, Cryptogrammed, Disconnected, Secure, Session};

/// Default install key (`SCBK-D`): bytes `0x30..=0x3F`.
///
/// # Spec: Annex D.8
pub const SCBK_D: [u8; 16] = [
    0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3A, 0x3B, 0x3C, 0x3D, 0x3E, 0x3F,
];
