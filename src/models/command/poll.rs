/// `osdp_POLL`
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Poll {}

impl Poll {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Poll {
    fn default() -> Self {
        Self {}
    }
}
