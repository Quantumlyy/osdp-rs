//! Helpers shared between integration tests in `tests/`.
//!
//! Cargo's "tests/common/mod.rs" convention keeps this from being compiled as
//! its own integration-test binary. Each test that needs a helper does
//! `mod common;` (and `#[allow(dead_code)]` to silence warnings about helpers
//! that one specific test happens not to use).

#![allow(dead_code)]

use osdp::command::Command;
use osdp::driver::pd::PdHandler;
use osdp::reply::{Ack, Reply};

/// PD handler that answers every command with `osdp_ACK`.
pub struct AlwaysAck;

impl PdHandler for AlwaysAck {
    fn on_command(&mut self, _command: &Command) -> Reply {
        Reply::Ack(Ack)
    }
}

/// Tiny LCG used by chaos-bus tests so we don't need a real RNG crate.
#[derive(Debug, Clone)]
pub struct Lcg(u64);

impl Lcg {
    pub fn new(seed: u64) -> Self {
        Self(seed | 1)
    }

    pub fn next_u32(&mut self) -> u32 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        (self.0 >> 32) as u32
    }

    /// Uniform draw in `0.0..=1.0`.
    pub fn next_unit(&mut self) -> f64 {
        (self.next_u32() as f64) / (u32::MAX as f64)
    }
}
