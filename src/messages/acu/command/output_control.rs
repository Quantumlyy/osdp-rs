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
            .flat_map(|os| {
                let timer_bytes = os.timer.to_le_bytes();
                [
                    os.output_number,
                    os.control_code as u8,
                    timer_bytes[0],
                    timer_bytes[1],
                ]
            })
            .collect::<Vec<u8>>()
    }
}
