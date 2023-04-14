/// `osdp_ABORT`
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Abort {}

impl Abort {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Abort {
    fn default() -> Self {
        Self {}
    }
}
