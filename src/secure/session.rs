//! Type-state secure-channel state machine.
//!
//! # Spec: Annex D.4
//!
//! States are phantom-typed so that calling out-of-order is a *compile* error
//! rather than a runtime fault. The intended sequence is:
//!
//! ```text
//! Disconnected --challenge(RND.A)----> Challenged
//! Challenged   --recv_ccrypt(...)----> Cryptogrammed
//! Cryptogrammed--recv_rmac_i(...)----> Secure
//! ```
//!
//! Calls from any state may transition back to [`Disconnected`] on error.

use crate::error::SecureSessionError;
use crate::reply::CCrypt;
use crate::secure::crypto::{
    client_cryptogram, initial_rmac, server_cryptogram, SessionKeys,
};
use crate::secure::mac::cbc_mac;
use core::marker::PhantomData;
use subtle::ConstantTimeEq;

/// Phantom: session not started.
#[derive(Debug, Clone, Copy)]
pub struct Disconnected;
/// Phantom: ACU has sent CHLNG and is awaiting CCRYPT.
#[derive(Debug, Clone, Copy)]
pub struct Challenged;
/// Phantom: ACU has verified CCRYPT and is awaiting RMAC_I.
#[derive(Debug, Clone, Copy)]
pub struct Cryptogrammed;
/// Phantom: SC fully established.
#[derive(Debug, Clone, Copy)]
pub struct Secure;

/// Secure-channel session state.
#[derive(Debug, Clone)]
pub struct Session<S> {
    scbk: [u8; 16],
    rnd_a: [u8; 8],
    rnd_b: [u8; 8],
    cuid: [u8; 8],
    keys: SessionKeys,
    /// Last MAC received from the *other* device — used as ICV for the next
    /// outgoing MAC, per Annex D.5.
    last_their_mac: [u8; 16],
    _state: PhantomData<S>,
}

impl Session<Disconnected> {
    /// Begin a new session, holding the SCBK we will use.
    pub fn new(scbk: [u8; 16]) -> Self {
        Self {
            scbk,
            rnd_a: [0; 8],
            rnd_b: [0; 8],
            cuid: [0; 8],
            keys: SessionKeys {
                s_enc: [0; 16],
                s_mac1: [0; 16],
                s_mac2: [0; 16],
            },
            last_their_mac: [0; 16],
            _state: PhantomData,
        }
    }

    /// Move to [`Challenged`] by capturing the `RND.A` we are about to send.
    pub fn challenge(mut self, rnd_a: [u8; 8]) -> Session<Challenged> {
        self.rnd_a = rnd_a;
        Session {
            scbk: self.scbk,
            rnd_a: self.rnd_a,
            rnd_b: self.rnd_b,
            cuid: self.cuid,
            keys: self.keys,
            last_their_mac: self.last_their_mac,
            _state: PhantomData,
        }
    }
}

impl Session<Challenged> {
    /// Verify the PD's [`CCrypt`] and move to [`Cryptogrammed`].
    pub fn receive_ccrypt(
        mut self,
        ccrypt: &CCrypt,
    ) -> Result<Session<Cryptogrammed>, (Session<Disconnected>, SecureSessionError)> {
        self.cuid = ccrypt.cuid;
        self.rnd_b = ccrypt.rnd_b;
        self.keys = SessionKeys::derive(&self.scbk, &self.rnd_a);
        let expected = client_cryptogram(&self.keys.s_enc, &self.rnd_a, &self.rnd_b);
        if expected.ct_eq(&ccrypt.client_cryptogram).unwrap_u8() == 0 {
            return Err((self.into_disconnected(), SecureSessionError::BadCryptogram));
        }
        Ok(Session {
            scbk: self.scbk,
            rnd_a: self.rnd_a,
            rnd_b: self.rnd_b,
            cuid: self.cuid,
            keys: self.keys,
            last_their_mac: self.last_their_mac,
            _state: PhantomData,
        })
    }

    fn into_disconnected(self) -> Session<Disconnected> {
        Session {
            scbk: self.scbk,
            rnd_a: [0; 8],
            rnd_b: [0; 8],
            cuid: [0; 8],
            keys: SessionKeys {
                s_enc: [0; 16],
                s_mac1: [0; 16],
                s_mac2: [0; 16],
            },
            last_their_mac: [0; 16],
            _state: PhantomData,
        }
    }
}

impl Session<Cryptogrammed> {
    /// Compute the server cryptogram to ship in `osdp_SCRYPT`.
    pub fn server_cryptogram(&self) -> [u8; 16] {
        server_cryptogram(&self.keys.s_enc, &self.rnd_a, &self.rnd_b)
    }

    /// Initial R-MAC, computed from `S-MAC1`/`S-MAC2` over the server cryptogram.
    pub fn initial_rmac(&self) -> [u8; 16] {
        initial_rmac(&self.keys.s_mac1, &self.keys.s_mac2, &self.server_cryptogram())
    }

    /// PD echoed back our initial R-MAC; advance to fully [`Secure`].
    pub fn confirm_rmac_i(
        self,
        their_rmac_i: &[u8; 16],
    ) -> Result<Session<Secure>, (Session<Disconnected>, SecureSessionError)> {
        let mine = self.initial_rmac();
        if mine.ct_eq(their_rmac_i).unwrap_u8() == 0 {
            let mut me = self;
            me.last_their_mac = [0; 16];
            return Err((
                Session {
                    scbk: me.scbk,
                    rnd_a: [0; 8],
                    rnd_b: [0; 8],
                    cuid: [0; 8],
                    keys: SessionKeys {
                        s_enc: [0; 16],
                        s_mac1: [0; 16],
                        s_mac2: [0; 16],
                    },
                    last_their_mac: [0; 16],
                    _state: PhantomData,
                },
                SecureSessionError::BadCryptogram,
            ));
        }
        Ok(Session {
            scbk: self.scbk,
            rnd_a: self.rnd_a,
            rnd_b: self.rnd_b,
            cuid: self.cuid,
            keys: self.keys,
            last_their_mac: mine,
            _state: PhantomData,
        })
    }
}

impl Session<Secure> {
    /// Compute the MAC trailer for an outgoing packet body.
    ///
    /// `bytes` is the SOM..end-of-DATA region (as per [`super::mac::cbc_mac`]).
    pub fn mac(&mut self, bytes: &[u8]) -> [u8; 16] {
        let mac = cbc_mac(bytes, &self.last_their_mac, &self.keys.s_mac1, &self.keys.s_mac2);
        self.last_their_mac = mac;
        mac
    }

    /// Verify a received MAC against our locally-computed value.
    pub fn verify(&mut self, bytes: &[u8], wire_mac: &[u8; 4]) -> Result<(), SecureSessionError> {
        let computed = cbc_mac(bytes, &self.last_their_mac, &self.keys.s_mac1, &self.keys.s_mac2);
        if computed[..4].ct_eq(wire_mac).unwrap_u8() == 0 {
            return Err(SecureSessionError::BadCryptogram);
        }
        self.last_their_mac = computed;
        Ok(())
    }

    /// Borrow the derived session keys (for AES-CBC DATA encryption).
    pub fn keys(&self) -> &SessionKeys {
        &self.keys
    }

    /// Last MAC the other end produced (used as encryption ICV per Annex D).
    pub fn last_other_mac(&self) -> &[u8; 16] {
        &self.last_their_mac
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reply::CCrypt;

    #[test]
    fn handshake_happy_path() {
        let scbk = crate::secure::SCBK_D;
        let session = Session::<Disconnected>::new(scbk);

        let rnd_a = [1u8; 8];
        let session = session.challenge(rnd_a);

        let rnd_b = [2u8; 8];
        let cuid = [3u8; 8];
        let keys = crate::secure::crypto::SessionKeys::derive(&scbk, &rnd_a);
        let cc = crate::secure::crypto::client_cryptogram(&keys.s_enc, &rnd_a, &rnd_b);
        let ccrypt = CCrypt {
            cuid,
            rnd_b,
            client_cryptogram: cc,
        };
        let session = session.receive_ccrypt(&ccrypt).unwrap();

        let rmac = session.initial_rmac();
        let session = session.confirm_rmac_i(&rmac).unwrap();
        let _ = session.last_other_mac();
    }

    #[test]
    fn bad_ccrypt_rejected() {
        let scbk = crate::secure::SCBK_D;
        let session = Session::<Disconnected>::new(scbk).challenge([0u8; 8]);
        let ccrypt = CCrypt {
            cuid: [0; 8],
            rnd_b: [0; 8],
            client_cryptogram: [0; 16],
        };
        let err = session.receive_ccrypt(&ccrypt).unwrap_err();
        assert_eq!(err.1, SecureSessionError::BadCryptogram);
    }
}
