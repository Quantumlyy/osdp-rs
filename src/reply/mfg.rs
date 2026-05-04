//! Manufacturer-specific replies.
//!
//! # Spec: §7.23–§7.25
//!
//! - `osdp_MFGSTATR` (`0x83`) — vendor status (OUI + payload).
//! - `osdp_MFGERRR` (`0x84`) — vendor error (OUI + error code + payload).
//! - `osdp_MFGREP` (`0x90`) — vendor reply (OUI + payload).

use crate::error::Error;
use alloc::vec::Vec;

macro_rules! oui_plus_payload {
    ($Ty:ident, $code:expr, $doc:literal) => {
        #[doc = $doc]
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct $Ty {
            /// IEEE-assigned OUI.
            pub oui: [u8; 3],
            /// Vendor payload.
            pub payload: Vec<u8>,
        }

        impl $Ty {
            /// Encode.
            pub fn encode(&self) -> Result<Vec<u8>, Error> {
                let mut out = Vec::with_capacity(3 + self.payload.len());
                out.extend_from_slice(&self.oui);
                out.extend_from_slice(&self.payload);
                Ok(out)
            }

            /// Decode.
            pub fn decode(data: &[u8]) -> Result<Self, Error> {
                if data.len() < 3 {
                    return Err(Error::MalformedPayload {
                        code: $code,
                        reason: "manufacturer reply requires 3-byte OUI",
                    });
                }
                let mut oui = [0u8; 3];
                oui.copy_from_slice(&data[..3]);
                Ok(Self {
                    oui,
                    payload: data[3..].to_vec(),
                })
            }
        }
    };
}

oui_plus_payload!(
    MfgStatR,
    0x83,
    "`osdp_MFGSTATR` body — manufacturer status reply."
);
oui_plus_payload!(
    MfgErrR,
    0x84,
    "`osdp_MFGERRR` body — manufacturer error reply."
);
oui_plus_payload!(MfgRep, 0x90, "`osdp_MFGREP` body — manufacturer reply.");
