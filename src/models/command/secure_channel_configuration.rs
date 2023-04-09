/// Encryption method to use with the key.
#[derive(Default)]
#[repr(u8)]
pub enum KeyType {
    #[default]
    SecureChannelBaseKey = 0x01,
}
