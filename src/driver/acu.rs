//! ACU (Access Control Unit) driver — issues commands to PDs and consumes
//! their replies. Manages SQN cycling, REPLY_DELAY enforcement, retry,
//! `osdp_BUSY` handling, and off-line detection.
//!
//! # Spec: §5.7

use crate::clock::Clock;
use crate::command::Command;
use crate::error::Error;
use crate::packet::{Address, ControlByte, CtrlFlags, PacketBuilder, ParsedPacket, Sqn};
use crate::reply::{Reply, ReplyCode};
use crate::transport::Transport;
use alloc::vec::Vec;

/// Per-PD bookkeeping owned by the ACU driver.
#[derive(Debug, Clone)]
pub struct PdState {
    /// Next SQN to send.
    pub next_sqn: Sqn,
    /// Whether the PD prefers CRC trailers.
    pub use_crc: bool,
    /// Last successful exchange (ms, from the [`Clock`]).
    pub last_seen_ms: u64,
    /// Strictly true once `last_seen_ms` has been set at least once.
    seen_at_least_once: bool,
}

impl Default for PdState {
    fn default() -> Self {
        Self {
            next_sqn: Sqn::ZERO,
            use_crc: true,
            last_seen_ms: 0,
            seen_at_least_once: false,
        }
    }
}

impl PdState {
    /// Advance SQN.
    pub fn bump_sqn(&mut self) {
        self.next_sqn = self.next_sqn.next();
    }

    /// Mark the PD as seen.
    pub fn mark_seen(&mut self, now_ms: u64) {
        self.last_seen_ms = now_ms;
        self.seen_at_least_once = true;
    }

    /// `true` once we've ever heard from the PD and the last exchange is
    /// older than [`crate::OFFLINE_THRESHOLD_MS`]. Returns `false` while we
    /// are still in the initial-connect window — call [`mark_offline`] if
    /// you want to force-declare a brand-new PD off-line.
    pub fn is_offline(&self, now_ms: u64) -> bool {
        self.seen_at_least_once
            && now_ms.saturating_sub(self.last_seen_ms) >= crate::OFFLINE_THRESHOLD_MS as u64
    }

    /// Reset to the freshly-booted state. Call this when an off-line PD
    /// should be re-discovered from scratch.
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

/// Outcome of a single command/reply exchange.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ExchangeOutcome {
    /// PD answered with a typed reply.
    Reply(Reply),
    /// PD answered with `osdp_BUSY`. Caller may try again later.
    Busy,
    /// No reply within the configured budget after exhausting retries.
    Timeout,
    /// PD has been silent for ≥ [`crate::OFFLINE_THRESHOLD_MS`].
    Offline,
}

/// Configuration for retry policy.
#[derive(Debug, Clone, Copy)]
pub struct RetryConfig {
    /// Number of *additional* attempts after the first (so `0` = no retry).
    pub max_retries: u8,
    /// Optional cap on how long to spend retrying a single command (ms). `0`
    /// disables the cap; the default REPLY_DELAY budget per attempt still
    /// applies.
    pub overall_budget_ms: u32,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 2,
            overall_budget_ms: 0,
        }
    }
}

/// ACU driver.
pub struct Acu<T: Transport, C: Clock> {
    transport: T,
    clock: C,
    /// Reply-delay budget per attempt, in milliseconds.
    pub reply_delay_ms: u32,
    /// Retry policy applied by [`Acu::exchange`].
    pub retry: RetryConfig,
    rx_buf: Vec<u8>,
}

impl<T: Transport, C: Clock> Acu<T, C> {
    /// New driver with default reply delay and retry policy.
    pub fn new(transport: T, clock: C) -> Self {
        Self {
            transport,
            clock,
            reply_delay_ms: crate::REPLY_DELAY_MS,
            retry: RetryConfig::default(),
            rx_buf: Vec::with_capacity(crate::MAX_BUS_PACKET),
        }
    }

    /// Borrow the underlying transport.
    pub fn transport(&mut self) -> &mut T {
        &mut self.transport
    }

    /// Borrow the clock.
    pub fn clock(&self) -> &C {
        &self.clock
    }

    /// Encode and send `command` to `pd_addr` once. Returns the bytes written.
    pub fn send_to(
        &mut self,
        pd_addr: u8,
        pd: &mut PdState,
        command: &Command,
    ) -> Result<Vec<u8>, Error> {
        self.send_with_sqn(pd_addr, pd.use_crc, pd.next_sqn, command)
    }

    fn send_with_sqn(
        &mut self,
        pd_addr: u8,
        use_crc: bool,
        sqn: Sqn,
        command: &Command,
    ) -> Result<Vec<u8>, Error> {
        let addr = Address::pd(pd_addr)?;
        let mut flags = CtrlFlags::empty();
        if use_crc {
            flags |= CtrlFlags::USE_CRC;
        }
        let ctrl = ControlByte::new(sqn, flags);
        let data = command.encode_data()?;
        let bytes = PacketBuilder::plain(addr, ctrl, command.code().as_byte(), data).encode()?;
        self.transport.write_all(&bytes)?;
        Ok(bytes)
    }

    /// Drain whatever bytes the transport has, then attempt to parse one
    /// reply. Returns `Err(Error::Timeout)` once the per-attempt
    /// reply-delay budget elapses without a complete packet.
    ///
    /// The loop reads up to a small fixed number of times per call to
    /// drain a transport that may return short reads. When the underlying
    /// transport returns `Ok(0)` (no more bytes immediately ready), we
    /// check the deadline and either return `Timeout` or immediately
    /// return — the caller is expected to call us again later.
    pub fn receive(&mut self, pd: &mut PdState) -> Result<Reply, Error> {
        let start = self.clock.now_ms();
        let mut empty_reads = 0u8;
        loop {
            if let Some((reply_code, data)) = self.try_parse_packet()? {
                let now = self.clock.now_ms();
                pd.mark_seen(now);
                pd.bump_sqn();
                return Reply::decode(reply_code, &data);
            }
            let mut tmp = [0u8; 64];
            let n = self.transport.read(&mut tmp)?;
            if n > 0 {
                self.rx_buf.extend_from_slice(&tmp[..n]);
                empty_reads = 0;
                continue;
            }
            if self.clock.now_ms().saturating_sub(start) >= self.reply_delay_ms as u64 {
                return Err(Error::Timeout);
            }
            empty_reads = empty_reads.saturating_add(1);
            // Nonblocking transports signal "no data" via Ok(0); after a few
            // such returns within budget we hand control back to the caller
            // (still as Timeout) rather than spin.
            if empty_reads >= 4 {
                return Err(Error::Timeout);
            }
        }
    }

    /// Run a command/reply round-trip with full retry & off-line policy.
    ///
    /// - On a clean reply, returns [`ExchangeOutcome::Reply`].
    /// - On `osdp_BUSY`, returns [`ExchangeOutcome::Busy`] without bumping SQN
    ///   (per Annex A.2: BUSY's SQN is always 0). The caller decides whether
    ///   to retry now or service other PDs first.
    /// - On timeout, retries up to [`RetryConfig::max_retries`] additional
    ///   times *re-using the same SQN* — that asks the PD to repeat its
    ///   prior reply, per §5.7 / Table 2.
    /// - When the PD has been silent for ≥ [`crate::OFFLINE_THRESHOLD_MS`],
    ///   returns [`ExchangeOutcome::Offline`].
    pub fn exchange(
        &mut self,
        pd_addr: u8,
        pd: &mut PdState,
        command: &Command,
    ) -> Result<ExchangeOutcome, Error> {
        let now = self.clock.now_ms();
        if pd.is_offline(now) {
            return Ok(ExchangeOutcome::Offline);
        }

        let started = now;
        let sqn = pd.next_sqn;
        let mut attempts: u8 = 0;
        let max = self.retry.max_retries;
        let budget = self.retry.overall_budget_ms;

        loop {
            self.send_with_sqn(pd_addr, pd.use_crc, sqn, command)?;
            match self.recv_one_with_sqn(sqn) {
                Ok(reply) => {
                    let now = self.clock.now_ms();
                    pd.mark_seen(now);
                    if matches!(reply, Reply::Busy(_)) {
                        // BUSY does not advance the SQN.
                        return Ok(ExchangeOutcome::Busy);
                    }
                    pd.bump_sqn();
                    return Ok(ExchangeOutcome::Reply(reply));
                }
                Err(Error::Timeout) => {
                    attempts += 1;
                    let now = self.clock.now_ms();
                    if pd.is_offline(now) {
                        return Ok(ExchangeOutcome::Offline);
                    }
                    if attempts > max {
                        return Ok(ExchangeOutcome::Timeout);
                    }
                    if budget != 0 && now.saturating_sub(started) >= budget as u64 {
                        return Ok(ExchangeOutcome::Timeout);
                    }
                    // Re-loop with the SAME SQN to ask for a reply repeat.
                    continue;
                }
                Err(other) => return Err(other),
            }
        }
    }

    /// Same as [`Self::receive`] but without mutating any [`PdState`] (used
    /// inside [`Self::exchange`] which has its own bookkeeping).
    fn recv_one_with_sqn(&mut self, _expected_sqn: Sqn) -> Result<Reply, Error> {
        let start = self.clock.now_ms();
        let mut empty_reads = 0u8;
        loop {
            if let Some((reply_code, data)) = self.try_parse_packet()? {
                return Reply::decode(reply_code, &data);
            }
            let mut tmp = [0u8; 64];
            let n = self.transport.read(&mut tmp)?;
            if n > 0 {
                self.rx_buf.extend_from_slice(&tmp[..n]);
                empty_reads = 0;
                continue;
            }
            if self.clock.now_ms().saturating_sub(start) >= self.reply_delay_ms as u64 {
                return Err(Error::Timeout);
            }
            empty_reads = empty_reads.saturating_add(1);
            if empty_reads >= 4 {
                return Err(Error::Timeout);
            }
        }
    }

    fn try_parse_packet(&mut self) -> Result<Option<(ReplyCode, Vec<u8>)>, Error> {
        while let Some(som_pos) = self.rx_buf.iter().position(|&b| b == crate::SOM) {
            self.rx_buf.drain(..som_pos);
            match ParsedPacket::parse(&self.rx_buf) {
                Ok((parsed, used)) => {
                    let code = ReplyCode::from_byte(parsed.code)?;
                    let data = parsed.data.to_vec();
                    self.rx_buf.drain(..used);
                    return Ok(Some((code, data)));
                }
                Err(Error::Truncated { .. }) => return Ok(None),
                Err(Error::BadSom(_)) => {
                    self.rx_buf.remove(0);
                    continue;
                }
                // CRC/checksum failures: skip just past the SOM and resync.
                Err(Error::BadCrc { .. }) | Err(Error::BadChecksum { .. }) => {
                    self.rx_buf.remove(0);
                    continue;
                }
                Err(other) => return Err(other),
            }
        }
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::clock::MockClock;
    use crate::command::Poll;
    use crate::transport::VecTransport;

    #[test]
    fn send_poll_emits_correct_bytes() {
        let clock = MockClock::new();
        let transport = VecTransport::new();
        let mut acu = Acu::new(transport, clock);
        let mut pd = PdState::default();
        let bytes = acu
            .send_to(0x05, &mut pd, &Command::Poll(Poll))
            .unwrap();
        assert_eq!(bytes[0], crate::SOM);
        assert_eq!(bytes[1], 0x05);
        assert_eq!(bytes[4] & 0x0F, 0x04); // SQN=0, CRC=on
    }

    #[test]
    fn exchange_offline_when_silent() {
        let clock = MockClock::new();
        let transport = VecTransport::new();
        let mut acu = Acu::new(transport, clock.clone());
        acu.retry = RetryConfig {
            max_retries: 0,
            overall_budget_ms: 0,
        };
        let mut pd = PdState::default();
        // Pretend we'd seen the PD long ago.
        pd.mark_seen(0);
        clock.set(crate::OFFLINE_THRESHOLD_MS as u64 + 1);
        let outcome = acu
            .exchange(0x05, &mut pd, &Command::Poll(Poll))
            .unwrap();
        assert_eq!(outcome, ExchangeOutcome::Offline);
    }

    #[test]
    fn timeout_then_no_more_retries_returns_timeout() {
        let clock = MockClock::new();
        let transport = VecTransport::new();
        let mut acu = Acu::new(transport, clock.clone());
        acu.retry = RetryConfig {
            max_retries: 0,
            overall_budget_ms: 0,
        };
        let mut pd = PdState::default();
        pd.mark_seen(0);
        // Advance just enough that we're hitting the per-attempt budget but
        // not yet off-line.
        clock.set(crate::REPLY_DELAY_MS as u64 + 1);
        let outcome = acu
            .exchange(0x05, &mut pd, &Command::Poll(Poll))
            .unwrap();
        assert_eq!(outcome, ExchangeOutcome::Timeout);
    }
}
