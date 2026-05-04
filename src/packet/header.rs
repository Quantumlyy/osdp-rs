//! Address, sequence number, and CTRL byte primitives.

use crate::error::Error;

/// PD bus address.
///
/// # Spec: §5.4
///
/// Bits 0..7 carry the address. Bit 7 (`0x80`) is the *reply flag*: it is set
/// in messages from PD → ACU. Address `0x7F` is the broadcast group.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Address(u8);

impl Address {
    /// Wrap an arbitrary byte. Use [`Address::pd`] / [`Address::reply`] for
    /// validated constructors.
    pub const fn from_raw(byte: u8) -> Self {
        Self(byte)
    }

    /// Construct an ACU → PD address. Errors if the requested address is
    /// out of range (`> 0x7F` would clash with the reply flag).
    pub const fn pd(addr: u8) -> Result<Self, Error> {
        if addr > crate::BROADCAST_ADDR {
            return Err(Error::BadAddress(addr));
        }
        Ok(Self(addr))
    }

    /// Construct a PD → ACU address (i.e. with the reply flag set).
    pub const fn reply(addr: u8) -> Result<Self, Error> {
        if addr > crate::BROADCAST_ADDR {
            return Err(Error::BadAddress(addr));
        }
        Ok(Self(addr | crate::REPLY_FLAG))
    }

    /// Underlying byte (with reply flag, if any).
    pub const fn as_byte(self) -> u8 {
        self.0
    }

    /// PD address with the reply flag stripped.
    pub const fn pd_addr(self) -> u8 {
        self.0 & !crate::REPLY_FLAG
    }

    /// `true` if this is a reply address (PD → ACU).
    pub const fn is_reply(self) -> bool {
        (self.0 & crate::REPLY_FLAG) != 0
    }

    /// `true` if this is the broadcast address `0x7F`.
    pub const fn is_broadcast(self) -> bool {
        self.pd_addr() == crate::BROADCAST_ADDR
    }
}

/// Sequence number (`0..=3`).
///
/// # Spec: §5.7, §Table 2
///
/// SQN 0 is reserved for boot/recovery. Normal cycle is `1 → 2 → 3 → 1 → …`.
/// The ACU re-uses an outstanding SQN to request a *reply repeat* from the
/// PD; the PD treats a new SQN as a new command.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Sqn(u8);

impl Sqn {
    /// Boot / recovery SQN.
    pub const ZERO: Self = Self(0);

    /// Validating constructor.
    pub const fn new(n: u8) -> Result<Self, Error> {
        if n > 3 {
            Err(Error::BadSqn(n))
        } else {
            Ok(Self(n))
        }
    }

    /// Underlying value.
    pub const fn value(self) -> u8 {
        self.0
    }

    /// Next SQN in the cycle, skipping `0`.
    pub const fn next(self) -> Self {
        Self(match self.0 {
            0 | 3 => 1,
            n => n + 1,
        })
    }
}

bitflags::bitflags! {
    /// Control byte flags.
    ///
    /// # Spec: §5.9 Table 2
    ///
    /// ```text
    /// 7 6 5 4 3 2 1 0
    ///         | | | |
    ///         | | +-+--- SQN (0..3)
    ///         | +------- 0=cksum trailer, 1=CRC trailer
    ///         +--------- Security Control Block present
    /// ```
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct CtrlFlags: u8 {
        /// CRC-16 trailer (vs. 8-bit checksum when clear).
        const USE_CRC = 0x04;
        /// Security Control Block follows the CTRL byte.
        const HAS_SCB = 0x08;
    }
}

/// Decoded control byte.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ControlByte {
    /// Sequence number.
    pub sqn: Sqn,
    /// Flag bits.
    pub flags: CtrlFlags,
}

impl ControlByte {
    /// Construct from validated parts.
    pub const fn new(sqn: Sqn, flags: CtrlFlags) -> Self {
        Self { sqn, flags }
    }

    /// Encode to the on-wire byte.
    pub const fn encode(self) -> u8 {
        self.sqn.value() | self.flags.bits()
    }

    /// Decode an on-wire byte. Reserved bits (top nibble) must be zero.
    pub const fn decode(byte: u8) -> Result<Self, Error> {
        if (byte & 0xF0) != 0 {
            return Err(Error::BadControlByte(byte));
        }
        let sqn = match Sqn::new(byte & 0x03) {
            Ok(s) => s,
            Err(e) => return Err(e),
        };
        let flags = match CtrlFlags::from_bits(byte & 0x0C) {
            Some(f) => f,
            None => return Err(Error::BadControlByte(byte)),
        };
        Ok(Self { sqn, flags })
    }

    /// `true` if the trailer is a CRC-16.
    pub const fn use_crc(self) -> bool {
        self.flags.contains(CtrlFlags::USE_CRC)
    }

    /// `true` if a Security Control Block is present.
    pub const fn has_scb(self) -> bool {
        self.flags.contains(CtrlFlags::HAS_SCB)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ctrl_byte_roundtrip() {
        for raw in 0u8..=0x0F {
            let cb = ControlByte::decode(raw).unwrap();
            assert_eq!(cb.encode(), raw);
        }
    }

    #[test]
    fn ctrl_byte_rejects_reserved() {
        for raw in 0x10..=0xFFu8 {
            assert!(ControlByte::decode(raw).is_err());
        }
    }

    #[test]
    fn sqn_cycles_skipping_zero() {
        let mut s = Sqn::new(1).unwrap();
        for _ in 0..10 {
            s = s.next();
            assert_ne!(s.value(), 0);
        }
    }

    #[test]
    fn address_reply_flag() {
        let pd = Address::pd(0x05).unwrap();
        assert!(!pd.is_reply());
        assert_eq!(pd.as_byte(), 0x05);

        let reply = Address::reply(0x05).unwrap();
        assert!(reply.is_reply());
        assert_eq!(reply.as_byte(), 0x85);
        assert_eq!(reply.pd_addr(), 0x05);

        let bcast = Address::pd(0x7F).unwrap();
        assert!(bcast.is_broadcast());
    }
}
