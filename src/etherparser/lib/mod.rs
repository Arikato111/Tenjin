// private
mod ethernet_impl;
mod ip_impl;
mod log;
mod to_u64_impl;

// public
pub use ethernet_impl::GetMacAddr;
pub use ip_impl::GetIp;
pub use log::Log;
pub use to_u64_impl::ToU64;
