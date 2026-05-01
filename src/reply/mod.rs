//! Typed OSDP replies (PD → ACU). See sibling [`crate::command`].
//!
//! # Spec: §7 / Annex A.2

use crate::error::Error;

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// All `REPLY` byte values from Annex A.2 of the spec.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
#[allow(missing_docs)]
pub enum ReplyCode {
    Ack = 0x40,
    Nak = 0x41,
    PdId = 0x45,
    PdCap = 0x46,
    LStatR = 0x48,
    IStatR = 0x49,
    OStatR = 0x4A,
    RStatR = 0x4B,
    Raw = 0x50,
    Fmt = 0x51,
    Keypad = 0x53,
    Com = 0x54,
    BioReadR = 0x57,
    BioMatchR = 0x58,
    CCrypt = 0x76,
    RMacI = 0x78,
    Busy = 0x79,
    FtStat = 0x7A,
    PivDataR = 0x80,
    GenAuthR = 0x81,
    CrAuthR = 0x82,
    MfgStatR = 0x83,
    MfgErrR = 0x84,
    MfgRep = 0x90,
    Xrd = 0xB1,
}

impl ReplyCode {
    /// Parse from raw byte.
    pub const fn from_byte(b: u8) -> Result<Self, Error> {
        Ok(match b {
            0x40 => Self::Ack,
            0x41 => Self::Nak,
            0x45 => Self::PdId,
            0x46 => Self::PdCap,
            0x48 => Self::LStatR,
            0x49 => Self::IStatR,
            0x4A => Self::OStatR,
            0x4B => Self::RStatR,
            0x50 => Self::Raw,
            0x51 => Self::Fmt,
            0x53 => Self::Keypad,
            0x54 => Self::Com,
            0x57 => Self::BioReadR,
            0x58 => Self::BioMatchR,
            0x76 => Self::CCrypt,
            0x78 => Self::RMacI,
            0x79 => Self::Busy,
            0x7A => Self::FtStat,
            0x80 => Self::PivDataR,
            0x81 => Self::GenAuthR,
            0x82 => Self::CrAuthR,
            0x83 => Self::MfgStatR,
            0x84 => Self::MfgErrR,
            0x90 => Self::MfgRep,
            0xB1 => Self::Xrd,
            other => return Err(Error::UnknownReply(other)),
        })
    }

    /// Raw byte.
    pub const fn as_byte(self) -> u8 {
        self as u8
    }
}

mod ack;
mod bio;
mod busy;
mod ccrypt;
mod com;
mod crauth;
mod ft_stat;
mod genauth;
mod istat;
mod keypad;
mod lstat;
mod mfg;
mod nak;
mod ostat;
mod pdcap;
mod pdid;
mod piv;
mod raw_card;
mod rmac_i;
mod rstat;
mod xrd;

pub use ack::Ack;
pub use bio::{BioMatchR, BioReadR};
pub use busy::Busy;
pub use ccrypt::CCrypt;
pub use com::Com;
pub use crauth::CrAuthR;
pub use ft_stat::FtStat;
pub use genauth::GenAuthR;
pub use istat::IStatR;
pub use keypad::Keypad;
pub use lstat::LStatR;
pub use mfg::{MfgErrR, MfgRep, MfgStatR};
pub use nak::{Nak, NakErrorCode};
pub use ostat::OStatR;
pub use pdcap::PdCap;
pub use pdid::PdId;
pub use piv::PivDataR;
pub use raw_card::{Fmt, Raw};
pub use rmac_i::RMacI;
pub use rstat::RStatR;
pub use xrd::Xrd;

/// Typed dispatch over every reply.
#[cfg(feature = "alloc")]
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum Reply {
    Ack(Ack),
    Nak(Nak),
    PdId(PdId),
    PdCap(PdCap),
    LStatR(LStatR),
    IStatR(IStatR),
    OStatR(OStatR),
    RStatR(RStatR),
    Raw(Raw),
    Fmt(Fmt),
    Keypad(Keypad),
    Com(Com),
    BioReadR(BioReadR),
    BioMatchR(BioMatchR),
    CCrypt(CCrypt),
    RMacI(RMacI),
    Busy(Busy),
    FtStat(FtStat),
    PivDataR(PivDataR),
    GenAuthR(GenAuthR),
    CrAuthR(CrAuthR),
    MfgStatR(MfgStatR),
    MfgErrR(MfgErrR),
    MfgRep(MfgRep),
    Xrd(Xrd),
}

#[cfg(feature = "alloc")]
impl Reply {
    /// Code byte for this reply.
    pub fn code(&self) -> ReplyCode {
        match self {
            Reply::Ack(_) => ReplyCode::Ack,
            Reply::Nak(_) => ReplyCode::Nak,
            Reply::PdId(_) => ReplyCode::PdId,
            Reply::PdCap(_) => ReplyCode::PdCap,
            Reply::LStatR(_) => ReplyCode::LStatR,
            Reply::IStatR(_) => ReplyCode::IStatR,
            Reply::OStatR(_) => ReplyCode::OStatR,
            Reply::RStatR(_) => ReplyCode::RStatR,
            Reply::Raw(_) => ReplyCode::Raw,
            Reply::Fmt(_) => ReplyCode::Fmt,
            Reply::Keypad(_) => ReplyCode::Keypad,
            Reply::Com(_) => ReplyCode::Com,
            Reply::BioReadR(_) => ReplyCode::BioReadR,
            Reply::BioMatchR(_) => ReplyCode::BioMatchR,
            Reply::CCrypt(_) => ReplyCode::CCrypt,
            Reply::RMacI(_) => ReplyCode::RMacI,
            Reply::Busy(_) => ReplyCode::Busy,
            Reply::FtStat(_) => ReplyCode::FtStat,
            Reply::PivDataR(_) => ReplyCode::PivDataR,
            Reply::GenAuthR(_) => ReplyCode::GenAuthR,
            Reply::CrAuthR(_) => ReplyCode::CrAuthR,
            Reply::MfgStatR(_) => ReplyCode::MfgStatR,
            Reply::MfgErrR(_) => ReplyCode::MfgErrR,
            Reply::MfgRep(_) => ReplyCode::MfgRep,
            Reply::Xrd(_) => ReplyCode::Xrd,
        }
    }

    /// Encode the DATA payload (does not include the code byte).
    pub fn encode_data(&self) -> Result<Vec<u8>, Error> {
        match self {
            Reply::Ack(r) => r.encode(),
            Reply::Nak(r) => r.encode(),
            Reply::PdId(r) => r.encode(),
            Reply::PdCap(r) => r.encode(),
            Reply::LStatR(r) => r.encode(),
            Reply::IStatR(r) => r.encode(),
            Reply::OStatR(r) => r.encode(),
            Reply::RStatR(r) => r.encode(),
            Reply::Raw(r) => r.encode(),
            Reply::Fmt(r) => r.encode(),
            Reply::Keypad(r) => r.encode(),
            Reply::Com(r) => r.encode(),
            Reply::BioReadR(r) => r.encode(),
            Reply::BioMatchR(r) => r.encode(),
            Reply::CCrypt(r) => r.encode(),
            Reply::RMacI(r) => r.encode(),
            Reply::Busy(r) => r.encode(),
            Reply::FtStat(r) => r.encode(),
            Reply::PivDataR(r) => r.encode(),
            Reply::GenAuthR(r) => r.encode(),
            Reply::CrAuthR(r) => r.encode(),
            Reply::MfgStatR(r) => r.encode(),
            Reply::MfgErrR(r) => r.encode(),
            Reply::MfgRep(r) => r.encode(),
            Reply::Xrd(r) => r.encode(),
        }
    }

    /// Decode the typed payload, given the code byte and DATA bytes.
    pub fn decode(code: ReplyCode, data: &[u8]) -> Result<Self, Error> {
        Ok(match code {
            ReplyCode::Ack => Reply::Ack(Ack::decode(data)?),
            ReplyCode::Nak => Reply::Nak(Nak::decode(data)?),
            ReplyCode::PdId => Reply::PdId(PdId::decode(data)?),
            ReplyCode::PdCap => Reply::PdCap(PdCap::decode(data)?),
            ReplyCode::LStatR => Reply::LStatR(LStatR::decode(data)?),
            ReplyCode::IStatR => Reply::IStatR(IStatR::decode(data)?),
            ReplyCode::OStatR => Reply::OStatR(OStatR::decode(data)?),
            ReplyCode::RStatR => Reply::RStatR(RStatR::decode(data)?),
            ReplyCode::Raw => Reply::Raw(Raw::decode(data)?),
            ReplyCode::Fmt => Reply::Fmt(Fmt::decode(data)?),
            ReplyCode::Keypad => Reply::Keypad(Keypad::decode(data)?),
            ReplyCode::Com => Reply::Com(Com::decode(data)?),
            ReplyCode::BioReadR => Reply::BioReadR(BioReadR::decode(data)?),
            ReplyCode::BioMatchR => Reply::BioMatchR(BioMatchR::decode(data)?),
            ReplyCode::CCrypt => Reply::CCrypt(CCrypt::decode(data)?),
            ReplyCode::RMacI => Reply::RMacI(RMacI::decode(data)?),
            ReplyCode::Busy => Reply::Busy(Busy::decode(data)?),
            ReplyCode::FtStat => Reply::FtStat(FtStat::decode(data)?),
            ReplyCode::PivDataR => Reply::PivDataR(PivDataR::decode(data)?),
            ReplyCode::GenAuthR => Reply::GenAuthR(GenAuthR::decode(data)?),
            ReplyCode::CrAuthR => Reply::CrAuthR(CrAuthR::decode(data)?),
            ReplyCode::MfgStatR => Reply::MfgStatR(MfgStatR::decode(data)?),
            ReplyCode::MfgErrR => Reply::MfgErrR(MfgErrR::decode(data)?),
            ReplyCode::MfgRep => Reply::MfgRep(MfgRep::decode(data)?),
            ReplyCode::Xrd => Reply::Xrd(Xrd::decode(data)?),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn code_roundtrip() {
        for byte in [
            0x40u8, 0x41, 0x45, 0x46, 0x48, 0x49, 0x4A, 0x4B, 0x50, 0x51, 0x53, 0x54, 0x57, 0x58,
            0x76, 0x78, 0x79, 0x7A, 0x80, 0x81, 0x82, 0x83, 0x84, 0x90, 0xB1,
        ] {
            assert_eq!(ReplyCode::from_byte(byte).unwrap().as_byte(), byte);
        }
    }

    #[test]
    fn unknown_byte_errors() {
        assert!(matches!(ReplyCode::from_byte(0xFF), Err(Error::UnknownReply(0xFF))));
    }
}
