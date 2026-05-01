//! Time abstraction. Drivers take a [`Clock`] so that timing logic is
//! testable from `no_std` targets without pulling in a runtime.

/// Minimal monotonic clock.
///
/// Implementations must be monotonic: `now_ms()` never goes backwards within a
/// single instance.
pub trait Clock {
    /// Current time as milliseconds since an arbitrary epoch.
    fn now_ms(&self) -> u64;
}

/// `Clock` that returns the system monotonic time. Available with `std`.
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub struct SystemClock {
    epoch: std::time::Instant,
}

#[cfg(feature = "std")]
impl SystemClock {
    /// New clock anchored at the current instant.
    pub fn new() -> Self {
        Self {
            epoch: std::time::Instant::now(),
        }
    }
}

#[cfg(feature = "std")]
impl Default for SystemClock {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "std")]
impl Clock for SystemClock {
    fn now_ms(&self) -> u64 {
        self.epoch.elapsed().as_millis() as u64
    }
}

/// Manually-driven `Clock` for tests.
#[derive(Debug, Default, Clone)]
pub struct MockClock {
    /// Current time in milliseconds.
    pub now: core::cell::Cell<u64>,
}

impl MockClock {
    /// New clock at `t = 0`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Advance the clock by `ms` milliseconds.
    pub fn advance(&self, ms: u64) {
        self.now.set(self.now.get() + ms);
    }

    /// Set the clock to `ms` milliseconds.
    pub fn set(&self, ms: u64) {
        self.now.set(ms);
    }
}

impl Clock for MockClock {
    fn now_ms(&self) -> u64 {
        self.now.get()
    }
}

impl<C: Clock + ?Sized> Clock for &C {
    fn now_ms(&self) -> u64 {
        (*self).now_ms()
    }
}
