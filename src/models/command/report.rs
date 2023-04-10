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

/// `osdp_ISTAT`
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct InputStatusReport {}

impl InputStatusReport {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for InputStatusReport {
    fn default() -> Self {
        Self {}
    }
}

/// `osdp_OSTAT`
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct OutputStatusReport {}

impl OutputStatusReport {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for OutputStatusReport {
    fn default() -> Self {
        Self {}
    }
}

/// `osdp_RSTAT`
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct ReaderStatusReport {}

impl ReaderStatusReport {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for ReaderStatusReport {
    fn default() -> Self {
        Self {}
    }
}
