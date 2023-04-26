use crate::{
    messages::command::{CommandType, OSDPCommand},
    models::command::abort::Abort,
};

impl OSDPCommand for Abort {
    fn cmnd(&self) -> CommandType {
        CommandType::Abort
    }

    fn build_command_data(&self) -> Vec<u8> {
        vec![]
    }
}
