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
///
/// Shares its underlying counter across clones via [`alloc::sync::Arc`], so
/// a clone handed to a driver and the original held by a test refer to the
/// *same* time value.
#[cfg(feature = "alloc")]
#[derive(Debug, Default, Clone)]
pub struct MockClock {
    inner: alloc::sync::Arc<core::sync::atomic::AtomicU64>,
}

#[cfg(feature = "alloc")]
impl MockClock {
    /// New clock at `t = 0`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Advance the clock by `ms` milliseconds.
    pub fn advance(&self, ms: u64) {
        self.inner
            .fetch_add(ms, core::sync::atomic::Ordering::Relaxed);
    }

    /// Set the clock to `ms` milliseconds.
    pub fn set(&self, ms: u64) {
        self.inner.store(ms, core::sync::atomic::Ordering::Relaxed);
    }
}

#[cfg(feature = "alloc")]
impl Clock for MockClock {
    fn now_ms(&self) -> u64 {
        self.inner.load(core::sync::atomic::Ordering::Relaxed)
    }
}

impl<C: Clock + ?Sized> Clock for &C {
    fn now_ms(&self) -> u64 {
        (*self).now_ms()
    }
}
