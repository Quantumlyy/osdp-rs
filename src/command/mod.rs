//! Typed OSDP commands (ACU → PD).
//!
//! # Spec: §6 / Annex A.1
//!
//! Each command is a strongly-typed Rust value; encoding to wire bytes goes
//! through [`Command::encode_data`] and decoding through [`Command::decode`].
//! [`CommandCode`] is the byte enum from Annex A.1.

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

use crate::error::Error;

/// All `CMND` byte values from Annex A.1 of the spec.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
#[allow(missing_docs)]
pub enum CommandCode {
    Poll = 0x60,
    Id = 0x61,
    Cap = 0x62,
    LStat = 0x64,
    IStat = 0x65,
    OStat = 0x66,
    RStat = 0x67,
    Out = 0x68,
    Led = 0x69,
    Buz = 0x6A,
    Text = 0x6B,
    ComSet = 0x6E,
    BioRead = 0x73,
    BioMatch = 0x74,
    KeySet = 0x75,
    Chlng = 0x76,
    SCrypt = 0x77,
    AcuRxSize = 0x7B,
    FileTransfer = 0x7C,
    Mfg = 0x80,
    XWrite = 0xA1,
    Abort = 0xA2,
    PivData = 0xA3,
    GenAuth = 0xA4,
    CrAuth = 0xA5,
    KeepActive = 0xA7,
}

impl CommandCode {
    /// Parse from raw byte.
    pub const fn from_byte(b: u8) -> Result<Self, Error> {
        Ok(match b {
            0x60 => Self::Poll,
            0x61 => Self::Id,
            0x62 => Self::Cap,
            0x64 => Self::LStat,
            0x65 => Self::IStat,
            0x66 => Self::OStat,
            0x67 => Self::RStat,
            0x68 => Self::Out,
            0x69 => Self::Led,
            0x6A => Self::Buz,
            0x6B => Self::Text,
            0x6E => Self::ComSet,
            0x73 => Self::BioRead,
            0x74 => Self::BioMatch,
            0x75 => Self::KeySet,
            0x76 => Self::Chlng,
            0x77 => Self::SCrypt,
            0x7B => Self::AcuRxSize,
            0x7C => Self::FileTransfer,
            0x80 => Self::Mfg,
            0xA1 => Self::XWrite,
            0xA2 => Self::Abort,
            0xA3 => Self::PivData,
            0xA4 => Self::GenAuth,
            0xA5 => Self::CrAuth,
            0xA7 => Self::KeepActive,
            other => return Err(Error::UnknownCommand(other)),
        })
    }

    /// Raw byte.
    pub const fn as_byte(self) -> u8 {
        self as u8
    }
}

mod abort;
mod acu_rx_size;
mod biometric;
mod buzzer;
mod cap;
mod chlng;
mod comset;
mod file_transfer;
mod id;
mod keep_active;
mod keyset;
mod led;
mod local_status;
mod mfg;
mod output;
mod piv;
mod poll;
mod scrypt;
mod text;
mod xwrite;

pub use abort::Abort;
pub use acu_rx_size::AcuRxSize;
pub use biometric::{BioFormat, BioMatch, BioRead, BioType};
pub use buzzer::{BuzzerControl, BuzzerTone};
pub use cap::Cap;
pub use chlng::Chlng;
pub use comset::ComSet;
pub use file_transfer::FileTransfer;
pub use id::Id;
pub use keep_active::KeepActive;
pub use keyset::KeySet;
pub use led::{LedColor, LedControl, LedPermanent, LedTemporary};
pub use local_status::{InputStatus, LocalStatus, OutputStatus, ReaderStatus};
pub use mfg::Mfg;
pub use output::{OutputControl, OutputControlCode};
pub use piv::{CrAuth, GenAuth, PivData};
pub use poll::Poll;
pub use scrypt::SCrypt;
pub use text::{Text, TextCommand};
pub use xwrite::{XWrite, XwrMode00, XwrMode01};

/// Typed dispatch over every command supported by the crate.
///
/// Variants whose parser is not yet implemented carry the raw payload bytes
/// so that round-tripping unknown-but-recognized commands still works.
#[cfg(feature = "alloc")]
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum Command {
    Poll(Poll),
    Id(Id),
    Cap(Cap),
    LocalStatus(LocalStatus),
    InputStatus(InputStatus),
    OutputStatus(OutputStatus),
    ReaderStatus(ReaderStatus),
    Output(OutputControl),
    Led(LedControl),
    Buzzer(BuzzerControl),
    Text(Text),
    ComSet(ComSet),
    BioRead(BioRead),
    BioMatch(BioMatch),
    KeySet(KeySet),
    Chlng(Chlng),
    SCrypt(SCrypt),
    AcuRxSize(AcuRxSize),
    FileTransfer(FileTransfer),
    Mfg(Mfg),
    XWrite(XWrite),
    Abort(Abort),
    PivData(PivData),
    GenAuth(GenAuth),
    CrAuth(CrAuth),
    KeepActive(KeepActive),
}

#[cfg(feature = "alloc")]
impl Command {
    /// Code byte for this command.
    pub fn code(&self) -> CommandCode {
        match self {
            Command::Poll(_) => CommandCode::Poll,
            Command::Id(_) => CommandCode::Id,
            Command::Cap(_) => CommandCode::Cap,
            Command::LocalStatus(_) => CommandCode::LStat,
            Command::InputStatus(_) => CommandCode::IStat,
            Command::OutputStatus(_) => CommandCode::OStat,
            Command::ReaderStatus(_) => CommandCode::RStat,
            Command::Output(_) => CommandCode::Out,
            Command::Led(_) => CommandCode::Led,
            Command::Buzzer(_) => CommandCode::Buz,
            Command::Text(_) => CommandCode::Text,
            Command::ComSet(_) => CommandCode::ComSet,
            Command::BioRead(_) => CommandCode::BioRead,
            Command::BioMatch(_) => CommandCode::BioMatch,
            Command::KeySet(_) => CommandCode::KeySet,
            Command::Chlng(_) => CommandCode::Chlng,
            Command::SCrypt(_) => CommandCode::SCrypt,
            Command::AcuRxSize(_) => CommandCode::AcuRxSize,
            Command::FileTransfer(_) => CommandCode::FileTransfer,
            Command::Mfg(_) => CommandCode::Mfg,
            Command::XWrite(_) => CommandCode::XWrite,
            Command::Abort(_) => CommandCode::Abort,
            Command::PivData(_) => CommandCode::PivData,
            Command::GenAuth(_) => CommandCode::GenAuth,
            Command::CrAuth(_) => CommandCode::CrAuth,
            Command::KeepActive(_) => CommandCode::KeepActive,
        }
    }

    /// Encode the DATA payload for this command (does not include the code byte).
    pub fn encode_data(&self) -> Result<Vec<u8>, Error> {
        match self {
            Command::Poll(c) => c.encode(),
            Command::Id(c) => c.encode(),
            Command::Cap(c) => c.encode(),
            Command::LocalStatus(c) => c.encode(),
            Command::InputStatus(c) => c.encode(),
            Command::OutputStatus(c) => c.encode(),
            Command::ReaderStatus(c) => c.encode(),
            Command::Output(c) => c.encode(),
            Command::Led(c) => c.encode(),
            Command::Buzzer(c) => c.encode(),
            Command::Text(c) => c.encode(),
            Command::ComSet(c) => c.encode(),
            Command::BioRead(c) => c.encode(),
            Command::BioMatch(c) => c.encode(),
            Command::KeySet(c) => c.encode(),
            Command::Chlng(c) => c.encode(),
            Command::SCrypt(c) => c.encode(),
            Command::AcuRxSize(c) => c.encode(),
            Command::FileTransfer(c) => c.encode(),
            Command::Mfg(c) => c.encode(),
            Command::XWrite(c) => c.encode(),
            Command::Abort(c) => c.encode(),
            Command::PivData(c) => c.encode(),
            Command::GenAuth(c) => c.encode(),
            Command::CrAuth(c) => c.encode(),
            Command::KeepActive(c) => c.encode(),
        }
    }

    /// Decode the typed payload, given the code byte and DATA bytes.
    pub fn decode(code: CommandCode, data: &[u8]) -> Result<Self, Error> {
        Ok(match code {
            CommandCode::Poll => Command::Poll(Poll::decode(data)?),
            CommandCode::Id => Command::Id(Id::decode(data)?),
            CommandCode::Cap => Command::Cap(Cap::decode(data)?),
            CommandCode::LStat => Command::LocalStatus(LocalStatus::decode(data)?),
            CommandCode::IStat => Command::InputStatus(InputStatus::decode(data)?),
            CommandCode::OStat => Command::OutputStatus(OutputStatus::decode(data)?),
            CommandCode::RStat => Command::ReaderStatus(ReaderStatus::decode(data)?),
            CommandCode::Out => Command::Output(OutputControl::decode(data)?),
            CommandCode::Led => Command::Led(LedControl::decode(data)?),
            CommandCode::Buz => Command::Buzzer(BuzzerControl::decode(data)?),
            CommandCode::Text => Command::Text(Text::decode(data)?),
            CommandCode::ComSet => Command::ComSet(ComSet::decode(data)?),
            CommandCode::BioRead => Command::BioRead(BioRead::decode(data)?),
            CommandCode::BioMatch => Command::BioMatch(BioMatch::decode(data)?),
            CommandCode::KeySet => Command::KeySet(KeySet::decode(data)?),
            CommandCode::Chlng => Command::Chlng(Chlng::decode(data)?),
            CommandCode::SCrypt => Command::SCrypt(SCrypt::decode(data)?),
            CommandCode::AcuRxSize => Command::AcuRxSize(AcuRxSize::decode(data)?),
            CommandCode::FileTransfer => Command::FileTransfer(FileTransfer::decode(data)?),
            CommandCode::Mfg => Command::Mfg(Mfg::decode(data)?),
            CommandCode::XWrite => Command::XWrite(XWrite::decode(data)?),
            CommandCode::Abort => Command::Abort(Abort::decode(data)?),
            CommandCode::PivData => Command::PivData(PivData::decode(data)?),
            CommandCode::GenAuth => Command::GenAuth(GenAuth::decode(data)?),
            CommandCode::CrAuth => Command::CrAuth(CrAuth::decode(data)?),
            CommandCode::KeepActive => Command::KeepActive(KeepActive::decode(data)?),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn code_roundtrip() {
        for byte in [
            0x60u8, 0x61, 0x62, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6A, 0x6B, 0x6E, 0x73, 0x74,
            0x75, 0x76, 0x77, 0x7B, 0x7C, 0x80, 0xA1, 0xA2, 0xA3, 0xA4, 0xA5, 0xA7,
        ] {
            assert_eq!(CommandCode::from_byte(byte).unwrap().as_byte(), byte);
        }
    }

    #[test]
    fn unknown_byte_errors() {
        assert!(matches!(CommandCode::from_byte(0xFF), Err(Error::UnknownCommand(0xFF))));
    }
}
