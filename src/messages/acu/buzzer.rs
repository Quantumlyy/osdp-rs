use crate::{
    messages::command::{CommandType, OSDPCommand},
    models::command::buzzer::ReaderBuzzerControl,
};

impl OSDPCommand for ReaderBuzzerControl<'_> {
    fn cmnd(&self) -> CommandType {
        CommandType::BuzzerControl
    }

    fn build_command_data(&self) -> Vec<u8> {
        self.buzzer_states
            .iter()
            .map(|bcs| {
                [
                    bcs.reader_number,
                    bcs.tone_code as u8,
                    bcs.on_time,
                    bcs.off_time,
                    bcs.count,
                ]
            })
            .flatten()
            .collect::<Vec<u8>>()
    }
}
