use etherparse::{Ethernet2Header, LinkSlice};

/// This trait makes easier get mac addr from etherparse
pub trait GetMacAddr {
    fn macs(&self) -> Result<Ethernet2Header, String>;
}

impl<'a> GetMacAddr for LinkSlice<'a> {
    /// Simpler way to get mac addr
    fn macs(&self) -> Result<Ethernet2Header, String> {
        if let Some(header) = self.to_header() {
            if let Some(link_header) = header.ethernet2() {
                return Ok(link_header);
            }
        }
        return Err("Not found Mac".into());
    }
}

/// This impl try to fix too much 'Some' from struct
/// When you get value from struct you shall get 'Some(struct)'
/// and when you unwarp it and get value from struct you get the same 'Some(struct)'
/// It's too much unwarp needed.
impl<'a> GetMacAddr for Option<LinkSlice<'a>> {
    fn macs(&self) -> Result<Ethernet2Header, String> {
        match self {
            Some(mac) => mac.macs(),
            None => Err(String::from("Mac not found")),
        }
    }
}
