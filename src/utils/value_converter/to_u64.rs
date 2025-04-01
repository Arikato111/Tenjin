/// A trait for converting values to u64 type.
/// 
/// This trait provides a unified interface for converting various types to u64.
/// It is particularly useful when working with large numeric values or byte arrays
/// that need to be converted to a single u64 value.
/// 
/// # Examples
/// ```
/// use tenjin_sdn::utils::value_converter::ToU64;
/// 
/// let bytes: [u8; 6] = [0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC];
/// let value: u64 = bytes.to_u64();
/// ```
pub trait ToU64 {
    /// Converts the value to u64.
    /// 
    /// # Returns
    /// * `u64` - The converted value
    fn to_u64(&self) -> u64;
}

/// Implementation of ToU64 for 6-byte arrays.
/// 
/// Converts a 6-byte array to u64 by treating the bytes as a big-endian number.
/// The bytes are packed into the u64 with the first byte becoming the most significant byte.
/// 
/// # Examples
/// ```
/// use tenjin_sdn::utils::value_converter::ToU64;
/// 
/// let mac: [u8; 6] = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55];
/// let value = mac.to_u64();
/// assert_eq!(value, 0x001122334455);
/// ```
impl ToU64 for [u8; 6] {
    fn to_u64(&self) -> u64 {
        let mut result: u64 = 0;
        for i in self {
            result = result << 8;
            result = result | (*i as u64);
        }
        return result;
    }
}
