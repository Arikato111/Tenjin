//! This module for convert any value to others value,
//! such as Array to u64 by impl `ToU64` trait to Array type.
// private
mod bits;
mod to_u32;
mod to_u64;

// public
pub use bits::{bit_bool, set_bit};
pub use to_u32::ToU32;
pub use to_u64::ToU64;
