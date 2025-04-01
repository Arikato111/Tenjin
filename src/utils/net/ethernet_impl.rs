use etherparse::{Ethernet2Header, LinkSlice};

/// A trait for extracting MAC addresses from etherparse's LinkSlice.
/// 
/// This trait provides a simplified interface for getting MAC addresses from network packets,
/// abstracting away the complexity of dealing with multiple header types and optional values.
pub trait GetMacAddr {
    /// Extracts the Ethernet II header containing MAC addresses from the packet.
    /// 
    /// # Returns
    /// * `Result<Ethernet2Header, String>` - The Ethernet II header if found, or an error message if not found
    fn macs(&self) -> Result<Ethernet2Header, String>;
}

/// Implementation of GetMacAddr for LinkSlice to extract MAC addresses from raw packet data.
impl<'a> GetMacAddr for LinkSlice<'a> {
    /// Extracts the Ethernet II header from the packet slice.
    /// 
    /// # Returns
    /// * `Result<Ethernet2Header, String>` - The Ethernet II header if found, or an error if not found
    fn macs(&self) -> Result<Ethernet2Header, String> {
        if let Some(header) = self.to_header() {
            if let Some(link_header) = header.ethernet2() {
                return Ok(link_header);
            }
        }
        return Err("Not found Mac".into());
    }
}

/// Implementation of GetMacAddr for Option<LinkSlice> to handle optional packet slices.
/// 
/// This implementation reduces the need for multiple unwrap() calls when working with
/// optional packet slices, providing a more ergonomic interface.
impl<'a> GetMacAddr for Option<LinkSlice<'a>> {
    /// Extracts the Ethernet II header from an optional packet slice.
    /// 
    /// # Returns
    /// * `Result<Ethernet2Header, String>` - The Ethernet II header if found, or an error if not found
    fn macs(&self) -> Result<Ethernet2Header, String> {
        match self {
            Some(mac) => mac.macs(),
            None => Err(String::from("Mac not found")),
        }
    }
}
