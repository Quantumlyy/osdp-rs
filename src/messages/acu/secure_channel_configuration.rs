use crate::{
    messages::command::{CommandType, OSDPCommand},
    models::command::secure_channel_configuration::EncryptionKeyConfiguration,
};

impl OSDPCommand for EncryptionKeyConfiguration<'_> {
    fn cmnd(&self) -> CommandType {
        CommandType::KeySet
    }

    fn build_command_data(&self) -> Vec<u8> {
        let mut data = vec![self.key_type as u8, ((self.key_data.len() + 7) / 8) as u8];
        data.extend_from_slice(self.key_data);

        data
    }
}
