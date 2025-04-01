//! Value conversion utilities for type-safe conversions between different numeric types.
//!
//! This module provides traits and functions for converting between different numeric types,
//! particularly focusing on bit-level operations and conversions to u32 and u64 types.
//!
//! # Features
//!
//! - Bit-level operations (checking and setting bits)
//! - Type-safe conversions to u32 for various numeric types
//! - Conversion of byte arrays to u64 values
//!
//! # Examples
//!
//! ```rust
//! use tenjin_sdn::utils::value_converter::{ToU32, ToU64, bit_bool, set_bit};
//!
//! // Convert u16 to u32
//! let value: u16 = 42;
//! let converted: u32 = value.to_u32();
//!
//! // Check if a bit is set
//! let is_set = bit_bool(3, 0b1000_u32);
//!
//! // Set or clear a bit
//! let result = set_bit(0b0000, 2, true); // Sets bit 2 to 1
//! ```

// private
mod bits;
mod to_u32;
mod to_u64;

// public
pub use bits::{bit_bool, set_bit};
pub use to_u32::ToU32;
pub use to_u64::ToU64;
