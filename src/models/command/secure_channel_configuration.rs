/// Encryption method to use with the key.
#[derive(Default, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum KeyType {
    #[default]
    SecureChannelBaseKey = 0x01,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct EncryptionKeyConfiguration<'a> {
    /// The key type.
    pub key_type: KeyType,
    /// The key data.
    pub key_data: &'a [u8],
}

impl<'a> EncryptionKeyConfiguration<'a> {
    /// # Arguments
    ///
    /// * `key_type` - The key type.
    /// * `key_data` - The key data.
    pub fn new(key_type: KeyType, key_data: &'a [u8]) -> Self {
        Self { key_type, key_data }
    }
}
