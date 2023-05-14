/// `osdp_ID`
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub struct IDReport {}

impl IDReport {
    pub fn new() -> Self {
        Self {}
    }
}

/// `osdp_LSTAT`
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub struct LocalStatusReport {}

impl LocalStatusReport {
    pub fn new() -> Self {
        Self {}
    }
}

/// `osdp_ISTAT`
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub struct InputStatusReport {}

impl InputStatusReport {
    pub fn new() -> Self {
        Self {}
    }
}

/// `osdp_OSTAT`
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub struct OutputStatusReport {}

impl OutputStatusReport {
    pub fn new() -> Self {
        Self {}
    }
}

/// `osdp_RSTAT`
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub struct ReaderStatusReport {}

impl ReaderStatusReport {
    pub fn new() -> Self {
        Self {}
    }
}
