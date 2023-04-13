/// `osdp_ACURXSIZE`
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ACUReceiveSize {
    pub acu_max_receive_buffer: u16,
}

impl ACUReceiveSize {
    pub fn new(acu_max_receive_buffer: u16) -> Self {
        Self {acu_max_receive_buffer}
    }
}
