use crate::{
    messages::reply::{OSDPReply, ReplyType},
    models::reply::report::DeviceIdentificationReport,
};

impl OSDPReply for DeviceIdentificationReport {
    fn rply(&self) -> ReplyType {
        ReplyType::PdIdReport
    }

    fn deserialize(data: &[u8]) -> Self {
        let sized_data: &[u8; 12] = data.try_into().unwrap();
        let sized_serial_numbers: &[u8; 4] = data[5..8]
            .try_into()
            .unwrap();
        let sized_firmware_revision_numbers: &[u8; 3] = data[9..11].try_into().unwrap();

        Self::new(
            sized_data[0],
            sized_data[1],
            sized_data[2],
            sized_data[3],
            sized_data[4],
            u32::from_le_bytes(*sized_serial_numbers),
            *sized_firmware_revision_numbers,
        )
    }
}
