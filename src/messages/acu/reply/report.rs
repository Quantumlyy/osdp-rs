use crate::{
    messages::reply::{OSDPReply, ReplyDeserializationError, ReplyType},
    models::reply::report::{
        DeviceCapabilitiesReport, DeviceCapability, DeviceIdentificationReport, LocalStatusReport,
    },
};

impl OSDPReply for DeviceIdentificationReport {
    fn rply(&self) -> ReplyType {
        ReplyType::PdIdReport
    }

    fn deserialize(data: &[u8]) -> Result<DeviceIdentificationReport, ReplyDeserializationError> {
        let sized_data: &[u8; 12] = match data.try_into() {
            Ok(data) => data,
            Err(_) => {
                return Err(ReplyDeserializationError::InvalidPacketSize {
                    minimum: 12,
                    maximum: 12,
                    received: data.len(),
                })
            }
        };
        let sized_serial_numbers: &[u8; 4] = data[5..8].try_into().unwrap();
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

impl OSDPReply for DeviceCapabilitiesReport {
    fn rply(&self) -> ReplyType {
        ReplyType::PdCapabilitiesReport
    }

    fn deserialize(data: &[u8]) -> Result<DeviceCapabilitiesReport, ReplyDeserializationError> {
        if data.len() % 3 != 0 {
            return Err(ReplyDeserializationError::InvalidDataWindowSize {
                multiple: 3,
                received: data.len(),
            });
        }

        let capabilities = data
            .windows(3)
            .map(|window| DeviceCapability {
                function_code: window[0],
                compliance: window[1],
                number_of: window[2],
            })
            .collect::<Vec<DeviceCapability>>();

        Ok(Self::new(capabilities))
    }
}

impl OSDPReply for LocalStatusReport {
    fn rply(&self) -> ReplyType {
        ReplyType::LocalStatusReport
    }

    fn deserialize(data: &[u8]) -> Result<LocalStatusReport, ReplyDeserializationError> {
        let sized_data: &[u8; 2] = match data.try_into() {
            Ok(data) => data,
            Err(_) => {
                return Err(ReplyDeserializationError::InvalidPacketSize {
                    minimum: 2,
                    maximum: 2,
                    received: data.len(),
                })
            }
        };

        Ok(Self::new(sized_data[0], sized_data[1]))
    }
}
