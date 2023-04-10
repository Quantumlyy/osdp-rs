/// `osdp_CAP`
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct PeripheralDeviceCapabilities {}

impl PeripheralDeviceCapabilities {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for PeripheralDeviceCapabilities {
    fn default() -> Self {
        Self {}
    }
}
