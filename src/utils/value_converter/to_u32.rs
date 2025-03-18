/// Convert any value to u32 type.
pub trait ToU32 {
    fn to_u32(&self) -> u32;
}
impl ToU32 for u32 {
    fn to_u32(&self) -> u32 {
        *self as u32
    }
}

impl ToU32 for u16 {
    fn to_u32(&self) -> u32 {
        *self as u32
    }
}
