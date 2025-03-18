use std::net::{Ipv4Addr, Ipv6Addr};
use etherparse::NetSlice;

/// This trait for convert 'NetSlice' to string format
/// This trait try to make easier get any ip from 'NetSlice'
/// with only one method.
pub trait GetIp {
    fn ipv4_dst(&self) -> Result<Ipv4Addr, String>;
    fn ipv4_src(&self) -> Result<Ipv4Addr, String>;
    fn ipv6_des(&self) -> Result<Ipv6Addr, String>;
    fn ipv6_src(&self) -> Result<Ipv6Addr, String>;
}

impl<'a> GetIp for NetSlice<'a> {
    fn ipv6_des(&self) -> Result<Ipv6Addr, String> {
        if let NetSlice::Ipv6(ip) = self {
            return Ok(ip.header().source_addr());
        } else {
            return Err("It is not ipv6".into());
        }
    }
    fn ipv6_src(&self) -> Result<Ipv6Addr, String> {
        if let NetSlice::Ipv6(ip) = self {
            return Ok(ip.header().source_addr());
        } else {
            return Err("It is not ipv6".into());
        }
    }

    fn ipv4_dst(&self) -> Result<Ipv4Addr, String> {
        if let NetSlice::Ipv4(ip) = self {
            return Ok(ip.header().destination_addr());
        } else {
            return Err("It is not ipv4".into());
        }
    }

    fn ipv4_src(&self) -> Result<Ipv4Addr, String> {
        if let NetSlice::Ipv4(ip) = self {
            return Ok(ip.header().destination_addr());
        } else {
            return Err("It is not ipv4".into());
        }
    }
}
