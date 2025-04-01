//! This module is related to Log.
use std::net::{Ipv4Addr, Ipv6Addr};

use etherparse::NetSlice;

/// This trait provides a method to convert network packet slices into a string format suitable for logging.
/// It is implemented for different types of network packets (IPv4, IPv6, ARP) to provide consistent logging output.
pub trait Log {
    /// Converts the network packet slice into a string representation for logging purposes.
    /// The output format varies based on the packet type but generally includes source and destination information.
    fn to_log(&self) -> String;
}

/// Implementation of the Log trait for NetSlice, which can represent different types of network packets.
/// This implementation handles IPv4, IPv6, and ARP packets, formatting each appropriately for logging.
impl<'a> Log for NetSlice<'a> {
    fn to_log(&self) -> String {
        let output = match self {
            // Handle IPv4 packets by extracting source and destination IP addresses
            NetSlice::Ipv4(ipv4_slice) => {
                let header = ipv4_slice.header();

                format!(
                    "ipv4 {}:{}",
                    Ipv4Addr::from(header.source()),
                    Ipv4Addr::from(header.destination())
                )
            }

            // Handle IPv6 packets by extracting source and destination IP addresses
            NetSlice::Ipv6(ipv6_slice) => {
                let header = ipv6_slice.header();
                format!(
                    "ipv6 {} {}",
                    Ipv6Addr::from(header.source()),
                    Ipv6Addr::from(header.destination()),
                )
            }
            // Handle ARP packets by formatting the sender and target hardware addresses
            NetSlice::Arp(arp_packet_slice) => {
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

