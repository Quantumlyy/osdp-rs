/// Encryption method to use with the key.
#[derive(Default, Clone, Copy)]
#[repr(u8)]
pub enum KeyType {
    #[default]
    SecureChannelBaseKey = 0x01,
}
