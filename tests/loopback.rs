//! End-to-end ACU↔PD loopback: a `VecTransport` queue feeds bytes from the
//! ACU's outgoing into the PD's incoming, and vice versa. We then run the
//! ACU through a small command sequence and assert the PD answers correctly.

use osdp::clock::MockClock;
use osdp::command::{Command, Id, Poll};
use osdp::driver::{
    acu::{Acu, PdState},
    pd::{Pd, PdHandler},
};
use osdp::reply::{Ack, PdId, Reply};
use osdp::transport::VecTransport;

extern crate alloc;
use alloc::vec::Vec;

/// PD that answers POLL with ACK and ID with a fixed PDID block.
struct LoopbackPd;
impl PdHandler for LoopbackPd {
    fn on_command(&mut self, command: &Command) -> Reply {
        match command {
            Command::Poll(_) => Reply::Ack(Ack),
            Command::Id(_) => Reply::PdId(PdId {
                vendor_oui: [0x00, 0x06, 0x8E],
                model: 0x12,
                version: 0x34,
                serial: 0xCAFEBABE,
                firmware: [9, 9, 9],
            }),
            _ => Reply::Ack(Ack),
        }
    }
}

/// Ferry bytes from one end's outgoing to the other end's incoming.
fn shuffle(from: &mut VecTransport, to: &mut VecTransport) {
    let bytes: Vec<u8> = from.outgoing.drain(..).collect();
    to.incoming.extend(bytes);
}

#[test]
fn poll_then_id_round_trip() {
    let acu_port = VecTransport::new();
    let pd_port = VecTransport::new();
    let acu_clock = MockClock::new();
    let pd_clock = MockClock::new();

    let mut acu = Acu::new(acu_port, acu_clock);
    let mut pd = Pd::new(pd_port, pd_clock, 0x05, LoopbackPd);
    let mut state = PdState::default();

    // ACU sends POLL.
    acu.send_to(0x05, &mut state, &Command::Poll(Poll)).unwrap();
    shuffle(acu.transport(), pd.transport());

    // PD processes and replies.
    let processed = pd.poll_once().unwrap();
    assert!(processed);
    shuffle(pd.transport(), acu.transport());

    // ACU receives ACK.
    let reply = acu.receive(&mut state).unwrap();
    assert!(matches!(reply, Reply::Ack(_)));
    assert_eq!(state.next_sqn.value(), 1);

    // ACU now sends ID.
    acu.send_to(0x05, &mut state, &Command::Id(Id::standard()))
        .unwrap();
    shuffle(acu.transport(), pd.transport());
    pd.poll_once().unwrap();
    shuffle(pd.transport(), acu.transport());

    let reply = acu.receive(&mut state).unwrap();
    match reply {
        Reply::PdId(id) => {
            assert_eq!(id.vendor_oui, [0x00, 0x06, 0x8E]);
            assert_eq!(id.serial, 0xCAFEBABE);
        }
        other => panic!("expected PDID, got {other:?}"),
    }
}

#[test]
fn pd_repeats_reply_on_duplicate_sqn() {
    let acu_port = VecTransport::new();
    let pd_port = VecTransport::new();
    let acu_clock = MockClock::new();
    let pd_clock = MockClock::new();

    let mut acu = Acu::new(acu_port, acu_clock);
    let mut pd = Pd::new(pd_port, pd_clock, 0x07, LoopbackPd);
    let mut state = PdState::default();

    // First exchange.
    acu.send_to(0x07, &mut state, &Command::Poll(Poll)).unwrap();
    shuffle(acu.transport(), pd.transport());
    pd.poll_once().unwrap();
    shuffle(pd.transport(), acu.transport());
    let _ = acu.receive(&mut state).unwrap();

    // ACU "loses" the reply and asks again with the same SQN. We force
    // that by rewinding state.next_sqn back to what it was.
    state.next_sqn = osdp::Sqn::new(0).unwrap();

    let bytes_first = acu
        .send_to(0x07, &mut state, &Command::Poll(Poll))
        .unwrap();
    shuffle(acu.transport(), pd.transport());
    pd.poll_once().unwrap();

    let pd_outgoing: Vec<u8> = pd.transport().outgoing.drain(..).collect();
    assert!(!pd_outgoing.is_empty(), "PD should repeat its prior reply");
    assert_eq!(bytes_first[4] & 0x03, 0); // we did request SQN 0
}
