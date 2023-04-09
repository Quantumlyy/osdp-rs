#[derive(Default, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum BiometricFormat {
    /// Default method to scan.
    #[default]
    NotSpecified = 0x00,
    /// Send raw fingerprint data as PGM.
    RawFingerprintData = 0x01,
    /// ANSI/INCITS 378 fingerprint template.
    FingerPrintTemplate = 0x02,
}

#[derive(Default, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum BiometricType {
    #[default]
    NotSpecified = 0x00,
    RightThumbPrint = 0x01,
    RightIndexFingerPrint = 0x02,
    RightMiddleFingerPrint = 0x03,
    RightRingFingerPrint = 0x04,
    RightLittleFingerPrint = 0x05,
    LeftThumbPrint = 0x06,
    LeftIndexFingerPrint = 0x07,
    LeftMiddleFingerPrint = 0x08,
    LeftRingFingerPrint = 0x09,
    LeftLittleFingerPrint = 0x0A,
    RightIrisScan = 0x0B,
    RightRetinaScan = 0x0C,
    LeftIrisScan = 0x0D,
    LeftRetinaScan = 0x0E,
    FullFaceImage = 0x0F,
    RightHandGeometry = 0x10,
    LeftHandGeometry = 0x11,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct BiometricRead {
    /// The reader number.
    pub reader: u8,
    /// The type/body part.
    pub biometric_type: BiometricType,
    /// The format of the attached template.
    pub biometric_format: BiometricFormat,
    /// The quality value normalised between `0x00` and `0xFF`.
    pub biometric_quality: u8,
}

impl BiometricRead {
    /// # Arguments
    ///
    /// * `reader` - The reader number starting at 0.
    /// * `biometric_type` - The type/body part to scan.
    /// * `biometric_format` - The format of the attached template.
    /// * `biometric_quality` - The quality value normalised between `0x00` and `0xFF`.
    pub fn new(
        reader: u8,
        biometric_type: BiometricType,
        biometric_format: BiometricFormat,
        biometric_quality: u8,
    ) -> Self {
        Self {
            reader,
            biometric_type,
            biometric_format,
            biometric_quality,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct BiometricMatch<'a> {
    /// The reader number.
    pub reader: u8,
    /// The type/body part.
    pub biometric_type: BiometricType,
    /// The format of the attached template.
    pub biometric_format: BiometricFormat,
    /// The quality value normalised between `0x00` and `0xFF`.
    pub biometric_quality: u8,
    /// The template data.
    pub biometric_template_data: &'a [u8],
}

impl<'a> BiometricMatch<'a> {
    /// # Arguments
    ///
    /// * `reader` - The reader number starting at 0.
    /// * `biometric_type` - The type/body part to scan.
    /// * `biometric_format` - The format of the attached template.
    /// * `biometric_quality` - The quality value normalised between `0x00` and `0xFF`.
    /// * `biometric_template_data` - The template data.
    pub fn new(
        reader: u8,
        biometric_type: BiometricType,
        biometric_format: BiometricFormat,
        biometric_quality: u8,
        biometric_template_data: &'a [u8],
    ) -> Self {
        Self {
            reader,
            biometric_type,
            biometric_format,
            biometric_quality,
            biometric_template_data,
        }
    }
}
