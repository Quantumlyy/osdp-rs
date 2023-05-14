/// `osdp_ABORT`
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub struct Abort {}

impl Abort {
    pub fn new() -> Self {
        Self {}
    }
}
