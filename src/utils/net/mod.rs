//! Network utilities module for handling Ethernet and IP packet processing.
//!
//! This module provides abstractions and utilities for working with network packets,
//! particularly focusing on Ethernet frames and IP packets. It includes:
//!
//! - MAC address handling and manipulation
//! - Ethernet frame parsing and processing
//! - IP address extraction and validation
//!
//! The module integrates with the `etherparse` crate to provide a more ergonomic
//! interface for working with network packets.

// private
mod ethernet_impl;
mod ip_impl;
mod mac_address;

// public
pub use ethernet_impl::GetMacAddr;
pub use ip_impl::GetIp;
pub use mac_address::MacAddr;
