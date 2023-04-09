use crate::{
    messages::command::{CommandType, OSDPCommand},
    models::command::biometric::BiometricRead,
};

impl OSDPCommand<4> for BiometricRead {
    fn cmnd(&self) -> CommandType {
        CommandType::BioRead
    }

    fn build_command_data(&self) -> [u8; 4] {
        [
            self.reader,
            self.biometric_type as u8,
            self.biometric_format as u8,
            self.biometric_quality,
        ]
    }
}
