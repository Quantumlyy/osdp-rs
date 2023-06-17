use bitvec::vec::BitVec;

/// `osdp_RAW`
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CardRawDataReport {
    pub reader_number: u8,
    pub format_code: u8,
    pub bit_count: u16,
    pub data: BitVec<u8>,
}

impl CardRawDataReport {
    pub fn new(reader_number: u8, format_code: u8, bit_count: u16, data: BitVec<u8>) -> Self {
        Self {
            reader_number,
            format_code,
            bit_count,
            data,
        }
    }
}
