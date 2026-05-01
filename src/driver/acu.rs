//! ACU (Access Control Unit) driver — issues commands to PDs and consumes
//! their replies. Manages SQN cycling, REPLY_DELAY enforcement, retry, and
//! off-line declaration.
//!
//! # Spec: §5.7

use crate::clock::Clock;
use crate::command::Command;
use crate::error::Error;
use crate::packet::{Address, ControlByte, CtrlFlags, PacketBuilder, ParsedPacket, Sqn};
use crate::reply::{Reply, ReplyCode};
use crate::transport::Transport;
use alloc::vec::Vec;

/// Per-PD bookkeeping.
#[derive(Debug, Clone)]
pub struct PdState {
    /// Next SQN to send.
    pub next_sqn: Sqn,
    /// Whether the PD prefers CRC trailers.
    pub use_crc: bool,
    /// Last successful exchange (ms, from the [`Clock`]).
    pub last_seen_ms: u64,
}

impl Default for PdState {
    fn default() -> Self {
        Self {
            next_sqn: Sqn::ZERO,
            use_crc: true,
            last_seen_ms: 0,
        }
    }
}

impl PdState {
    /// Advance SQN.
    pub fn bump_sqn(&mut self) {
        self.next_sqn = self.next_sqn.next();
    }

    /// Has the PD been silent for longer than the off-line threshold?
    pub fn is_offline(&self, now_ms: u64) -> bool {
        now_ms.saturating_sub(self.last_seen_ms) >= crate::OFFLINE_THRESHOLD_MS as u64
    }
}

/// ACU driver.
pub struct Acu<T: Transport, C: Clock> {
    /// Underlying transport.
    transport: T,
    /// Time source.
    clock: C,
    /// Reply-delay budget in milliseconds.
    pub reply_delay_ms: u32,
    /// Receive scratch buffer.
    rx_buf: Vec<u8>,
}

impl<T: Transport, C: Clock> Acu<T, C> {
    /// New driver.
    pub fn new(transport: T, clock: C) -> Self {
        Self {
            transport,
            clock,
            reply_delay_ms: crate::REPLY_DELAY_MS,
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

    /// Encode and send `command` to `pd_addr`. Returns the bytes written.
    pub fn send_to(
        &mut self,
        pd_addr: u8,
        pd: &mut PdState,
        command: &Command,
    ) -> Result<Vec<u8>, Error> {
        let addr = Address::pd(pd_addr)?;
        let mut flags = CtrlFlags::empty();
        if pd.use_crc {
            flags |= CtrlFlags::USE_CRC;
        }
        let ctrl = ControlByte::new(pd.next_sqn, flags);
        let data = command.encode_data()?;
        let bytes = PacketBuilder::plain(addr, ctrl, command.code().as_byte(), data).encode()?;
        self.transport.write_all(&bytes)?;
        Ok(bytes)
    }

    /// Block until at least one full packet is available, decode it, and
    /// return both the typed reply and the raw [`ParsedPacket`] metadata.
    ///
    /// The deadline is `start_ms + reply_delay_ms`.
    pub fn receive(&mut self, pd: &mut PdState) -> Result<Reply, Error> {
        let start = self.clock.now_ms();
        loop {
            let len = self.try_parse_packet()?;
            if let Some((reply_code, data)) = len {
                pd.last_seen_ms = self.clock.now_ms();
                pd.bump_sqn();
                return Reply::decode(reply_code, &data);
            }
            if self.clock.now_ms() - start >= self.reply_delay_ms as u64 {
                return Err(Error::Timeout);
            }
            // Drain a chunk; transport returns 0 if no data is ready.
            let mut tmp = [0u8; 64];
            let n = self.transport.read(&mut tmp)?;
            if n > 0 {
                self.rx_buf.extend_from_slice(&tmp[..n]);
            }
        }
    }

    fn try_parse_packet(&mut self) -> Result<Option<(ReplyCode, Vec<u8>)>, Error> {
        // Find SOM
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
        // CTRL byte: SQN=0, USE_CRC set
        assert_eq!(bytes[4] & 0x0F, 0x04);
    }
}
