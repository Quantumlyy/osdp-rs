use crate::{
    messages::command::{CommandType, OSDPCommand},
    models::command::led::ReaderLEDControl,
};

impl OSDPCommand for ReaderLEDControl<'_> {
    fn cmnd(&self) -> CommandType {
        CommandType::LEDControl
    }

    fn build_command_data(&self) -> Vec<u8> {
        self.led_states
            .iter()
            .map(|lcs| {
                let temporary_timer_bytes = lcs.temporary_timer.to_le_bytes();

                [
                    lcs.reader_number,
                    lcs.led,

                    lcs.temporary_control_code as u8,
                    lcs.temporary_on_time,
                    lcs.temporary_off_time,
                    lcs.temporary_on_color as u8,
                    lcs.temporary_off_color as u8,
                    temporary_timer_bytes[0],
                    temporary_timer_bytes[1],

                    lcs.permanent_control_code as u8,
                    lcs.permanent_on_time,
                    lcs.permanent_off_time,
                    lcs.permanent_on_color as u8,
                    lcs.permanent_off_color as u8,
                ]
            })
            .flatten()
            .collect::<Vec<u8>>()
    }
}
