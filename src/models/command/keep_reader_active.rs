/// `osdp_KEEPACTIVE`
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct KeepReaderActive {
    pub keep_active_time: u16,
}

impl KeepReaderActive {
    pub fn new(keep_active_time: u16) -> Self {
        Self {keep_active_time}
    }
}
