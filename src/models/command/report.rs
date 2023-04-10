/// `osdp_ID`
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct IDReport {}

impl IDReport {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for IDReport {
    fn default() -> Self {
        Self {}
    }
}

/// `osdp_LSTAT`
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct LocalStatusReport {}

impl LocalStatusReport {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for LocalStatusReport {
    fn default() -> Self {
        Self {}
    }
}
