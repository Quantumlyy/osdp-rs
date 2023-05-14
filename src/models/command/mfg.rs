pub struct ManufacturerSpecificCommand<'a> {
    /// IEEE assigned OUI, "first octet".
    pub vendor_code_1: u8,
    /// IEEE assigned OUI, "second octet".
    pub vendor_code_2: u8,
    /// IEEE assigned OUI, "third octet".
    pub vendor_code_3: u8,
    /// Defined by the vendor/manufacturer.
    pub data: &'a [u8],
}

impl<'a> ManufacturerSpecificCommand<'a> {
    /// # Arguments
    ///
    /// * `vendor_code_1` - IEEE assigned OUI, "first octet".
    /// * `vendor_code_2` - IEEE assigned OUI, "second octet".
    /// * `vendor_code_3` - IEEE assigned OUI, "third octet".
    /// * `data` - Defined by the vendor/manufacturer.
    pub fn new(vendor_code_1: u8, vendor_code_2: u8, vendor_code_3: u8, data: &'a [u8]) -> Self {
        Self {
            vendor_code_1,
            vendor_code_2,
            vendor_code_3,
            data,
        }
    }
}
