use crate::models::errors::length::InvalidLengthError;

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum LEDTemporaryControlCode {
    /// NOP - do not alter this LED's temporary settings.
    /// The remaining values of the temporary settings record are ignored.
    #[default]
    NOP = 0x00,
    /// Cancel any temporary operation and display this LED's permanent state immediately.
    CancelTemporaryOperation = 0x01,
    /// Set the temporary state as given and start timer immediately.
    SetTemporaryState = 0x02,
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum LEDPermanentControlCode {
    #[default]
    NOP = 0x00,
    SetPermanentState = 0x01,
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum LEDColor {
    /// off/unlit
    #[default]
    Black = 0,
    Red = 1,
    Green = 2,
    Amber = 3,
    Blue = 4,
    Magenta = 5,
    Cyan = 6,
    White = 7,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ReaderLEDControlState {
    /// The reader number.
    pub reader: u8,
    /// The LED number.
    ///  - `0x00` = first LED
    ///  - `0x01` = second LED
    ///  - etc...
    pub led: u8,
    // Temporary settings
    /// The mode to enter temporarily.
    pub temporary_control_code: LEDTemporaryControlCode,
    /// ON duration of the flash in units of 100ms.
    /// A zero value means no duration.
    pub temporary_on_time: u8,
    /// OFF duration of the flash in units of 100ms.
    /// A zero value means no duration.
    pub temporary_off_time: u8,
    /// The color set during the ON time.
    pub temporary_on_color: LEDColor,
    /// The color set during the OFF time.
    pub temporary_off_color: LEDColor,
    /// The timer value in units of 100ms.
    /// A zero value means "forever".
    pub temporary_timer: u16,
    // Permanent settings
    /// The mode to return to after the timer expires.
    pub permanent_control_code: LEDPermanentControlCode,
    /// ON duration of the flash in units of 100ms.
    /// A zero value means no duration.
    pub permanent_on_time: u8,
    /// OFF duration of the flash in units of 100ms.
    /// A zero value means no duration.
    pub permanent_off_time: u8,
    /// The color set during the ON time.
    pub permanent_on_color: LEDColor,
    /// The color set during the OFF time.
    pub permanent_off_color: LEDColor,
}

impl ReaderLEDControlState {
    /// # Arguments
    ///
    /// * `reader` - The reader number.
    /// * `led` - The LED number.
    ///             - `0x00` = first LED
    ///             - `0x01` = second LED
    ///             - etc...
    /// * `temporary_control_code` - The mode to enter temporarily.
    /// * `temporary_on_time` - ON duration of the flash in units of 100ms. A zero value means no duration.
    /// * `temporary_off_time` - OFF duration of the flash in units of 100ms. A zero value means no duration.
    /// * `temporary_on_color` - The color set during the ON time.
    /// * `temporary_off_color` - The color set during the OFF time.
    /// * `temporary_timer` - The timer value in units of 100ms. A zero value means "forever".
    /// * `permanent_control_code` - The mode to return to after the timer expires.
    /// * `permanent_on_time` - ON duration of the flash in units of 100ms. A zero value means no duration.
    /// * `permanent_off_time` - OFF duration of the flash in units of 100ms. A zero value means no duration.
    /// * `permanent_on_color` - The color set during the ON time.
    /// * `permanent_off_color` - The color set during the OFF time.
    pub fn new(
        reader: u8,
        led: u8,
        temporary_control_code: LEDTemporaryControlCode,
        temporary_on_time: u8,
        temporary_off_time: u8,
        temporary_on_color: LEDColor,
        temporary_off_color: LEDColor,
        temporary_timer: u16,
        permanent_control_code: LEDPermanentControlCode,
        permanent_on_time: u8,
        permanent_off_time: u8,
        permanent_on_color: LEDColor,
        permanent_off_color: LEDColor,
    ) -> Self {
        Self {
            reader,
            led,
            temporary_control_code,
            temporary_on_time,
            temporary_off_time,
            temporary_on_color,
            temporary_off_color,
            temporary_timer,
            permanent_control_code,
            permanent_on_time,
            permanent_off_time,
            permanent_on_color,
            permanent_off_color,
        }
    }
}

/// `osdp_LED`
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ReaderLEDControl<'a> {
    pub led_states: &'a [ReaderLEDControlState],
}

impl<'a> ReaderLEDControl<'a> {
    /// # Arguments
    ///
    /// * `led_states` - The LED states to control.
    pub fn new(led_states: &'a [ReaderLEDControlState]) -> Result<Self, InvalidLengthError> {
        if led_states.len() == 0 {
            return Err(InvalidLengthError::new_minimum(1));
        }

        Ok(Self { led_states })
    }
}
