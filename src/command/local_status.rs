//! Status-request commands with empty payloads:
//!
//! - `osdp_LSTAT` (`0x64`) — local (tamper/power) status.
//! - `osdp_ISTAT` (`0x65`) — input contact status.
//! - `osdp_OSTAT` (`0x66`) — output status.
//! - `osdp_RSTAT` (`0x67`) — reader-tamper status.
//!
//! # Spec: §6.4 – §6.7

use crate::error::Error;
use crate::payload_util::require_exact_len;
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
                require_exact_len(data, 0, $code)?;
                Ok(Self)
            }
        }
    };
}

empty_status!(LocalStatus, 0x64, "osdp_LSTAT");
empty_status!(InputStatus, 0x65, "osdp_ISTAT");
empty_status!(OutputStatus, 0x66, "osdp_OSTAT");
empty_status!(ReaderStatus, 0x67, "osdp_RSTAT");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lstat_roundtrip_empty() {
        assert!(LocalStatus.encode().unwrap().is_empty());
        assert_eq!(LocalStatus::decode(&[]).unwrap(), LocalStatus);
    }

    #[test]
    fn lstat_rejects_payload() {
        assert!(matches!(
            LocalStatus::decode(&[0x00]),
            Err(Error::PayloadLength { code: 0x64, .. })
        ));
    }

    #[test]
    fn istat_rejects_payload() {
        assert!(matches!(
            InputStatus::decode(&[0x00]),
            Err(Error::PayloadLength { code: 0x65, .. })
        ));
    }

    #[test]
    fn ostat_rejects_payload() {
        assert!(matches!(
            OutputStatus::decode(&[0x00]),
            Err(Error::PayloadLength { code: 0x66, .. })
        ));
    }

    #[test]
    fn rstat_rejects_payload() {
        assert!(matches!(
            ReaderStatus::decode(&[0x00]),
            Err(Error::PayloadLength { code: 0x67, .. })
        ));
    }
}
