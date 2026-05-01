//! `osdp_PDCAP` (`0x46`) — peripheral capabilities.
//!
//! # Spec: §7.5, Annex B
//!
//! Body is a sequence of [`Capability`] triplets.

use crate::caps::{Capability, FunctionCode};
use crate::error::Error;
use alloc::vec::Vec;

/// `osdp_PDCAP` body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PdCap {
    /// All advertised capabilities, in the order they appeared on the wire.
    pub capabilities: Vec<Capability>,
}

impl PdCap {
    /// New body.
    pub fn new(capabilities: Vec<Capability>) -> Self {
        Self { capabilities }
    }

    /// Look up the first capability matching `code`, if present.
    pub fn capability(&self, code: FunctionCode) -> Option<Capability> {
        self.capabilities
            .iter()
            .copied()
            .find(|c| c.code == code.as_byte())
    }

    /// Encode.
    pub fn encode(&self) -> Result<Vec<u8>, Error> {
        let mut out = Vec::with_capacity(self.capabilities.len() * Capability::WIRE_LEN);
        for c in &self.capabilities {
            out.extend_from_slice(&c.encode());
        }
        Ok(out)
    }

    /// Decode.
    pub fn decode(data: &[u8]) -> Result<Self, Error> {
        if data.len() % Capability::WIRE_LEN != 0 {
            return Err(Error::MalformedPayload {
                code: 0x46,
                reason: "PDCAP payload must be multiple of 3 bytes",
            });
        }
        let capabilities = data
            .chunks_exact(Capability::WIRE_LEN)
            .map(|c| Capability::decode([c[0], c[1], c[2]]))
            .collect();
        Ok(Self { capabilities })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lookup_function_code() {
        let body = PdCap::new(alloc::vec![
            Capability {
                code: FunctionCode::CommunicationSecurity.as_byte(),
                compliance: 1,
                number_of: 1,
            },
            Capability {
                code: FunctionCode::ReceiveBufferSize.as_byte(),
                compliance: 0x80,
                number_of: 0,
            },
        ]);
        let aes = body.capability(FunctionCode::CommunicationSecurity).unwrap();
        assert_eq!(aes.compliance, 1);
        let rxsize = body.capability(FunctionCode::ReceiveBufferSize).unwrap();
        assert_eq!(rxsize.u16_value(), 128);
    }

    #[test]
    fn roundtrip() {
        let body = PdCap::new(alloc::vec![
            Capability { code: 1, compliance: 1, number_of: 4 },
            Capability { code: 2, compliance: 1, number_of: 2 },
        ]);
        let bytes = body.encode().unwrap();
        assert_eq!(bytes, [1, 1, 4, 2, 1, 2]);
        let parsed = PdCap::decode(&bytes).unwrap();
        assert_eq!(parsed, body);
    }
}
