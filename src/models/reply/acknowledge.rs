use num_derive::FromPrimitive;

/// `osdp_ACK`
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct GeneralAcknowledge {}

impl GeneralAcknowledge {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for GeneralAcknowledge {
    fn default() -> Self {
        Self {}
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, FromPrimitive)]
#[repr(u8)]
pub enum NegativeAcknowledgeErrorCode {
    #[default]
    NoError = 0x00,
    BadCksumCrc = 0x01,
    CommandLengthError = 0x02,
    UnknownCommandCode = 0x03,
    UnexpectedSequenceNumber = 0x04,
    UnsupportedSecurityBlock = 0x05,
    EncryptedCommunicationRequired = 0x06,
    BioTypeNotSupported = 0x07,
    BioFormatNotSupported = 0x08,
    UnableToProcessCommand = 0x09,
}

/// `osdp_NAK`
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NegativeAcknowledge {
    pub error_code: NegativeAcknowledgeErrorCode,
    pub data: Vec<u8>,
}

impl NegativeAcknowledge {
    pub fn new(error_code: NegativeAcknowledgeErrorCode, data: Vec<u8>) -> Self {
        Self { error_code, data }
    }
}
