//! Loopback example: an in-memory `VecTransport` carries POLLs from a
//! synthetic ACU to a synthetic PD that always responds with `osdp_ACK`.
//!
//! Run with:
//!
//! ```sh
//! cargo run --example loopback_poll
//! ```

use osdp::clock::SystemClock;
use osdp::command::{Command, Poll};
use osdp::driver::acu::{Acu, PdState};
use osdp::driver::pd::{Pd, PdHandler};
use osdp::reply::{Ack, Reply};
use osdp::transport::VecTransport;

struct AlwaysAck;
impl PdHandler for AlwaysAck {
    fn on_command(&mut self, _command: &Command) -> Reply {
        Reply::Ack(Ack)
    }
}

fn shuffle(from: &mut VecTransport, to: &mut VecTransport) {
    let bytes: Vec<u8> = from.outgoing.drain(..).collect();
    to.incoming.extend(bytes);
}

fn main() {
    let mut acu = Acu::new(VecTransport::new(), SystemClock::new());
    let mut pd = Pd::new(VecTransport::new(), SystemClock::new(), 0x05, AlwaysAck);
    let mut state = PdState::default();
    state.mark_seen(0); // pretend we discovered it on boot

    println!("loopback example: 5 POLL exchanges through a VecTransport bridge");
    for round in 1..=5u32 {
        // ACU writes POLL bytes to its outgoing queue.
        let bytes = acu
            .send_to(0x05, &mut state, &Command::Poll(Poll))
            .expect("send");
        println!(
            "  [{round}] ACU wrote {} bytes (sqn={})",
            bytes.len(),
            bytes[4] & 0x03
        );

        // Ferry ACU outgoing → PD incoming.
        shuffle(acu.transport(), pd.transport());

        // PD processes the packet and queues its reply.
        pd.poll_once().expect("pd poll");

        // Ferry PD outgoing → ACU incoming.
        shuffle(pd.transport(), acu.transport());

        // ACU reads its reply.
        match acu.receive(&mut state) {
            Ok(Reply::Ack(_)) => println!(
                "  [{round}] ACU got ACK; sqn now {}",
                state.next_sqn.value()
            ),
            other => println!("  [{round}] unexpected reply {other:?}"),
        }
    }
}
