//! PD (Peripheral Device) driver — receives commands from the ACU and
//! emits replies. A real PD also owns the SCS state on the slave side.
//!
//! This is currently a thin scaffold: the dispatcher wires up incoming
//! [`crate::command::Command`] values and delegates response generation to a
//! caller-supplied handler.

use crate::clock::Clock;
use crate::command::{Command, CommandCode};
use crate::error::Error;
use crate::packet::{Address, ControlByte, CtrlFlags, PacketBuilder, ParsedPacket, Sqn};
use crate::reply::Reply;
use crate::transport::Transport;
use alloc::vec::Vec;

/// Trait for PD-side application logic. Each incoming command becomes a
/// method call; the handler returns the reply to emit.
pub trait PdHandler {
    /// Dispatch a fully-decoded command and return the reply payload.
    fn on_command(&mut self, command: &Command) -> Reply;
}

/// PD driver.
pub struct Pd<T: Transport, C: Clock, H: PdHandler> {
    transport: T,
    #[allow(dead_code)]
    clock: C,
    /// PD's own bus address.
    pub address: u8,
    /// Whether to advertise CRC trailers in replies.
    pub use_crc: bool,
    handler: H,
    rx_buf: Vec<u8>,
    /// Last SQN we acted upon.
    last_sqn: Option<u8>,
    /// Last reply we sent (so we can repeat it on a duplicate SQN).
    last_reply: Option<Vec<u8>>,
}

impl<T: Transport, C: Clock, H: PdHandler> Pd<T, C, H> {
    /// New driver.
    pub fn new(transport: T, clock: C, address: u8, handler: H) -> Self {
        Self {
            transport,
            clock,
            address,
            use_crc: true,
            handler,
            rx_buf: Vec::with_capacity(crate::MAX_BUS_PACKET),
            last_sqn: None,
            last_reply: None,
        }
    }

    /// Borrow the underlying transport.
    pub fn transport(&mut self) -> &mut T {
        &mut self.transport
    }

    /// Drain whatever bytes the transport has, dispatch the next complete
    /// command, and reply on the wire if there is one.
    ///
    /// Returns `Ok(true)` if a packet was processed, `Ok(false)` if more
    /// bytes are required.
    pub fn poll_once(&mut self) -> Result<bool, Error> {
        let mut tmp = [0u8; 64];
        let n = self.transport.read(&mut tmp)?;
        if n > 0 {
            self.rx_buf.extend_from_slice(&tmp[..n]);
        }

        while let Some(som_pos) = self.rx_buf.iter().position(|&b| b == crate::SOM) {
            self.rx_buf.drain(..som_pos);
            match ParsedPacket::parse(&self.rx_buf) {
                Ok((parsed, used)) => {
                    let pd_addr = parsed.addr.pd_addr();
                    if pd_addr != self.address && !parsed.addr.is_broadcast() {
                        // Not for us — drop and resume.
                        self.rx_buf.drain(..used);
                        continue;
                    }
                    let code = CommandCode::from_byte(parsed.code)?;
                    let sqn = parsed.ctrl.sqn.value();
                    let cmd_data = parsed.data.to_vec();
                    let used_len = used;
                    self.rx_buf.drain(..used_len);

                    if Some(sqn) == self.last_sqn {
                        // ACU is asking for a reply repeat.
                        if let Some(reply) = &self.last_reply {
                            let r = reply.clone();
                            self.transport.write_all(&r)?;
                        }
                        return Ok(true);
                    }

                    let command = Command::decode(code, &cmd_data)?;
                    let reply = self.handler.on_command(&command);
                    let bytes = self.encode_reply(sqn, &reply)?;
                    self.transport.write_all(&bytes)?;
                    self.last_sqn = Some(sqn);
                    self.last_reply = Some(bytes);
                    let _ = self.clock.now_ms();
                    return Ok(true);
                }
                Err(Error::Truncated { .. }) => return Ok(false),
                Err(Error::BadSom(_)) => {
                    self.rx_buf.remove(0);
                    continue;
                }
                Err(other) => return Err(other),
            }
        }

        Ok(false)
    }

    fn encode_reply(&self, sqn: u8, reply: &Reply) -> Result<Vec<u8>, Error> {
        let addr = Address::reply(self.address)?;
        let ctrl = ControlByte::new(
            Sqn::new(sqn)?,
            if self.use_crc { CtrlFlags::USE_CRC } else { CtrlFlags::empty() },
        );
        let data = reply.encode_data()?;
        PacketBuilder::plain(addr, ctrl, reply.code().as_byte(), data).encode()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::clock::MockClock;
    use crate::command::Command;
    use crate::reply::{Ack, Reply};
    use crate::transport::VecTransport;

    struct AlwaysAck;
    impl PdHandler for AlwaysAck {
        fn on_command(&mut self, _command: &Command) -> Reply {
            Reply::Ack(Ack)
        }
    }

    #[test]
    fn dispatches_poll_to_ack() {
        let clock = MockClock::new();
        let mut transport = VecTransport::new();
        // Encode a POLL ourselves and feed it to the PD's incoming queue.
        let bytes = PacketBuilder::plain(
            Address::pd(0x05).unwrap(),
            ControlByte::new(Sqn::new(1).unwrap(), CtrlFlags::USE_CRC),
            CommandCode::Poll.as_byte(),
            Vec::new(),
        )
        .encode()
        .unwrap();
        transport.feed(&bytes);
        let mut pd = Pd::new(transport, clock, 0x05, AlwaysAck);
        assert!(pd.poll_once().unwrap());
        let reply_bytes: Vec<u8> = pd.transport().outgoing.drain(..).collect();
        let (parsed, _) = ParsedPacket::parse(&reply_bytes).unwrap();
        assert_eq!(parsed.code, 0x40);
        assert!(parsed.addr.is_reply());
    }
}
