//! Not all commands can are implemented by all parties that use OSDP.
//! Below is a compatibility table for `OSDP v2.2`, `iClass HADP`, `iClass SE OSDP v1`, and `iClass SE OSDP v2`.
//!
//! - `✅` = Fully implemented.
//! - `❌` = Not implemented, not applicable, or not enabled.
//!
//! | Command Name        | Command Value | OSDP v2.2   | iClass HADP   | iClass SE OSDP v1   | iClass SE OSDP v2   |
//! |---------------------|---------------|-------------|---------------|---------------------|---------------------|
//! | `osdp_POLL`         | `0x60`        | ✅          | ✅             | ✅                  | ✅                  |
//! | `osdp_ID`           | `0x61`        | ✅          | ✅             | ✅                  | ✅                  |
//! | `osdp_CAP`          | `0x62`        | ✅          | ✅             | ✅                  | ✅                  |
//! | `osdp_DIAG`         | `0x63`        | ❌          | ❌             | ❌                  | ❌                  |
//! | `osdp_LSTAT`        | `0x64`        | ✅          | ✅             | ✅                  | ✅                  |
//! | `osdp_ISTAT`        | `0x65`        | ✅          | ❌             | ✅                  | ✅                  |
//! | `osdp_OSTAT`        | `0x66`        | ✅          | ❌             | ✅                  | ✅                  |
//! | `osdp_RSTAT`        | `0x67`        | ✅          | ❌             | ❌                  | ❌                  |
//! | `osdp_OUT`          | `0x68`        | ✅          | ❌             | ✅                  | ✅                  |
//! | `osdp_LED`          | `0x69`        | ✅          | ✅             | ✅                  | ✅                  |
//! | `osdp_BUZ`          | `0x6A`        | ✅          | ✅             | ✅                  | ✅                  |
//! | `osdp_TEXT`         | `0x6B`        | ✅          | ✅             | ✅                  | ✅                  |
//! | `osdp_RMODE`        | `0x6C`        | ❌          | ❌             | ❌                  | ❌                  |
//! | `osdp_TDSET`        | `0x6D`        | ❌          | ❌             | ❌                  | ❌                  |
//! | `osdp_COMSET`       | `0x6E`        | ✅          | ✅             | ✅                  | ✅                  |
//! | `osdp_DATA`         | `0x6F`        | ❌          | ❌             | ❌                  | ❌                  |
//! | `osdp_XMIT`         | `0x70`        | ❌          | ❌             | ❌                  | ❌                  |
//! | `osdp_PROMPT`       | `0x71`        | ❌          | ❌             | ❌                  | ❌                  |
//! | `osdp_SPE`          | `0x72`        | ❌          | ❌             | ❌                  | ❌                  |
//! | `osdp_BIOREAD`      | `0x73`        | ✅          | ❌             | ❌                  | ✅                  |
//! | `osdp_BIOMATCH`     | `0x74`        | ✅          | ❌             | ❌                  | ✅                  |
//! | `osdp_KEYSET`       | `0x75`        | ✅          | ❌             | ❌                  | ✅                  |
//! | `osdp_CHLNG`        | `0x76`        | ✅          | ❌             | ❌                  | ✅                  |
//! | `osdp_SCRYPT`       | `0x77`        | ✅          | ❌             | ❌                  | ✅                  |
//! | `osdp_CONT`         | `0x79`        | ❌          | ❌             | ❌                  | ❌                  |
//! | `osdp_ACURXSIZE`    | `0x7B`        | ✅          | ❌             | ❌                  | ❌                  |
//! | `osdp_FILETRANSFER` | `0x7C`        | ✅          | ❌             | ❌                  | ❌                  |
//! | `osdp_MFG`          | `0x80`        | ✅          | ✅             | ✅                  | ✅                  |
//! | `osdp_SCDONE`       | `0xA0`        | ❌          | ❌             | ❌                  | ❌                  |
//! | `osdp_XWR`          | `0xA1`        | ✅          | ❌             | ❌                  | ✅                  |
//! | `osdp_ABORT`        | `0xA2`        | ✅          | ❌             | ❌                  | ❌                  |
//! | `osdp_PIVDATA`      | `0xA3`        | ✅          | ❌             | ❌                  | ❌                  |
//! | `osdp_GENAUTH`      | `0xA4`        | ✅          | ❌             | ❌                  | ❌                  |
//! | `osdp_CRAUTH`       | `0xA5`        | ✅          | ❌             | ❌                  | ❌                  |
//! | `osdp_KEEPACTIVE`   | `0xA7`        | ✅          | ❌             | ❌                  | ❌                  |

pub mod abort;
pub mod acu_receive_size;
pub mod biometric;
pub mod buzzer;
pub mod keep_reader_active;
pub mod output_control;
pub mod peripheral_device_capabilities;
pub mod poll;
pub mod report;
pub mod secure_channel_configuration;
