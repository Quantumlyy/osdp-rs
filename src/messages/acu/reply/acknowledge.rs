use num_traits::FromPrimitive;

use crate::{
    messages::reply::{OSDPReply, ReplyType},
    models::reply::acknowledge::{GeneralAcknowledge, NegativeAcknowledge},
};

impl OSDPReply for GeneralAcknowledge {
    fn rply(&self) -> ReplyType {
        ReplyType::Ack
    }

    fn deserialize(_data: &[u8]) -> Self {
        Self::default()
    }
}

impl OSDPReply for NegativeAcknowledge {
    fn rply(&self) -> ReplyType {
        ReplyType::Nak
    }

    fn deserialize(data: &[u8]) -> Self {
        // This looks bad but if every PD adheres to the OSDP v2.2 spec, which we presume it does, then this should never error.
        let error_code = FromPrimitive::from_u8(data[0]).unwrap();

        Self::new(error_code, data[1..].to_owned())
    }
}
