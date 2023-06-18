# osdp-rs

`osdp-rs` is a Rust implementation of the Open Supervised Device Protocol (OSDP).

[![Crates.io](https://img.shields.io/crates/v/osdp)](https://crates.io/crates/osdp)
[![Documentation](https://docs.rs/osdp/badge.svg)](https://docs.rs/osdp)

# Implementation

## Commands

### Command data build

 - [x] osdp_POLL
 - [x] osdp_ID
 - [x] osdp_CAP
 - [x] osdp_LSTAT
 - [x] osdp_ISTAT
 - [x] osdp_OSTAT
 - [x] osdp_RSTAT
 - [x] osdp_OUT
 - [x] osdp_LED
 - [x] osdp_BUZ
 - [ ] osdp_TEXT
 - [x] osdp_COMSET
 - [x] osdp_BIOREAD
 - [x] osdp_BIOMATCH
 - [x] osdp_KEYSET
 - [ ] osdp_CHLNG
 - [ ] osdp_SCRYPT
 - [x] osdp_MFG
 - [x] osdp_ACURXSIZE
 - [x] osdp_KEEPACTIVE
 - [x] osdp_ABORT
 - [ ] osdp_PIVDATA
 - [ ] osdp_GENAUTH
 - [ ] osdp_CRAUTH
 - [ ] osdp_FILETRANSFER
 - [ ] osdp_XWR

## Replies

### Reply data parsing

 - [x] osdp_ACK
 - [x] osdp_NAK
 - [x] osdp_PDID
 - [x] osdp_PDCAP
 - [x] osdp_LSTATR
 - [ ] osdp_ISTATR
 - [ ] osdp_OSTATR
 - [x] osdp_RAW
 - [ ] osdp_FMT
 - [ ] osdp_KEYPAD
 - [ ] osdp_COM
 - [ ] osdp_BIOREADR
 - [ ] osdp_BIOMATCHR
 - [ ] osdp_CCRYPT
 - [ ] osdp_RMAC_I
 - [ ] osdp_MFGREP
 - [ ] osdp_BUSY
 - [ ] osdp_PIVDATAR
 - [ ] osdp_GENAUTHR
 - [ ] osdp_MFGSTATR
 - [ ] osdp_MFGERRR
 - [ ] osdp_FSTAT
 - [ ] osdp_XRD
