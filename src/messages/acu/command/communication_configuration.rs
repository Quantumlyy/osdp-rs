use crate::{
    messages::command::{CommandType, OSDPCommand},
    models::command::communication_configuration::CommunicationConfiguration,
};

impl OSDPCommand for CommunicationConfiguration {
    fn cmnd(&self) -> CommandType {
        CommandType::CommunicationSet
    }

    fn build_command_data(&self) -> Vec<u8> {
        let baud_rate_bytes: [u8; 4] = self.baud_rate.to_le_bytes();

        vec![
            self.address,
            baud_rate_bytes[0],
            baud_rate_bytes[1],
            baud_rate_bytes[2],
            baud_rate_bytes[3],
        ]
    }
}
