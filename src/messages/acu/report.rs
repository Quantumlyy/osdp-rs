use crate::{
    messages::command::{CommandType, OSDPCommand},
    models::command::report::{IDReport, LocalStatusReport, InputStatusReport},
};

impl OSDPCommand for IDReport {
    fn cmnd(&self) -> CommandType {
        CommandType::IdReport
    }

    fn build_command_data(&self) -> Vec<u8> {
        vec![
            0x00
        ]
    }
}

impl OSDPCommand for LocalStatusReport {
    fn cmnd(&self) -> CommandType {
        CommandType::LocalStatus
    }

    fn build_command_data(&self) -> Vec<u8> {
        vec![
        ]
    }
}

impl OSDPCommand for InputStatusReport {
    fn cmnd(&self) -> CommandType {
        CommandType::LocalStatus
    }

    fn build_command_data(&self) -> Vec<u8> {
        vec![
        ]
    }
}
