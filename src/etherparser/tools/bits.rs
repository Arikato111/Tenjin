pub fn bit_bool(position: u16, byte: u16) -> bool {
    (byte >> position) & 1 == 1
}

pub fn mac_to_bytes(byte: [u8; 6]) -> u64 {
    let mut addr = [0u8; 8];
    for i in 2..8 {
        addr[i] = byte[i - 2];
    }
    u64::from_be_bytes(addr)
}
