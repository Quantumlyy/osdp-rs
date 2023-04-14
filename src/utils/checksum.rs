pub fn calculate_checksum(packet: &Vec<u8>) -> u8 {
    let sum: u16 = packet.iter().map(|&byte| byte as u16).sum();
    ((0x100 - sum) & 0xFF) as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_checksum_test() {
        assert_eq!(
            0x44,
            calculate_checksum(&vec![
                0x53, // SOM
                0x00, // addr
                0x08, // len LSB
                0x00, // len MSB
                0x00, // control
                0x61, // cmd
                0x00, // data
                0x00, // checksum
            ])
        );
    }
}
