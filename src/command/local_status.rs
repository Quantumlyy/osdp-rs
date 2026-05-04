//! Status-request commands with empty payloads:
//!
//! - `osdp_LSTAT` (`0x64`) — local (tamper/power) status.
//! - `osdp_ISTAT` (`0x65`) — input contact status.
//! - `osdp_OSTAT` (`0x66`) — output status.
//! - `osdp_RSTAT` (`0x67`) — reader-tamper status.
//!
//! # Spec: §6.4 – §6.7

use crate::error::Error;
use alloc::vec::Vec;

macro_rules! empty_status {
    ($Ty:ident, $code:expr, $name:literal) => {
        #[doc = concat!("`", $name, "` body (empty).")]
        #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
        pub struct $Ty;

        impl $Ty {
            /// Encode (always empty).
            pub fn encode(&self) -> Result<Vec<u8>, Error> {
                Ok(Vec::new())
            }

            /// Decode (must be empty).
            pub fn decode(data: &[u8]) -> Result<Self, Error> {
                if !data.is_empty() {
                    return Err(Error::MalformedPayload {
                        code: $code,
                        reason: concat!($name, " has no payload"),
                    });
                }
                Ok(Self)
            }
        }
    };
}

empty_status!(LocalStatus, 0x64, "osdp_LSTAT");
empty_status!(InputStatus, 0x65, "osdp_ISTAT");
empty_status!(OutputStatus, 0x66, "osdp_OSTAT");
empty_status!(ReaderStatus, 0x67, "osdp_RSTAT");
