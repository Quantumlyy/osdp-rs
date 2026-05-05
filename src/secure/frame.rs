//! Helpers for constructing and parsing fully-secured frames.
//!
//! These tie together [`super::session::Session<Secure>`], the
//! [`crate::packet::PacketBuilder`], and the AES-CBC payload codec into a
//! single ergonomic call.
//!
//! # Spec: Annex D.5

use crate::error::{Error, SecureSessionError};
use crate::packet::{Address, ControlByte, CtrlFlags, PacketBuilder, ParsedPacket, Scb, ScsType};
use crate::secure::session::{Secure, Session};
use alloc::vec::Vec;

/// What kind of secured frame we're building.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    /// ACU → PD: produces SCS_15 (mac-only) or SCS_17 (encrypted+mac).
    AcuToPd,
    /// PD → ACU: produces SCS_16 (mac-only) or SCS_18 (encrypted+mac).
    PdToAcu,
}

impl Direction {
    /// Pick the SCS type based on whether the payload should be encrypted.
    pub const fn scs_for(self, encrypt: bool) -> ScsType {
        match (self, encrypt) {
            (Self::AcuToPd, false) => ScsType::Scs15,
            (Self::AcuToPd, true) => ScsType::Scs17,
            (Self::PdToAcu, false) => ScsType::Scs16,
            (Self::PdToAcu, true) => ScsType::Scs18,
        }
    }
}

/// Build a fully-secured packet (SCB + optional encryption + MAC + trailer).
///
/// `data` is the *plaintext* DATA payload — it will be AES-CBC encrypted with
/// the current ICV if `encrypt` is true. The `code` byte is *not* encrypted
/// (it sits in front of DATA in the wire layout).
pub fn seal(
    session: &mut Session<Secure>,
    addr: Address,
    sqn: crate::packet::Sqn,
    direction: Direction,
    encrypt: bool,
    code: u8,
    data: &[u8],
) -> Result<Vec<u8>, Error> {
    let ty = direction.scs_for(encrypt);
    let scb = Scb::new(ty, []);
    let payload: Vec<u8> = if encrypt {
        session.seal_data(data)
    } else {
        data.to_vec()
    };
    let builder = PacketBuilder {
        addr,
        ctrl: ControlByte::new(sqn, CtrlFlags::USE_CRC | CtrlFlags::HAS_SCB),
        scb: Some(scb),
        code,
        data: payload,
    };
    builder.encode_with_mac(|bytes| {
        let full = session.mac(bytes);
        let mut tag = [0u8; crate::packet::MAC_LEN];
        tag.copy_from_slice(&full[..crate::packet::MAC_LEN]);
        tag
    })
}

/// Verify the MAC on `parsed` and (if SCS_17/18) decrypt the DATA payload.
///
/// The caller is expected to first run [`ParsedPacket::parse`] and verify
/// that a [`crate::packet::ScsType`] is present and is one of `SCS_15..=18`.
pub fn unseal(
    session: &mut Session<Secure>,
    parsed: &ParsedPacket<'_>,
    raw: &[u8],
) -> Result<Vec<u8>, Error> {
    let scb = parsed
        .scb
        .ok_or(Error::SecureSession(SecureSessionError::NotSecure))?;
    if !scb.ty.has_mac() {
        return Err(Error::SecureSession(SecureSessionError::NotSecure));
    }
    let trailer_len = if parsed.ctrl.use_crc() { 2 } else { 1 };
    let mac_offset = raw
        .len()
        .checked_sub(trailer_len + crate::packet::MAC_LEN)
        .ok_or(Error::ShortMac)?;
    let mac_bytes = parsed.mac.ok_or(Error::ShortMac)?;

    // Decrypt DATA first, while the rolling-ICV still reflects the *prior*
    // MAC. `verify` will then advance the chain.
    let plaintext = if scb.ty.is_encrypted() {
        session.open_data(parsed.data).map_err(Error::from)?
    } else {
        parsed.data.to_vec()
    };
    session
        .verify(&raw[..mac_offset], &mac_bytes)
        .map_err(Error::from)?;
    Ok(plaintext)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::Sqn;
    use crate::reply::CCrypt;
    use crate::secure::SCBK_D;
    use crate::secure::crypto::{SessionKeys, client_cryptogram};
    use crate::secure::session::{Disconnected, Session};

    fn handshake_pair() -> (Session<Secure>, Session<Secure>) {
        // ACU side
        let acu = Session::<Disconnected>::new(SCBK_D);
        let rnd_a = [0xAAu8; 8];
        let acu = acu.challenge(rnd_a);

        // PD side computes the same things to mirror state.
        let rnd_b = [0xBBu8; 8];
        let cuid = [0xCCu8; 8];
        let keys = SessionKeys::derive(&SCBK_D, &rnd_a);
        let cc = client_cryptogram(&keys.s_enc, &rnd_a, &rnd_b);

        let acu = acu
            .receive_ccrypt(&CCrypt {
                cuid,
                rnd_b,
                client_cryptogram: cc,
            })
            .unwrap();
        let rmac = acu.initial_rmac();
        let acu = acu.confirm_rmac_i(&rmac).unwrap();

        // PD side mirror: Disconnected → Challenged → Cryptogrammed → Secure
        let pd = Session::<Disconnected>::new(SCBK_D).challenge(rnd_a);
        let pd = pd
            .receive_ccrypt(&CCrypt {
                cuid,
                rnd_b,
                client_cryptogram: cc,
            })
            .unwrap();
        let pd_rmac = pd.initial_rmac();
        let pd = pd.confirm_rmac_i(&pd_rmac).unwrap();

        // Both sides agree on the same initial R-MAC seed.
        assert_eq!(rmac, pd_rmac);
        (acu, pd)
    }

    #[test]
    fn seal_then_unseal_mac_only() {
        let (mut acu, mut pd) = handshake_pair();
        let bytes = seal(
            &mut acu,
            Address::pd(0x05).unwrap(),
            Sqn::new(1).unwrap(),
            Direction::AcuToPd,
            false,
            0x60, // POLL
            &[],
        )
        .unwrap();
        let (parsed, _used) = ParsedPacket::parse(&bytes).unwrap();
        let plain = unseal(&mut pd, &parsed, &bytes).unwrap();
        assert!(plain.is_empty());
    }

    #[test]
    fn seal_then_unseal_encrypted() {
        let (mut acu, mut pd) = handshake_pair();
        let payload = b"sensitive command data";
        let bytes = seal(
            &mut acu,
            Address::pd(0x05).unwrap(),
            Sqn::new(2).unwrap(),
            Direction::AcuToPd,
            true,
            0x6E, // COMSET
            payload,
        )
        .unwrap();
        let (parsed, _used) = ParsedPacket::parse(&bytes).unwrap();
        // The on-wire DATA is the ciphertext, padded to 16-byte multiple.
        assert_eq!(parsed.data.len() % 16, 0);
        assert_ne!(parsed.data, payload);

        let plain = unseal(&mut pd, &parsed, &bytes).unwrap();
        assert_eq!(plain, payload);
    }

    #[test]
    fn tampered_mac_rejected() {
        let (mut acu, mut pd) = handshake_pair();
        let mut bytes = seal(
            &mut acu,
            Address::pd(0x05).unwrap(),
            Sqn::new(1).unwrap(),
            Direction::AcuToPd,
            false, // mac-only so we don't have the larger encrypt-then-mac surface
            0x60,
            &[],
        )
        .unwrap();
        // Flip a MAC byte and re-CRC so the parser accepts the frame.
        let n = bytes.len();
        let mac_byte = n - 2 - 4; // skip CRC (2) and target the last MAC byte
        bytes[mac_byte] ^= 0x01;
        let crc = crate::packet::crc16(&bytes[..n - 2]);
        bytes[n - 2..].copy_from_slice(&crc.to_le_bytes());

        let (parsed, _used) = ParsedPacket::parse(&bytes).unwrap();
        let err = unseal(&mut pd, &parsed, &bytes).unwrap_err();
        assert!(matches!(err, Error::SecureSession(_)));
    }
}
