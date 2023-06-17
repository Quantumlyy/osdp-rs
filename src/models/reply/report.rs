/// `osdp_PDID`
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DeviceIdentificationReport {
    /// IEEE assigned OUI, "first octet".
    pub vendor_code_1: u8,
    /// IEEE assigned OUI, "second octet".
    pub vendor_code_2: u8,
    /// IEEE assigned OUI, "third octet".
    pub vendor_code_3: u8,
    /// Model number assigned by the vendor.
    pub model_number: u8,
    /// Manufacturer's version of the product.
    pub version: u8,
    /// Serial number assigned and managed by the vendor.
    pub serial_number: u32,
    /// Firmware revision code
    ///
    /// The firmware revision fields are assigned and managed by the vendor.
    ///
    /// `firmware_revision_code` is a 3 byte array containing the major, minor, and build numbers.
    pub firmware_revision_code: [u8; 3],
}

impl DeviceIdentificationReport {
    /// # Arguments
    ///
    /// * `vendor_code_1` - IEEE assigned OUI, "first octet".
    /// * `vendor_code_2` - IEEE assigned OUI, "second octet".
    /// * `vendor_code_3` - IEEE assigned OUI, "third octet".
    /// * `model_number` - Model number assigned by the vendor.
    /// * `version` - Manufacturer's version of the product.
    /// * `serial_number` - Serial number assigned and managed by the vendor.
    /// * `firmware_revision_code` - Firmware revision code as a 3 byte array containing the major, minor, and build numbers.
    pub fn new(
        vendor_code_1: u8,
        vendor_code_2: u8,
        vendor_code_3: u8,
        model_number: u8,
        version: u8,
        serial_number: u32,
        firmware_revision_code: [u8; 3],
    ) -> Self {
        Self {
            vendor_code_1,
            vendor_code_2,
            vendor_code_3,
            model_number,
            version,
            serial_number,
            firmware_revision_code,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DeviceCapability {
    /// Function code
    pub function_code: u8,
    /// Compliance level
    pub compliance: u8,
    /// Number of
    pub number_of: u8,
}

/// `osdp_PDCAP`
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DeviceCapabilitiesReport {
    pub capabilities: Vec<DeviceCapability>,
}

impl DeviceCapabilitiesReport {
    /// # Arguments
    ///
    /// * `capabilities` - The device capabilities.
    pub fn new(capabilities: Vec<DeviceCapability>) -> Self {
        Self { capabilities }
    }
}

/// `osdp_LSTATR`
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LocalStatusReport {
    /// Status of the tamper circuit
    ///
    /// - `0x00` - normal
    /// - `0x01` - tamper
    pub tamper: u8,
    /// Status of the power circuit
    ///
    /// - `0x00` - normal
    /// - `0x01` - power failure
    pub power: u8,
}

impl LocalStatusReport {
    /// # Arguments
    ///
    /// * `tamper` - Status of the tamper circuit
    /// * `power` - Status of the power circuit
    pub fn new(tamper: u8, power: u8) -> Self {
        Self { tamper, power }
    }
}
