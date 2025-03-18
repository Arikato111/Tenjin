use super::ToU32;

/// This function checks if specified position of a bit of u32 is one.
pub fn bit_bool<T: ToU32>(position: u16, byte: T) -> bool {
    (byte.to_u32() >> position) & 1 == 1
}

/// This function changes the value at the specified bit position of u32 to one.
pub fn set_bit(byte: u32, position: u32, set: bool) -> u32 {
    if set {
        byte | (1 << position)
    } else {
        byte & !(1 << position)
    }
}
