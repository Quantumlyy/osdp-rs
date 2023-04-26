use crate::models::reply::acknowledge::{GeneralAcknowledge, NegativeAcknowledge};

use super::reply::{ReplyType, ReplyData, OSDPReply};

pub struct ReplyPacket {
    pub reply_type: ReplyType,
    pub buffer_raw: Vec<u8>,
    pub buffer_data: Vec<u8>,
}

impl ReplyPacket {
    pub fn parse(&self) -> ReplyData {
        let buffer = self.buffer_data.as_slice();

        match self.reply_type {
            ReplyType::Ack => ReplyData::ACK(GeneralAcknowledge::deserialize(buffer)),
            ReplyType::Nak => ReplyData::NAK(NegativeAcknowledge::deserialize(buffer)),
            ReplyType::PdIdReport => todo!(),
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
