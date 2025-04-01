use std::net::{Ipv4Addr, Ipv6Addr};
use etherparse::NetSlice;

/// A trait for extracting IP addresses from etherparse's NetSlice.
/// 
/// This trait provides a simplified interface for getting source and destination IP addresses
/// from network packets, supporting both IPv4 and IPv6 protocols.
pub trait GetIp {
    /// Extracts the IPv4 destination address from the packet.
    /// 
    /// # Returns
    /// * `Result<Ipv4Addr, String>` - The IPv4 destination address if found, or an error if not found
    fn ipv4_dst(&self) -> Result<Ipv4Addr, String>;
    
    /// Extracts the IPv4 source address from the packet.
    /// 
    /// # Returns
    /// * `Result<Ipv4Addr, String>` - The IPv4 source address if found, or an error if not found
    fn ipv4_src(&self) -> Result<Ipv4Addr, String>;
    
    /// Extracts the IPv6 destination address from the packet.
    /// 
    /// # Returns
    /// * `Result<Ipv6Addr, String>` - The IPv6 destination address if found, or an error if not found
    fn ipv6_dst(&self) -> Result<Ipv6Addr, String>;
    
    /// Extracts the IPv6 source address from the packet.
    /// 
    /// # Returns
    /// * `Result<Ipv6Addr, String>` - The IPv6 source address if found, or an error if not found
    fn ipv6_src(&self) -> Result<Ipv6Addr, String>;
}

/// Implementation of GetIp for NetSlice to extract IP addresses from raw packet data.
impl<'a> GetIp for NetSlice<'a> {
    /// Extracts the IPv6 destination address from the packet.
    /// 
    /// # Returns
    /// * `Result<Ipv6Addr, String>` - The IPv6 destination address if found, or an error if not IPv6
    fn ipv6_dst(&self) -> Result<Ipv6Addr, String> {
        if let NetSlice::Ipv6(ip) = self {
            return Ok(ip.header().source_addr());
        } else {
            return Err("It is not ipv6".into());
        }
    }

    /// Extracts the IPv6 source address from the packet.
    /// 
    /// # Returns
    /// * `Result<Ipv6Addr, String>` - The IPv6 source address if found, or an error if not IPv6
    fn ipv6_src(&self) -> Result<Ipv6Addr, String> {
        if let NetSlice::Ipv6(ip) = self {
            return Ok(ip.header().source_addr());
        } else {
            return Err("It is not ipv6".into());
        }
    }

    /// Extracts the IPv4 destination address from the packet.
    /// 
    /// # Returns
    /// * `Result<Ipv4Addr, String>` - The IPv4 destination address if found, or an error if not IPv4
    fn ipv4_dst(&self) -> Result<Ipv4Addr, String> {
        if let NetSlice::Ipv4(ip) = self {
            return Ok(ip.header().destination_addr());
        } else {
            return Err("It is not ipv4".into());
        }
    }

    /// Extracts the IPv4 source address from the packet.
    /// 
    /// # Returns
    /// * `Result<Ipv4Addr, String>` - The IPv4 source address if found, or an error if not IPv4
    fn ipv4_src(&self) -> Result<Ipv4Addr, String> {
        if let NetSlice::Ipv4(ip) = self {
            return Ok(ip.header().destination_addr());
        } else {
            return Err("It is not ipv4".into());
        }
    }
}
