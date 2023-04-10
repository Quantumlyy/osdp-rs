use crate::{
    messages::command::{CommandType, OSDPCommand},
    models::command::poll::Poll,
};

impl OSDPCommand for Poll {
    fn cmnd(&self) -> CommandType {
        CommandType::Poll
    }

    fn build_command_data(&self) -> Vec<u8> {
        vec![]
    }
}
