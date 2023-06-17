use crate::{
    messages::reply::{OSDPReply, ReplyType, ReplyDeserializationError},
    models::{reply::report::DeviceIdentificationReport},
};

impl OSDPReply for DeviceIdentificationReport {
    fn rply(&self) -> ReplyType {
        ReplyType::PdIdReport
    }

    fn deserialize(data: &[u8]) -> Result<DeviceIdentificationReport, ReplyDeserializationError> {
        let sized_data: &[u8; 12] = match data.try_into() {
            Ok(data) => data,
            Err(_) => return Err(ReplyDeserializationError::InvalidPacketSizeError {
                minimum: 12,
                maximum: 12,
                received: data.len(),
            })
        };
        let sized_serial_numbers: &[u8; 4] = data[5..8]
            .try_into()
            .unwrap();
        let sized_firmware_revision_numbers: &[u8; 3] = data[9..11].try_into().unwrap();

        Ok(Self::new(
            sized_data[0],
            sized_data[1],
            sized_data[2],
            sized_data[3],
            sized_data[4],
            u32::from_le_bytes(*sized_serial_numbers),
            *sized_firmware_revision_numbers,
        ))
    }
}
