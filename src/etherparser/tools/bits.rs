pub trait NumberTrait {
    fn get_value(&self) -> u32;
}
impl NumberTrait for u32 {
    fn get_value(&self) -> u32 {
        *self as u32
    }
}

impl NumberTrait for u16 {
    fn get_value(&self) -> u32 {
        *self as u32
    }
}

pub fn bit_bool<T: NumberTrait>(position: u16, byte: T) -> bool {
    (byte.get_value() >> position) & 1 == 1
}

pub fn set_bit(byte: u32, position: u32, set: bool) -> u32 {
    if set {
        byte | (1 << position)
    } else {
        byte & !(1 << position)
    }
}

pub fn mac_to_bytes(byte: [u8; 6]) -> u64 {
    let mut addr = [0u8; 8];
    for i in 2..8 {
        addr[i] = byte[i - 2];
    }
    u64::from_be_bytes(addr)
}

pub fn bytes_to_mac(bytes: u64) -> [u8; 6] {
    let mut address = [0; 6];
    for i in 0..6 {
        address[i] = ((bytes >> (8 * i)) & 0xff) as u8;
    }
    address.reverse();
    address
}
