/// `osdp_ACK`
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct GeneralAcknowledge {}

impl GeneralAcknowledge {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for GeneralAcknowledge {
    fn default() -> Self {
        Self {}
    }
}