//! Annex D handshake walkthrough — drives both sides of the secure-channel
//! type-state machine and seals/unseals an encrypted payload across the
//! resulting session.
//!
//! Run with:
//!
//! ```sh
//! cargo run --example handshake --features secure-channel
//! ```

use osdp::packet::{Address, ParsedPacket, Sqn};
use osdp::reply::CCrypt;
use osdp::secure::{
    crypto::{client_cryptogram, SessionKeys},
    seal, unseal, Direction, Disconnected, Session, SCBK_D,
};

fn main() {
    println!("OSDP secure channel handshake walkthrough");

    // Both sides start from the install key (SCBK-D).
    let acu = Session::<Disconnected>::new(SCBK_D);
    let pd = Session::<Disconnected>::new(SCBK_D);

    // ACU sends CHLNG with RND.A.
    let rnd_a = [0x11u8, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88];
    let acu = acu.challenge(rnd_a);
    let pd = pd.challenge(rnd_a);
    println!("  → CHLNG with RND.A = {rnd_a:02X?}");

    // PD picks RND.B + cUID and computes the client cryptogram.
    let rnd_b = [0xAAu8, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x00, 0x11];
    let cuid = [0xDEu8, 0xAD, 0xBE, 0xEF, 0xCA, 0xFE, 0xBA, 0xBE];
    let keys = SessionKeys::derive(&SCBK_D, &rnd_a);
    let client_crypto = client_cryptogram(&keys.s_enc, &rnd_a, &rnd_b);
    let ccrypt = CCrypt {
        cuid,
        rnd_b,
        client_cryptogram: client_crypto,
    };
    println!("  ← CCRYPT (cUID, RND.B, client cryptogram)");

    // ACU verifies CCRYPT, advances to Cryptogrammed.
    let acu = acu.receive_ccrypt(&ccrypt).expect("CCRYPT verifies");
    let pd = pd.receive_ccrypt(&ccrypt).expect("PD self-check");

    // ACU computes the server cryptogram + initial R-MAC.
    let server_crypto = acu.server_cryptogram();
    let initial_rmac = acu.initial_rmac();
    println!("  → SCRYPT (server cryptogram)");

    let mut acu = acu.confirm_rmac_i(&initial_rmac).expect("R-MAC matches");
    let mut pd = pd.confirm_rmac_i(&initial_rmac).expect("R-MAC matches");
    println!("  ← RMAC_I; both ends now Secure");
    let _ = server_crypto;

    // Send a sealed COMSET (encrypted DATA + MAC).
    let sealed = seal(
        &mut acu,
        Address::pd(0x05).unwrap(),
        Sqn::new(1).unwrap(),
        Direction::AcuToPd,
        true,
        0x6E, // COMSET
        &[0x05, 0x80, 0x25, 0x00, 0x00],
    )
    .expect("seal");
    println!(
        "  → encrypted COMSET ({} bytes total, including SCB+MAC+CRC)",
        sealed.len()
    );

    // PD parses + unseals.
    let (parsed, _used) = ParsedPacket::parse(&sealed).expect("frame parses");
    let plain = unseal(&mut pd, &parsed, &sealed).expect("MAC verified");
    println!("  ← decrypted payload = {plain:02X?}");
}
