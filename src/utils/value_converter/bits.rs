use super::ToU32;

/// Checks if a specific bit position in a value is set to 1.
///
/// # Arguments
/// * `position` - The bit position to check (0-based index)
/// * `byte` - The value to check (must implement ToU32 trait)
///
/// # Returns
/// * `bool` - true if the bit is set to 1, false otherwise
///
/// # Examples
/// ```
/// use tenjin_sdn::utils::value_converter::bit_bool;
///
/// let value: u32 = 0b1000;
/// assert!(bit_bool(3, value));  // Bit 3 is set
/// assert!(!bit_bool(2, value)); // Bit 2 is not set
/// ```
pub fn bit_bool<T: ToU32>(position: u16, byte: T) -> bool {
    (byte.to_u32() >> position) & 1 == 1
}

/// Sets or clears a specific bit position in a u32 value.
///
/// # Arguments
/// * `byte` - The original u32 value to modify
/// * `position` - The bit position to modify (0-based index)
/// * `set` - true to set the bit to 1, false to clear it to 0
///
/// # Returns
/// * `u32` - The modified value with the specified bit set or cleared
///
/// # Examples
/// ```
/// use tenjin_sdn::utils::value_converter::set_bit;
///
/// let value = 0b0000;
/// let result = set_bit(value, 2, true);  // Sets bit 2 to 1
/// assert_eq!(result, 0b0100);
///
/// let result = set_bit(result, 2, false); // Clears bit 2 to 0
/// assert_eq!(result, 0b0000);
/// ```
pub fn set_bit(byte: u32, position: u32, set: bool) -> u32 {
    if set {
        byte | (1 << position)
    } else {
        byte & !(1 << position)
    }
}
