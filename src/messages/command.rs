use std::vec;

use crate::constants::SOM;

#[derive(Debug, Default, Clone, Copy)]
#[repr(u8)]
pub enum CommandType {
    /// osdp_POLL - poll request
    #[default]
    Poll = 0x60,
    /// osdp_ID - ID report request
    IdReport = 0x61,
    /// osdp_CAP - peripheral device capabilities request
    DeviceCapabilities = 0x62,
    /// osdp_LSTAT - local status report request
    LocalStatus = 0x64,
    /// osdp_ISTAT - input status report request
    InputStatus = 0x65,
    /// osdp_OSTAT - output status report request
    OutputStatus = 0x66,
    /// osdp_RSTAT - reader status report request
    ReaderStatus = 0x67,
    /// osdp_OUT - output control command
    OutputControl = 0x68,
    /// osdp_LED - Reader LED control command
    LEDControl = 0x69,
    /// osdp_BUZ - Reader buzzer control command
    BuzzerControl = 0x6A,
    /// osdp_TEXT - Reader text output command
    TextOutput = 0x6B,
    /// osdp_COMSET - communication configuration command
    CommunicationSet = 0x6E,
    /// osdp_BIOREAD - scan and match biometric data
    BioRead = 0x73,
    /// osdp_BIOMATCH - scan and match biometric template
    BioMatch = 0x74,
    /// osdp_KEYSET - Encryption key set
    KeySet = 0x75,
    /// osdp_CHLNG - challenge and secure session initialization request
    SessionChallenge = 0x76,
    /// osdp_SCRYPT - server's random number and server cryptogram
    ServerCryptogram = 0x77,
    /// osdp_ACURXSIZE - ACU receive size
    MaxReplySize = 0x7B,
    /// osdp_FILETRANSFER - file transfer command
    FileTransfer = 0x7C,
    /// osdp_MFG - manufacturer specific command
    ManufacturerSpecific = 0x80,
    /// osdp_XWR - extended write data
    ExtendedWrite = 0xA1,
    /// osdp_ABORT - abort current operation
    Abort = 0xA2,
    /// osdp_PIVDATA - get PIV data
    PivData = 0xA3,
    /// osdp_GENAUTH - generate authenticate
    GenerateChallenge = 0xA4,
    /// osdp_CRAUTH - authenticate challenge
    AuthenticateChallenge = 0xA5,
    /// osdp_KEEPACTIVE - keep reader active
    KeepActive = 0xA7,
}

pub trait OSDPCommand {
    /// The command type.
    fn cmnd(&self) -> CommandType;

    /// The command data.
    fn build_command_data(&self) -> Vec<u8>;

    fn build_command_header(&self) -> Vec<u8> {
        vec![
            SOM,
            0x00, // TODO: device address
            0x00, // LEN_LSB
            0x00, // LEN_MSB
            0x00, // TODO: message control byte
            // TODO: security
            self.cmnd() as u8,
        ]
    }

    fn build_command_modify(&self, _command: &mut Vec<u8>) {}

    fn build_command(&self) -> Vec<u8> {
        let header = self.build_command_header();
        let data = self.build_command_data();

        let mut command = vec![];
        command.extend(header);
        command.extend(data);

        let command_length = command.len().to_be_bytes();
        command[2] = command_length[0];
        command[3] = command_length[1];

        self.build_command_modify(&mut command);

        command
    }
}
