/// Output Control Code
#[derive(Default, Clone, Copy)]
#[repr(u8)]
pub enum OutputControlCode {
    /// NOP – do not alter this output.
    #[default]
    Nop = 0x00,
    /// Set the permanent state to OFF, abort timed operation (if any).
    PermanentStateOffAbortTimedOperation = 0x01,
    /// Set the permanent state to ON, abort timed operation (if any).
    PermanentStateOnAbortTimedOperation = 0x02,
    /// Set the permanent state to OFF, allow timed operation to complete.
    PermanentStateOffAllowTimedOperation = 0x03,
    /// Set the permanent state to ON, allow timed operation to complete.
    PermanentStateOnAllowTimedOperation = 0x04,
    /// Set the temporary state to ON, resume permanent state on timeout.
    TemporaryStateOnResumePermanentState = 0x05,
    /// Set the temporary state to OFF, resume permanent state on timeout.
    TemporaryStateOffResumePermanentState = 0x06,
}
