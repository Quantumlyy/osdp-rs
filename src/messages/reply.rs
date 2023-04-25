#[derive(Debug, Default, Clone, Copy)]
#[repr(u8)]
pub enum ReplyType {
    /// osdp_ACK - general acknowledge; nothing to report
    #[default]
    Ack = 0x40,
    /// osdp_NAK - acknowledge; error response
    Nak = 0x41,
    /// osdp_PDID - device identification report
    PdIdReport = 0x45,
    /// osdp_PDCAP - device capabilities report
    PdCapabilitiesReport = 0x46,
    /// osdp_LSTATR - local status report
    LocalStatusReport = 0x48,
    /// osdp_ISTATR - input status report
    InputStatusReport = 0x49,
    /// osdp_OSTATR - output status report
    OutputStatusReport = 0x4A,
    /// osdp_RSTATR - reader tamper status report
    ReaderStatusReport = 0x4B,
    /// osdp_RAW - card data report; raw bits array
    RawReaderData = 0x50,
    /// osdp_FMT - card data report; character array
    FormattedReaderData = 0x51,
    /// osdp_KEYPAD - keypad data report
    KeypadData = 0x53,
    /// osdp_COM - communication configuration report
    PdCommunicationsConfigurationReport = 0x54,
    /// osdp_BIOREADR - scan and send biometric data
    BiometricData = 0x57,
    /// osdp_BIOMATCHR - scan and match biometric template
    BiometricMatchResult = 0x58,
    /// osdp_CCRYPT - client's ID and client's random number
    CrypticData = 0x76,
    /// osdp_RMAC_I - client cryptogram packet and the initial R-MAC
    InitialRMac = 0x78,
    /// osdp_BUSY - PD busy reply
    Busy = 0x79,
    /// osdp_FTSTAT - file transfer status
    FileTransferStatus = 0x7A,
    /// osdp_PIVDATAR - PIV data reply
    PIVData = 0x80,
    /// osdp_CRAUTHR - response to challenge
    ResponseToChallenge = 0x82,
    /// osdp_osdp_MFGSTATR - manufacturer specific status reply
    ManufactureSpecific = 0x90,
    /// osdp_XRD - extended read reply
    ExtendedRead = 0xB1,
}
