use crate::{
    messages::command::{CommandType, OSDPCommand},
    models::command::id_report::IDReport,
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
