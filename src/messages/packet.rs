use crate::models::reply::{
    acknowledge::{GeneralAcknowledge, NegativeAcknowledge},
    report::DeviceIdentificationReport,
};

use super::reply::{OSDPReply, ReplyData, ReplyDeserializationError, ReplyType};

pub struct ReplyPacket {
    pub reply_type: ReplyType,
    pub buffer_raw: Vec<u8>,
    pub buffer_data: Vec<u8>,
}

macro_rules! deserialize_to_enum {
    ($t:ty, $variant:ident, $buffer:expr) => {{
        <$t>::deserialize($buffer).map(ReplyData::$variant)
    }};
}

impl ReplyPacket {
    pub fn parse(&self) -> Result<ReplyData, ReplyDeserializationError> {
        let buffer = self.buffer_data.as_slice();

        match self.reply_type {
            ReplyType::Ack => deserialize_to_enum!(GeneralAcknowledge, ACK, buffer),
            ReplyType::Nak => deserialize_to_enum!(NegativeAcknowledge, NAK, buffer),
            ReplyType::PdIdReport => deserialize_to_enum!(DeviceIdentificationReport, PDID, buffer),
            ReplyType::PdCapabilitiesReport => todo!(),
            ReplyType::LocalStatusReport => todo!(),
            ReplyType::InputStatusReport => todo!(),
            ReplyType::OutputStatusReport => todo!(),
            ReplyType::ReaderStatusReport => todo!(),
            ReplyType::RawReaderData => todo!(),
            ReplyType::FormattedReaderData => todo!(),
            ReplyType::KeypadData => todo!(),
            ReplyType::PdCommunicationsConfigurationReport => todo!(),
            ReplyType::BiometricData => todo!(),
            ReplyType::BiometricMatchResult => todo!(),
            ReplyType::CrypticData => todo!(),
            ReplyType::InitialRMac => todo!(),
            ReplyType::Busy => todo!(),
            ReplyType::FileTransferStatus => todo!(),
            ReplyType::PIVData => todo!(),
            ReplyType::ResponseToChallenge => todo!(),
            ReplyType::ManufactureSpecific => todo!(),
            ReplyType::ExtendedRead => todo!(),
        }
    }
}
