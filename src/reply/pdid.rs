//! `osdp_PDID` (`0x45`) — peripheral device identification.
//!
//! # Spec: §7.4
//!
//! Body is exactly 12 bytes:
//!
//! ```text
//! +-----+-----+-----+-------+--------+----------+----------+----------+----------+----------+----------+
//! |VC1  |VC2  |VC3  |Model# |Version |Serial#0  |Serial#1  |Serial#2  |Serial#3  |FW major  |FW minor  |FW build|
//! +-----+-----+-----+-------+--------+----------+----------+----------+----------+----------+----------+
//!  byte0 byte1 byte2 byte3   byte4    byte5..8 (LE u32)                          byte9..11
//! ```

use crate::error::Error;
use alloc::vec::Vec;

/// `osdp_PDID` body.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PdId {
    /// IEEE-assigned OUI.
    pub vendor_oui: [u8; 3],
    /// Model number.
    pub model: u8,
    /// Hardware/firmware family version.
    pub version: u8,
    /// 32-bit serial number.
    pub serial: u32,
    /// Firmware revision: `[major, minor, build]`.
    pub firmware: [u8; 3],
}

impl PdId {
    /// Wire size.
    pub const WIRE_LEN: usize = 12;

    /// Encode.
    pub fn encode(&self) -> Result<Vec<u8>, Error> {
        let mut out = Vec::with_capacity(Self::WIRE_LEN);
        out.extend_from_slice(&self.vendor_oui);
        out.push(self.model);
        out.push(self.version);
        out.extend_from_slice(&self.serial.to_le_bytes());
        out.extend_from_slice(&self.firmware);
        Ok(out)
    }

    /// Decode.
    pub fn decode(data: &[u8]) -> Result<Self, Error> {
        if data.len() != Self::WIRE_LEN {
            return Err(Error::MalformedPayload {
                code: 0x45,
                reason: "PDID requires 12 bytes",
            });
        }
        let mut vendor_oui = [0u8; 3];
        vendor_oui.copy_from_slice(&data[..3]);
        let mut firmware = [0u8; 3];
        firmware.copy_from_slice(&data[9..12]);
        Ok(Self {
            vendor_oui,
            model: data[3],
            version: data[4],
            serial: u32::from_le_bytes([data[5], data[6], data[7], data[8]]),
            firmware,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let id = PdId {
            vendor_oui: [0x00, 0x06, 0x8E], // HID's OUI
            model: 0x12,
            version: 0x34,
            serial: 0xDEAD_BEEF,
            firmware: [1, 2, 3],
        };
        let bytes = id.encode().unwrap();
        assert_eq!(bytes.len(), 12);
        let parsed = PdId::decode(&bytes).unwrap();
        assert_eq!(parsed, id);
    }

    #[test]
    fn rejects_truncated() {
        assert!(PdId::decode(&[0; 11]).is_err());
        assert!(PdId::decode(&[0; 13]).is_err());
    }
}
