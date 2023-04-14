//! Not all commands can are implemented by all parties that use OSDP.
//! Below is a compatibility table for `OSDP v2.2`, `iClass HADP`, `iClass SE OSDP v1`, and `iClass SE OSDP v2`.
//! 
//! `[x]` = Fully implemented.
//! `[ ]` = Not implemented, not applicable, or not enabled.
//! `[?]` = Unknown.
//! | Command Name        | Command Value | OSDP v2.2 | iClass HADP | iClass SE OSDP v1 | iClass SE OSDP v2 |
//! |---------------------|---------------|-----------|-------------|-------------------|-------------------|
//! | `osdp_POLL`         | `0x60`        | [x]       | [x]         | [x]               | [x]               |
//! | `osdp_ID`           | `0x61`        | [x]       | [x]         | [x]               | [x]               |
//! | `osdp_CAP`          | `0x62`        | [x]       | [x]         | [x]               | [x]               |
//! | `osdp_DIAG`         | `0x63`        | [ ]       | [ ]         | [ ]               | [ ]               |
//! | `osdp_LSTAT`        | `0x64`        | [x]       | [x]         | [x]               | [x]               |
//! | `osdp_ISTAT`        | `0x65`        | [x]       | [ ]         | [x]               | [x]               |
//! | `osdp_OSTAT`        | `0x66`        | [x]       | [ ]         | [x]               | [x]               |
//! | `osdp_RSTAT`        | `0x67`        | [x]       | [ ]         | [ ]               | [ ]               |
//! | `osdp_OUT`          | `0x68`        | [x]       | [ ]         | [x]               | [x]               |
//! | `osdp_LED`          | `0x69`        | [x]       | [x]         | [x]               | [x]               |
//! | `osdp_BUZ`          | `0x6A`        | [x]       | [x]         | [x]               | [x]               |
//! | `osdp_TEXT`         | `0x6B`        | [x]       | [x]         | [x]               | [x]               |
//! | `osdp_RMODE`        | `0x6C`        | [ ]       | [ ]         | [ ]               | [ ]               |
//! | `osdp_TDSET`        | `0x6D`        | [ ]       | [ ]         | [ ]               | [ ]               |
//! | `osdp_COMSET`       | `0x6E`        | [x]       | [x]         | [x]               | [x]               |
//! | `osdp_DATA`         | `0x6F`        | [ ]       | [ ]         | [ ]               | [ ]               |
//! | `osdp_XMIT`         | `0x70`        | [ ]       | [ ]         | [ ]               | [ ]               |
//! | `osdp_PROMPT`       | `0x71`        | [ ]       | [ ]         | [ ]               | [ ]               |
//! | `osdp_SPE`          | `0x72`        | [ ]       | [ ]         | [ ]               | [ ]               |
//! | `osdp_BIOREAD`      | `0x73`        | [x]       | [ ]         | [ ]               | [x]               |
//! | `osdp_BIOMATCH`     | `0x74`        | [x]       | [ ]         | [ ]               | [x]               |
//! | `osdp_KEYSET`       | `0x75`        | [x]       | [ ]         | [ ]               | [x]               |
//! | `osdp_CHLNG`        | `0x76`        | [x]       | [ ]         | [ ]               | [x]               |
//! | `osdp_SCRYPT`       | `0x77`        | [x]       | [ ]         | [ ]               | [x]               |
//! | `osdp_CONT`         | `0x79`        | [ ]       | [ ]         | [ ]               | [ ]               |
//! | `osdp_ACURXSIZE`    | `0x7B`        | [x]       | [ ]         | [ ]               | [ ]               |
//! | `osdp_FILETRANSFER` | `0x7C`        | [x]       | [ ]         | [ ]               | [ ]               |
//! | `osdp_MFG`          | `0x80`        | [x]       | [x]         | [x]               | [x]               |
//! | `osdp_SCDONE`       | `0xA0`        | [ ]       | [ ]         | [ ]               | [ ]               |
//! | `osdp_XWR`          | `0xA1`        | [x]       | [ ]         | [ ]               | [x]               |
//! | `osdp_ABORT`        | `0xA2`        | [x]       | [ ]         | [ ]               | [ ]               |
//! | `osdp_PIVDATA`      | `0xA3`        | [x]       | [ ]         | [ ]               | [ ]               |
//! | `osdp_GENAUTH`      | `0xA4`        | [x]       | [ ]         | [ ]               | [ ]               |
//! | `osdp_CRAUTH`       | `0xA5`        | [x]       | [ ]         | [ ]               | [ ]               |
//! | `osdp_KEEPACTIVE`   | `0xA7`        | [x]       | [ ]         | [ ]               | [ ]               |

pub mod abort;
pub mod acu_receive_size;
pub mod biometric;
pub mod buzzer;
pub mod keep_reader_active;
pub mod output_control;
pub mod peripheral_device_capabilities;
pub mod report;
pub mod poll;
pub mod secure_channel_configuration;
