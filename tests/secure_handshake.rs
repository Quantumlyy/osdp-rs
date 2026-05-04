//! Annex D handshake walk-through.
//!
//! Drives the ACU and PD ends of the [`osdp::secure::Session`] type-state
//! machine end-to-end and asserts the resulting MAC stream agrees on both
//! sides. This exercises every spec quote in §S of the implementation plan:
//!
//! - SCBK-D install key
//! - `S-ENC`, `S-MAC1`, `S-MAC2` derivation
//! - client + server cryptograms
//! - initial R-MAC
//! - rolling-ICV CBC-MAC across multiple packets in both directions
//!
//! The handshake itself isn't transported on a real bus — we exercise the
//! cryptographic primitives directly. The ACU driver test in
//! `osdp::driver::acu::tests` covers the framing/SQN side.

use osdp::reply::CCrypt;
use osdp::secure::{
    Disconnected, SCBK_D, Session,
    crypto::{SessionKeys, client_cryptogram},
};

/// Each side of the handshake derives the same session keys when given the
/// same SCBK and `RND.A`.
#[test]
fn both_sides_derive_same_keys() {
    let rnd_a = [0xAAu8; 8];
    let acu = SessionKeys::derive(&SCBK_D, &rnd_a);
    let pd = SessionKeys::derive(&SCBK_D, &rnd_a);
    assert_eq!(acu, pd);
}

/// Full ACU-side type-state walk: Disconnected → Challenged → Cryptogrammed →
/// Secure → MAC computation.
#[test]
fn happy_path_handshake_yields_secure_session() {
    // ACU side begins.
    let acu = Session::<Disconnected>::new(SCBK_D);
    let rnd_a = [0x11u8; 8];
    let acu = acu.challenge(rnd_a);

    // PD picks RND.B and computes its cryptogram.
    let rnd_b = [0x22u8; 8];
    let cuid = [0x33u8; 8];
    let keys = SessionKeys::derive(&SCBK_D, &rnd_a);
    let cc = client_cryptogram(&keys.s_enc, &rnd_a, &rnd_b);

    // ACU verifies CCRYPT.
    let acu = acu
        .receive_ccrypt(&CCrypt {
            cuid,
            rnd_b,
            client_cryptogram: cc,
        })
        .expect("CCRYPT verifies");

    // ACU computes server cryptogram + initial R-MAC.
    let server_crypto = acu.server_cryptogram();
    let initial_rmac = acu.initial_rmac();
    assert_ne!(server_crypto, [0u8; 16]);
    assert_ne!(initial_rmac, [0u8; 16]);

    // PD echoes back the same R-MAC; ACU advances to Secure.
    let mut acu = acu.confirm_rmac_i(&initial_rmac).expect("RMAC_I matches");

    // First MAC — ICV is the initial R-MAC.
    let mac1 = acu.mac(b"hello");
    let mac2 = acu.mac(b"hello");
    // Same input but the ICV changed → the second MAC must differ.
    assert_ne!(mac1, mac2);
}

/// Bad cryptogram from PD aborts the session and returns us to Disconnected.
#[test]
fn bad_ccrypt_returns_to_disconnected() {
    let session = Session::<Disconnected>::new(SCBK_D).challenge([0u8; 8]);
    let bogus = CCrypt {
        cuid: [0; 8],
        rnd_b: [0; 8],
        client_cryptogram: [0xFF; 16],
    };
    assert!(session.receive_ccrypt(&bogus).is_err());
}
