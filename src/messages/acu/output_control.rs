use crate::{
    messages::command::{CommandType, OSDPCommand},
    models::command::output_control::OutputControl,
};

impl OSDPCommand for OutputControl<'_> {
    fn cmnd(&self) -> CommandType {
        CommandType::OutputControl
    }

    fn build_command_data(&self) -> Vec<u8> {
        self.output_states
            .iter()
            .map(|os| {
                let timer_bytes = os.timer.to_be_bytes();
                [
                    os.output_number,
                    os.control_code as u8,
                    timer_bytes[0],
                    timer_bytes[1],
                ]
            })
            .flatten()
            .collect::<Vec<u8>>()
    }
}
