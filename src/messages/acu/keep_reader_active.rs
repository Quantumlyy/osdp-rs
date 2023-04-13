use crate::{
    messages::command::{CommandType, OSDPCommand},
    models::command::acu_receive_size::ACUReceiveSize,
};

impl OSDPCommand for ACUReceiveSize {
    fn cmnd(&self) -> CommandType {
        CommandType::MaxReplySize
    }

    fn build_command_data(&self) -> Vec<u8> {
        self.acu_max_receive_buffer.to_be_bytes().to_vec()
    }
}
