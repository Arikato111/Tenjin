/// Convert any value to u64.
pub trait ToU64 {
    fn to_u64(&self) -> u64;
}
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
