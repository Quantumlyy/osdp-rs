use crate::models::errors::length::InvalidLengthError;

/// Tone code.
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum ToneCode {
    /// No tone (off).
    /// 
    /// # Note
    /// Use of this value is deprecated. Use `ToneCode::Off` instead.
    #[deprecated]
    NoTone = 0x00,
    #[default]
    Off = 0x01,
    DefaultTone = 0x02,
}

/// `osdp_BUZZ`
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct BuzzerControlState {
    /// The reader number.
    /// 
    /// # Example
    /// - `0x00` = first reader
    /// - `0x01` = second reader
    /// - etc...
    pub reader_number: u8,
    pub tone_code: ToneCode,
    /// The ON duration of the sound, in units of 100ms.
    /// 
    /// # Note
    /// Must be non-zero unless `tone_code` is `ToneCode::Off`.
    pub on_time: u8,
    /// The OFF duration of the sound, in units of 100ms.
    pub off_time: u8,
    /// The number of times to repeat the ON/OFF cycle.
    /// 0 = tone continues until another tone command is received.
    pub count: u8,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ReaderBuzzerControl<'a> {
    pub buzzer_states: &'a [BuzzerControlState],
}

impl<'a> ReaderBuzzerControl<'a> {
    /// # Arguments
    ///
    /// * `buzzer_states` - The buzzer control states.
    pub fn new(buzzer_states: &'a [BuzzerControlState]) -> Result<Self, InvalidLengthError> {
        if buzzer_states.len() == 0 {
            return Err(InvalidLengthError::new_minimum(1));
        }

        Ok(Self { buzzer_states })
    }
}
