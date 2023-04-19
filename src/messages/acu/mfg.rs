use crate::{
    messages::command::{CommandType, OSDPCommand},
    models::command::mfg::ManufacturerSpecificCommand,
};

impl OSDPCommand for ManufacturerSpecificCommand<'_> {
    fn cmnd(&self) -> CommandType {
        CommandType::ManufacturerSpecific
    }

    fn build_command_data(&self) -> Vec<u8> {
        let mut data = vec![
            self.vendor_code_1, //
            self.vendor_code_2,
            self.vendor_code_3,
        ];
        data.extend_from_slice(self.data);

        data
    }
}
