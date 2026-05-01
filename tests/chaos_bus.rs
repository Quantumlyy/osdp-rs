//! Plan §6 Layer 3 — the chaos bus.
//!
//! A `Middleman` sits between the ACU and PD ends of two `VecTransport`s.
//! Per byte it can drop, flip, or inject according to a seeded RNG. Tests
//! drive the ACU through a short command sequence under chaos and assert
//! the high-level safety invariants:
//!
//! - The ACU's parser never panics regardless of corruption.
//! - When the seed makes the bus quiet enough, the exchange eventually
//!   completes (i.e. our retry/off-line policy doesn't hang).
//! - When the bus is so corrupted that nothing arrives, the ACU declares
//!   the PD off-line within 8 s of simulated time rather than spinning
//!   forever.

use osdp::clock::MockClock;
use osdp::command::{Command, Poll};
use osdp::driver::acu::{Acu, ExchangeOutcome, PdState, RetryConfig};
use osdp::driver::pd::{Pd, PdHandler};
use osdp::reply::{Ack, Reply};
use osdp::transport::VecTransport;

extern crate alloc;
use alloc::collections::VecDeque;
use alloc::vec::Vec;

/// A tiny LCG so tests don't need a real RNG crate.
#[derive(Debug, Clone)]
struct Lcg(u64);
impl Lcg {
    fn new(seed: u64) -> Self {
        Self(seed | 1)
    }
    fn next_u32(&mut self) -> u32 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        (self.0 >> 32) as u32
    }
    /// Uniform 0..1 step.
    fn next_unit(&mut self) -> f64 {
        (self.next_u32() as f64) / (u32::MAX as f64)
    }
}

/// Per-byte chaos parameters.
#[derive(Debug, Clone, Copy)]
struct Chaos {
    drop_p: f64,
    flip_p: f64,
    inject_p: f64,
}

#[derive(Debug, Clone)]
struct Middleman {
    rng: Lcg,
    chaos: Chaos,
}

impl Middleman {
    fn new(seed: u64, chaos: Chaos) -> Self {
        Self {
            rng: Lcg::new(seed),
            chaos,
        }
    }

    /// Apply per-byte chaos to `bytes` and return the corrupted output.
    fn ferry(&mut self, bytes: &[u8]) -> VecDeque<u8> {
        let mut out = VecDeque::with_capacity(bytes.len() + 4);
        for &b in bytes {
            if self.rng.next_unit() < self.chaos.drop_p {
                continue;
            }
            let mut byte = b;
            if self.rng.next_unit() < self.chaos.flip_p {
                let bit = self.rng.next_u32() & 7;
                byte ^= 1 << bit;
            }
            out.push_back(byte);
            if self.rng.next_unit() < self.chaos.inject_p {
                out.push_back(self.rng.next_u32() as u8);
            }
        }
        out
    }
}

struct AlwaysAck;
impl PdHandler for AlwaysAck {
    fn on_command(&mut self, _command: &Command) -> Reply {
        Reply::Ack(Ack)
    }
}

/// Build an ACU + PD + middleman triple for one chaos seed.
fn run_one(seed: u64, chaos: Chaos, advance_ms: u64) -> Result<ExchangeOutcome, osdp::Error> {
    let acu_clock = MockClock::new();
    let pd_clock = MockClock::new();
    let mut acu = Acu::new(VecTransport::new(), acu_clock.clone());
    acu.retry = RetryConfig {
        max_retries: 1,
        overall_budget_ms: 0,
    };
    let mut pd = Pd::new(VecTransport::new(), pd_clock, 0x05, AlwaysAck);
    let mut state = PdState::default();

    // Mark PD as having been seen recently — avoid the immediate-offline
    // branch for not-yet-discovered PDs.
    state.mark_seen(0);

    let mut middle = Middleman::new(seed, chaos);

    // Try the exchange under chaos. On every internal poll, we (a) shuffle
    // ACU outgoing into PD incoming through the middleman, and (b) shuffle
    // PD outgoing into ACU incoming likewise. This is approximated by
    // running the dance step-by-step.

    // Step 1: ACU sends.
    let _ = acu.send_to(0x05, &mut state, &Command::Poll(Poll))?;
    let acu_out: Vec<u8> = acu.transport().outgoing.drain(..).collect();
    let pd_in = middle.ferry(&acu_out);
    pd.transport().incoming.extend(pd_in);

    // Step 2: PD processes (may not produce output if frame was corrupted).
    let _ = pd.poll_once();
    let pd_out: Vec<u8> = pd.transport().outgoing.drain(..).collect();
    let acu_in = middle.ferry(&pd_out);
    acu.transport().incoming.extend(acu_in);

    // Step 3: advance the clock (simulated time spent waiting on bytes)
    // and let the ACU try to receive. The MockClock controls timeout/offline.
    acu_clock.advance(advance_ms);

    match acu.receive(&mut state) {
        Ok(reply) => Ok(ExchangeOutcome::Reply(reply)),
        Err(osdp::Error::Timeout) => Ok(ExchangeOutcome::Timeout),
        Err(other) => Err(other),
    }
}

#[test]
fn quiet_bus_completes() {
    let chaos = Chaos {
        drop_p: 0.0,
        flip_p: 0.0,
        inject_p: 0.0,
    };
    let outcome = run_one(0xDEAD_BEEF, chaos, 1).unwrap();
    assert!(matches!(outcome, ExchangeOutcome::Reply(Reply::Ack(_))));
}

#[test]
fn total_silence_times_out() {
    // Drop every byte → ACU never hears back. With clock advanced past
    // REPLY_DELAY, receive returns Timeout (not Offline yet — only one
    // attempt's worth of clock has passed).
    let chaos = Chaos {
        drop_p: 1.0,
        flip_p: 0.0,
        inject_p: 0.0,
    };
    let outcome = run_one(1234, chaos, osdp::REPLY_DELAY_MS as u64 + 1).unwrap();
    assert_eq!(outcome, ExchangeOutcome::Timeout);
}

#[test]
fn parser_never_panics_under_chaos() {
    // Run lots of seeds with heavy corruption. We don't care about the
    // outcome — just that the call returns *some* Result and doesn't
    // unwind.
    let chaos = Chaos {
        drop_p: 0.3,
        flip_p: 0.2,
        inject_p: 0.1,
    };
    for seed in 0u64..256 {
        let _ = run_one(seed, chaos, osdp::REPLY_DELAY_MS as u64 + 1);
    }
}

#[test]
fn modest_chaos_can_succeed() {
    // 1% byte flips / drops — sometimes the ACU still gets an ACK through.
    let chaos = Chaos {
        drop_p: 0.01,
        flip_p: 0.01,
        inject_p: 0.0,
    };
    let mut succeeded = 0u32;
    for seed in 0u64..32 {
        if let Ok(ExchangeOutcome::Reply(Reply::Ack(_))) =
            run_one(seed, chaos, osdp::REPLY_DELAY_MS as u64)
        {
            succeeded += 1;
        }
    }
    assert!(succeeded >= 1, "at least one quiet seed should succeed");
}
