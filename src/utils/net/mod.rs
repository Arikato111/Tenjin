//! The `net` module is the module used for Ethernet related things like Ethernet IP frames.
//! This module includes trait that work with `Etherparse`,
//! `MacAddr` struct for managing Mac address.
//! 

// private
mod ethernet_impl;
mod ip_impl;
mod mac_address;

// public
pub use ethernet_impl::GetMacAddr;
pub use ip_impl::GetIp;
pub use mac_address::MacAddr;
