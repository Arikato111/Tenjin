//! Utility modules for the OpenFlow controller implementation
//! 
//! This module provides various utility functions and traits that support the core
//! OpenFlow controller functionality. It includes networking utilities, logging capabilities,
//! and value conversion helpers.

/// Network-related utilities
/// 
/// Provides implementations for handling network-related data types including:
/// - MAC addresses
/// - Ethernet frames
/// - IP packets
pub mod net;
pub use net::MacAddr;

/// Value conversion utilities
/// 
/// Provides traits and implementations for converting between different numeric types
/// and bit-level operations, particularly useful for OpenFlow protocol handling.
pub mod value_converter;

/// Logging utilities
/// 
/// Provides logging functionality and formatting for network packets and other data types.
/// Includes the `Log` trait for consistent string representation of various types.
pub mod log;
pub use value_converter::*;
