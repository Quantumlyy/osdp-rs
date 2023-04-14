use crate::models::errors::length::InvalidLengthError;

/// Output Control Code
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
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

/// `osdp_OUT`
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct OutputState {
    /// The output number.
    ///
    /// # Example
    /// - `0x00` = first output
    /// - `0x01` = second output
    /// - etc...
    pub output_number: u8,
    /// The requested output state.
    pub control_code: OutputControlCode,
    /// A 16 bit timer, in units of 100ms.
    /// A zero value means "forever".
    pub timer: u16,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct OutputControl<'a> {
    pub output_states: &'a [OutputState],
}

impl<'a> OutputControl<'a> {
    /// # Arguments
    ///
    /// * `output_states` - The output states.
    pub fn new(output_states: &'a [OutputState]) -> Result<Self, InvalidLengthError> {
        if output_states.len() == 0 {
            return Err(InvalidLengthError::new_minimum(1));
        }

        Ok(Self { output_states })
    }
}
