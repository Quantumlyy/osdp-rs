pub fn calculate_checksum(packet: &[u8]) -> u8 {
    let sum: u16 = packet.iter().map(|&byte| byte as u16).sum();
    (!(sum & 0xFF) + 1) as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_checksum_test() {
        assert_eq!(
            0x0F,
            calculate_checksum(&[
                0x53, // SOM
                0x7F, // addr
                0x0C, // len LSB
                0x00, // len MSB
                0x00, // control
                0x6E, // cmd
                0x00, // data
                0x80, // data
                0x25, // data
                0x00, // data
                0x00
            ])
        );

        assert_eq!(
            0x44,
            calculate_checksum(&[
                0x53, // SOM
                0x00, // addr
                0x08, // len LSB
                0x00, // len MSB
                0x00, // control
                0x61, // cmd
                0x00
            ])
        );
    }
}
