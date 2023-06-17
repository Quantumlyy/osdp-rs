/// Start of message
///
/// # Note
/// The constant value `0x53`, begins each message header.
/// This character is used for synchronization.
pub const SOM: u8 = 0x53;

/// Broadcast address
///
/// # Note
/// Address `0x7F` is reserved as a special "BROADCAST" address that each PD will accept and respond to, just as if it matched its communication address.
/// The reply message will use `0x7F` plus the reply flag (`0x7F` + `0x80` = `0xFF`) in its address field.
/// Since each PD will respond to `0x7F`, the use of the broadcast address should be limited to controlled (single PD) configurations.
pub const BROADCAST_ADDR: u8 = 0x7F;

pub const ASCII_DISPLAY_RANGE_START: u8 = 0x20;
pub const ASCII_DISPLAY_RANGE_END: u8 = 0x7E;
