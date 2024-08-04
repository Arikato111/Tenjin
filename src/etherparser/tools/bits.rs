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
