use crate::{
    messages::reply::{OSDPReply, ReplyType},
    models::reply::acknowledge::GeneralAcknowledge,
};

impl OSDPReply for GeneralAcknowledge {
    fn rply(&self) -> ReplyType {
        ReplyType::Ack
    }

    fn deserialize(_data: &[u8]) -> Self {
        Self {}
    }
}
