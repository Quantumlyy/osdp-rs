use crate::{
    messages::command::{CommandType, OSDPCommand},
    models::command::peripheral_device_capabilities::PeripheralDeviceCapabilities,
};

impl OSDPCommand for PeripheralDeviceCapabilities {
    fn cmnd(&self) -> CommandType {
        CommandType::DeviceCapabilities
    }

    fn build_command_data(&self) -> Vec<u8> {
        vec![0x00]
    }
}
