/// `osdp_COMSET`
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct CommunicationConfiguration {
    /// Unit ID to which this PD will respond after the change takes effect.
    pub address: u8,
    pub baud_rate: u32,
}

impl CommunicationConfiguration {
    pub fn new(address: u8, baud_rate: u32) -> Self {
        Self { address, baud_rate }
    }
}
