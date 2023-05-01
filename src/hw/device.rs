pub trait Device {
    /// Address of the device.
    fn addr(&self) -> u8;

    fn sqn(&self) -> u8;
    /// Determines if the CRC is present in the message.
    fn crc(&self) -> bool;
    /// Determines if the Security Control Block is present in the message.
    fn scb(&self) -> bool;

    /// Message Control Byte.
    fn control_byte(&self) -> u8 {
        let crc = if self.crc() { 0x04 } else { 0x00 };
        let scb = if self.scb() { 0x08 } else { 0x00 };

        self.sqn() & 0x03 | crc | scb
    }
}
