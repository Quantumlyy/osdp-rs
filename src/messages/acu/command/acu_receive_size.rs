use crate::{
    messages::command::{CommandType, OSDPCommand},
    models::command::keep_reader_active::KeepReaderActive,
};

impl OSDPCommand for KeepReaderActive {
    fn cmnd(&self) -> CommandType {
        CommandType::KeepActive
    }

    fn build_command_data(&self) -> Vec<u8> {
        self.keep_active_time.to_le_bytes().to_vec()
    }
}
