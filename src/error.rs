//! Crate-wide error type. `core::error::Error`-compatible.

use core::fmt;

/// Convenience [`Result`] alias.
pub type Result<T> = core::result::Result<T, Error>;

/// Crate-wide error.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    /// Buffer too small to hold a syntactically valid packet header.
    Truncated {
        /// Bytes available.
        have: usize,
        /// Bytes required.
        need: usize,
    },
    /// SOM mismatch — packet did not start with `0x53`.
    BadSom(u8),
    /// `LEN` field disagrees with the actual buffer length.
    BadLength {
        /// Length declared in the LEN field.
        declared: usize,
        /// Length actually delivered.
        actual: usize,
    },
    /// Reserved bits in the [`crate::ControlByte`] are non-zero, or the byte is
    /// otherwise malformed.
    BadControlByte(u8),
    /// CRC verification failed.
    BadCrc {
        /// CRC carried on the wire.
        got: u16,
        /// CRC we computed from the header + payload.
        want: u16,
    },
    /// Checksum verification failed.
    BadChecksum {
        /// Trailer byte carried on the wire.
        got: u8,
        /// Checksum we computed.
        want: u8,
    },
    /// Address field is reserved or invalid.
    BadAddress(u8),
    /// Sequence number is not in `0..=3`.
    BadSqn(u8),
    /// Security block type is unknown.
    BadSecurityBlock(u8),
    /// Length of the security block is wrong for its type.
    BadSecurityBlockLength(u8),
    /// MAC verification failed (constant-time compare).
    BadMac,
    /// MAC is too short (must be at least 4 bytes on the wire).
    ShortMac,
    /// Command code is not recognized.
    UnknownCommand(u8),
    /// Reply code is not recognized.
    UnknownReply(u8),
    /// Payload bytes do not match the expected layout for this command/reply.
    MalformedPayload {
        /// Command or reply code.
        code: u8,
        /// One-line explanation.
        reason: &'static str,
    },
    /// Multi-part receiver detected an invariant violation.
    Multipart(MultipartError),
    /// Secure-channel state machine rejected an event.
    SecureSession(SecureSessionError),
    /// Transport-level I/O.
    Io(&'static str),
    /// Reply did not arrive within the configured budget.
    Timeout,
    /// PD declared off-line.
    Offline,
    /// Address mismatch between expected PD and reply ADDR.
    AddrMismatch {
        /// Address we sent to.
        sent: u8,
        /// Address echoed back in the reply.
        got: u8,
    },
    /// Reply carried a sequence number we did not request. Per spec §5.7 /
    /// Table 2 the PD must echo the ACU's SQN; a mismatch typically means a
    /// stale reply from a desynchronised PD.
    SqnMismatch {
        /// SQN the ACU sent in the prompting command.
        expected: u8,
        /// SQN observed in the reply.
        got: u8,
    },
    /// PD answered with [`crate::reply::Nak`].
    Nak {
        /// Error code byte from the reply.
        code: u8,
    },
    /// Output buffer was too small.
    BufferOverflow {
        /// Minimum required capacity.
        need: usize,
        /// Capacity available.
        have: usize,
    },
}

/// Subcategory of multi-part assembler errors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum MultipartError {
    /// First fragment did not arrive at offset 0.
    UnexpectedFirstOffset(u16),
    /// Out-of-order or overlapping fragment.
    OutOfOrderOffset {
        /// Expected next offset.
        expected: u16,
        /// Offset declared by the fragment we received.
        got: u16,
    },
    /// `MpSizeTotal` differs from the figure in a previous fragment.
    InconsistentTotal {
        /// First fragment's stated total.
        first: u16,
        /// New fragment's stated total.
        now: u16,
    },
    /// Final fragment overruns the declared total.
    OverflowsTotal,
    /// `MpFragmentSize` does not match the actual payload.
    BadFragmentSize,
    /// Counterparty aborted the transfer.
    Aborted,
}

/// Subcategory of secure-channel state errors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum SecureSessionError {
    /// Cryptogram comparison failed.
    BadCryptogram,
    /// Caller attempted to wrap a frame before SCS was fully established.
    NotSecure,
    /// SCS event arrived in a state that does not accept it.
    BadTransition,
    /// Encrypted DATA must be padded to a 16-byte multiple.
    BadPadding,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Truncated { have, need } => {
                write!(f, "truncated: have {have} bytes, need {need}")
            }
            Error::BadSom(b) => write!(f, "bad SOM byte: {b:#04x}"),
            Error::BadLength { declared, actual } => {
                write!(f, "length mismatch: declared {declared}, actual {actual}")
            }
            Error::BadControlByte(b) => write!(f, "bad control byte: {b:#04x}"),
            Error::BadCrc { got, want } => write!(f, "bad CRC: got {got:#06x}, want {want:#06x}"),
            Error::BadChecksum { got, want } => {
                write!(f, "bad checksum: got {got:#04x}, want {want:#04x}")
            }
            Error::BadAddress(b) => write!(f, "bad address: {b:#04x}"),
            Error::BadSqn(b) => write!(f, "bad sequence number: {b}"),
            Error::BadSecurityBlock(b) => write!(f, "unknown SCB type: {b:#04x}"),
            Error::BadSecurityBlockLength(b) => write!(f, "bad SCB length for type {b:#04x}"),
            Error::BadMac => f.write_str("bad MAC"),
            Error::ShortMac => f.write_str("MAC too short"),
            Error::UnknownCommand(c) => write!(f, "unknown command code: {c:#04x}"),
            Error::UnknownReply(c) => write!(f, "unknown reply code: {c:#04x}"),
            Error::MalformedPayload { code, reason } => {
                write!(f, "malformed payload for {code:#04x}: {reason}")
            }
            Error::Multipart(e) => write!(f, "multipart: {e:?}"),
            Error::SecureSession(e) => write!(f, "secure session: {e:?}"),
            Error::Io(s) => write!(f, "io: {s}"),
            Error::Timeout => f.write_str("reply timed out"),
            Error::Offline => f.write_str("PD declared off-line"),
            Error::AddrMismatch { sent, got } => {
                write!(f, "address mismatch: sent {sent:#04x}, got {got:#04x}")
            }
            Error::SqnMismatch { expected, got } => {
                write!(f, "SQN mismatch: expected {expected}, got {got}")
            }
            Error::Nak { code } => write!(f, "PD replied NAK ({code:#04x})"),
            Error::BufferOverflow { need, have } => {
                write!(f, "buffer overflow: need {need}, have {have}")
            }
        }
    }
}

impl core::error::Error for Error {}

impl From<MultipartError> for Error {
    fn from(e: MultipartError) -> Self {
        Error::Multipart(e)
    }
}

impl From<SecureSessionError> for Error {
    fn from(e: SecureSessionError) -> Self {
        Error::SecureSession(e)
    }
}
