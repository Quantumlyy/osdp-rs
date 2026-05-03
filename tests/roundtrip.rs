//! Property-based roundtrip tests for the codec.
//!
//! These hit the high-value invariants from the v0.2 plan §6:
//!
//! - "build → bytes → parse → equal" for every command/reply we generate.
//! - "parser is total — never panics on any byte slice" against random noise.
//! - Length-prefix invariant: the LEN field always equals `bytes.len()`.

use osdp::caps::Capability;
use osdp::command::{
    AcuRxSize, BioFormat, BioMatch, BioRead, BioType, BuzzerControl, BuzzerTone, ComSet, Command,
    Id, KeepActive, KeySet, OutputControl, OutputControlCode,
};
use osdp::packet::{Address, ControlByte, CtrlFlags, PacketBuilder, ParsedPacket, Sqn};
use osdp::reply::{
    Ack, BioMatchR, BioReadR, Busy, CCrypt, Com, FtStat, IStatR, LStatR, Nak, NakErrorCode, OStatR,
    PdCap, PdId, RMacI, RStatR, Raw, Reply,
};
use proptest::prelude::*;

mod arb {
    use super::*;

    pub fn arb_addr() -> impl Strategy<Value = u8> {
        0u8..=0x7Eu8
    }

    pub fn arb_sqn() -> impl Strategy<Value = u8> {
        0u8..=3u8
    }

    pub fn arb_command() -> impl Strategy<Value = Command> {
        prop_oneof![
            Just(Command::Poll(osdp::command::Poll)),
            Just(Command::Id(Id::standard())),
            Just(Command::Cap(osdp::command::Cap::standard())),
            Just(Command::LocalStatus(osdp::command::LocalStatus)),
            Just(Command::InputStatus(osdp::command::InputStatus)),
            Just(Command::OutputStatus(osdp::command::OutputStatus)),
            Just(Command::ReaderStatus(osdp::command::ReaderStatus)),
            (any::<u8>(), any::<u8>(), any::<u16>()).prop_map(|(o, c, t)| Command::Output(
                OutputControl::new(alloc::vec![osdp::command::output::OutputRecord {
                    output: o,
                    code: match c % 7 {
                        0 => OutputControlCode::Nop,
                        1 => OutputControlCode::PermanentOffAbortTimed,
                        2 => OutputControlCode::PermanentOnAbortTimed,
                        3 => OutputControlCode::PermanentOffAllowTimed,
                        4 => OutputControlCode::PermanentOnAllowTimed,
                        5 => OutputControlCode::TemporaryOnResume,
                        _ => OutputControlCode::TemporaryOffResume,
                    },
                    timer: t,
                }])
            )),
            (
                any::<u8>(),
                any::<u8>(),
                any::<u8>(),
                any::<u8>(),
                any::<u8>()
            )
                .prop_map(|(r, t, on, off, c)| Command::Buzzer(BuzzerControl {
                    reader: r,
                    tone: match t % 3 {
                        0 => BuzzerTone::None,
                        1 => BuzzerTone::Off,
                        _ => BuzzerTone::Default,
                    },
                    on_time: on,
                    off_time: off,
                    count: c,
                })),
            (0u8..=0x7Eu8, any::<u32>()).prop_map(|(a, b)| Command::ComSet(ComSet {
                address: a,
                baud: b
            })),
            any::<[u8; 16]>().prop_map(|k| Command::KeySet(KeySet::scbk(k))),
            any::<u16>().prop_map(|m| Command::AcuRxSize(AcuRxSize { max_size: m })),
            any::<u16>().prop_map(|d| Command::KeepActive(KeepActive { duration_ms: d })),
            (any::<u8>(), any::<u8>(), any::<u8>(), any::<u8>()).prop_map(|(r, t, f, q)| {
                Command::BioRead(BioRead {
                    reader: r,
                    bio_type: BioType::from_byte(t),
                    bio_format: BioFormat::from_byte(f),
                    quality: q,
                })
            }),
            (
                any::<u8>(),
                any::<u8>(),
                any::<u8>(),
                any::<u8>(),
                prop::collection::vec(any::<u8>(), 0..16)
            )
                .prop_map(|(r, t, f, q, tpl)| Command::BioMatch(BioMatch {
                    reader: r,
                    bio_type: BioType::from_byte(t),
                    bio_format: BioFormat::from_byte(f),
                    quality: q,
                    template: tpl,
                })),
            Just(Command::Abort(osdp::command::Abort)),
        ]
    }

    pub fn arb_reply() -> impl Strategy<Value = Reply> {
        prop_oneof![
            Just(Reply::Ack(Ack)),
            (0u8..=9u8).prop_map(|c| Reply::Nak(Nak::simple(match c {
                0 => NakErrorCode::NoError,
                1 => NakErrorCode::BadCrcOrChecksum,
                2 => NakErrorCode::CommandLengthError,
                3 => NakErrorCode::UnknownCommandCode,
                4 => NakErrorCode::UnexpectedSequenceNumber,
                5 => NakErrorCode::SecurityBlockTypeNotSupported,
                6 => NakErrorCode::EncryptedCommunicationRequired,
                7 => NakErrorCode::BioTypeNotSupported,
                8 => NakErrorCode::BioFormatNotSupported,
                _ => NakErrorCode::UnableToProcessCommandRecord,
            }))),
            (
                any::<[u8; 3]>(),
                any::<u8>(),
                any::<u8>(),
                any::<u32>(),
                any::<[u8; 3]>()
            )
                .prop_map(|(o, m, v, s, fw)| Reply::PdId(PdId {
                    vendor_oui: o,
                    model: m,
                    version: v,
                    serial: s,
                    firmware: fw,
                })),
            prop::collection::vec(any::<[u8; 3]>(), 0..16).prop_map(|caps| Reply::PdCap(
                PdCap::new(caps.into_iter().map(Capability::decode).collect())
            )),
            (any::<u8>(), any::<u8>()).prop_map(|(t, p)| Reply::LStatR(LStatR {
                tamper: t,
                power: p
            })),
            prop::collection::vec(any::<u8>(), 0..16)
                .prop_map(|i| Reply::IStatR(IStatR { inputs: i })),
            prop::collection::vec(any::<u8>(), 0..16)
                .prop_map(|o| Reply::OStatR(OStatR { outputs: o })),
            prop::collection::vec(any::<u8>(), 0..16)
                .prop_map(|r| Reply::RStatR(RStatR { readers: r })),
            (any::<u8>(), any::<u8>(), 0u16..=64u16).prop_map(|(r, fc, bc)| Reply::Raw(Raw {
                reader: r,
                format_code: fc,
                bit_count: bc,
                bits: alloc::vec![0u8; (bc as usize).div_ceil(8)],
            })),
            (0u8..=0x7Eu8, any::<u32>()).prop_map(|(a, b)| Reply::Com(Com {
                address: a,
                baud: b
            })),
            (any::<u8>(), any::<u8>(), any::<u8>()).prop_map(|(r, m, s)| Reply::BioMatchR(
                BioMatchR {
                    reader: r,
                    result: m,
                    score: s,
                }
            )),
            (any::<[u8; 8]>(), any::<[u8; 8]>(), any::<[u8; 16]>()).prop_map(|(c, b, cc)| {
                Reply::CCrypt(CCrypt {
                    cuid: c,
                    rnd_b: b,
                    client_cryptogram: cc,
                })
            }),
            any::<[u8; 16]>().prop_map(|m| Reply::RMacI(RMacI { r_mac_i: m })),
            Just(Reply::Busy(Busy)),
            (any::<u8>(), any::<u16>(), any::<u16>(), any::<u16>()).prop_map(|(s, d, p, f)| {
                Reply::FtStat(FtStat {
                    status: s,
                    delay_ms: d,
                    preferred_size: p,
                    flags: f,
                })
            }),
            (
                any::<u8>(),
                any::<u8>(),
                prop::collection::vec(any::<u8>(), 0..16)
            )
                .prop_map(|(r, q, d)| Reply::BioReadR(BioReadR {
                    reader: r,
                    bio_type: BioType::NotSpecified,
                    bio_format: BioFormat::NotSpecified,
                    quality: q,
                    data: {
                        let _ = r;
                        d
                    },
                })),
        ]
    }
}

extern crate alloc;

fn build_packet(
    addr: u8,
    sqn: u8,
    code: u8,
    data: alloc::vec::Vec<u8>,
    use_crc: bool,
) -> alloc::vec::Vec<u8> {
    PacketBuilder::plain(
        Address::pd(addr).unwrap(),
        ControlByte::new(
            Sqn::new(sqn).unwrap(),
            if use_crc {
                CtrlFlags::USE_CRC
            } else {
                CtrlFlags::empty()
            },
        ),
        code,
        data,
    )
    .encode()
    .unwrap()
}

proptest! {
    /// Build → bytes → parse → equal for typed commands.
    #[test]
    fn command_roundtrip(
        cmd in arb::arb_command(),
        addr in arb::arb_addr(),
        sqn in arb::arb_sqn(),
        use_crc in any::<bool>(),
    ) {
        let data = cmd.encode_data().unwrap();
        let bytes = build_packet(addr, sqn, cmd.code().as_byte(), data, use_crc);
        let (parsed, used) = ParsedPacket::parse(&bytes).expect("our own bytes parse");
        prop_assert_eq!(used, bytes.len());
        prop_assert_eq!(parsed.code, cmd.code().as_byte());
        let decoded = Command::decode(cmd.code(), parsed.data).unwrap();
        prop_assert_eq!(decoded, cmd);
    }

    /// Build → bytes → parse → equal for typed replies.
    #[test]
    fn reply_roundtrip(
        rep in arb::arb_reply(),
        addr in arb::arb_addr(),
        sqn in arb::arb_sqn(),
        use_crc in any::<bool>(),
    ) {
        let data = rep.encode_data().unwrap();
        let bytes = build_packet(addr, sqn, rep.code().as_byte(), data, use_crc);
        let (parsed, used) = ParsedPacket::parse(&bytes).expect("our own bytes parse");
        prop_assert_eq!(used, bytes.len());
        prop_assert_eq!(parsed.code, rep.code().as_byte());
        let decoded = Reply::decode(rep.code(), parsed.data).unwrap();
        prop_assert_eq!(decoded, rep);
    }

    /// LEN field on the wire equals `bytes.len()`.
    #[test]
    fn length_field_invariant(
        cmd in arb::arb_command(),
        addr in arb::arb_addr(),
        sqn in arb::arb_sqn(),
        use_crc in any::<bool>(),
    ) {
        let data = cmd.encode_data().unwrap();
        let bytes = build_packet(addr, sqn, cmd.code().as_byte(), data, use_crc);
        let len = u16::from_le_bytes([bytes[2], bytes[3]]) as usize;
        prop_assert_eq!(len, bytes.len());
    }

    /// Parser totality — never panics on hostile noise.
    #[test]
    fn parser_total(noise in prop::collection::vec(any::<u8>(), 0..2048)) {
        let _ = ParsedPacket::parse(&noise);
    }

    /// Mutating any single byte of a CRC-protected packet must either parse
    /// to a different packet or fail with an `Error`. It must never panic.
    #[test]
    fn one_bit_mutation_safe(
        cmd in arb::arb_command(),
        addr in arb::arb_addr(),
        sqn in arb::arb_sqn(),
        bit in 0usize..2048,
    ) {
        let data = cmd.encode_data().unwrap();
        let mut bytes = build_packet(addr, sqn, cmd.code().as_byte(), data, true);
        let n_bits = bytes.len() * 8;
        let bit = bit % n_bits;
        bytes[bit / 8] ^= 1 << (bit % 8);
        // Either parses (different from original) or errors. Either way: no panic.
        let _ = ParsedPacket::parse(&bytes);
    }
}
