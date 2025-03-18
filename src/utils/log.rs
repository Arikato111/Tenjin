//! This module is related to Log.
use std::net::{Ipv4Addr, Ipv6Addr};

use etherparse::NetSlice;

/// this trait is used to show any value as string as log format.
pub trait Log {
    fn to_log(&self) -> String;
}

impl<'a> Log for NetSlice<'a> {
    fn to_log(&self) -> String {
        let output = match self {
            NetSlice::Ipv4(ipv4_slice) => {
                let header = ipv4_slice.header();

                format!(
                    "ipv4 {}:{}",
                    Ipv4Addr::from(header.source()),
                    Ipv4Addr::from(header.destination())
                )
            }

            NetSlice::Ipv6(ipv6_slice) => {
                let header = ipv6_slice.header();
                format!(
                    "ipv6 {} {}",
                    // header.source(),
                    Ipv6Addr::from(header.source()),
                    Ipv6Addr::from(header.destination()),
                )
            }
            NetSlice::Arp(arp_packet_slice) => {
                // let header = arp_packet_slice.
                format!(
                    "arp {:?} {:?}",
                    arp_packet_slice
                        .sender_hw_addr()
                        .iter()
                        .map(|v| v.to_string())
                        .collect::<Vec<String>>()
                        .join(":"),
                    arp_packet_slice.target_hw_addr()
                )
            }
        };
        output
    }
}

