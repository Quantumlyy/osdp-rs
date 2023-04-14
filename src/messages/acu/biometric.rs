use crate::{
    messages::command::{CommandType, OSDPCommand},
    models::command::biometric::{BiometricMatch, BiometricRead},
};

impl OSDPCommand for BiometricRead {
    fn cmnd(&self) -> CommandType {
        CommandType::BioRead
    }

    fn build_command_data(&self) -> Vec<u8> {
        vec![
            self.reader,
            self.biometric_type as u8,
            self.biometric_format as u8,
            self.biometric_quality,
        ]
    }
}

impl OSDPCommand for BiometricMatch<'_> {
    fn cmnd(&self) -> CommandType {
        CommandType::BioMatch
    }

    fn build_command_data(&self) -> Vec<u8> {
        let mut data = vec![
            self.reader,
            self.biometric_type as u8,
            self.biometric_format as u8,
            self.biometric_quality,
        ];
        data.extend_from_slice(&self.biometric_template_data.len().to_le_bytes());
        data.extend_from_slice(self.biometric_template_data);

        data
    }
}
