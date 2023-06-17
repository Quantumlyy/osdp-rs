use bitvec::vec::BitVec;

use crate::{
    messages::reply::{OSDPReply, ReplyDeserializationError, ReplyType},
    models::reply::card::CardRawDataReport,
};

impl OSDPReply for CardRawDataReport {
    fn rply(&self) -> ReplyType {
        ReplyType::Nak
    }

    fn deserialize(data: &[u8]) -> Result<CardRawDataReport, ReplyDeserializationError> {
        if data.len() < 4 {
            return Err(ReplyDeserializationError::InvalidPacketSize {
                minimum: 4,
                maximum: usize::MAX,
                received: data.len(),
            });
        }

        Ok(Self::new(
            data[0],
            data[1],
            u16::from_le_bytes([data[2], data[3]]),
            BitVec::<u8>::from_slice(&data[4..]),
        ))
    }
}
