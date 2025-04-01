/// A trait for converting values to u32 type.
///
/// This trait provides a unified interface for converting various numeric types to u32.
/// It is particularly useful when working with bit-level operations that require u32 values.
///
/// # Examples
/// ```
/// use tenjin_sdn::utils::value_converter::ToU32;
///
/// let value: u16 = 42;
/// let converted: u32 = value.to_u32();
/// assert_eq!(converted, 42);
/// ```
pub trait ToU32 {
    /// Converts the value to u32.
    ///
    /// # Returns
    /// * `u32` - The converted value
    fn to_u32(&self) -> u32;
}

/// Implementation of ToU32 for u32 values.
///
/// This is a simple pass-through conversion since the value is already u32.
impl ToU32 for u32 {
    fn to_u32(&self) -> u32 {
        *self as u32
    }
}

/// Implementation of ToU32 for u16 values.
///
/// Converts u16 to u32 with zero extension.
impl ToU32 for u16 {
    fn to_u32(&self) -> u32 {
        *self as u32
    }
}
