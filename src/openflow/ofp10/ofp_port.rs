//! OpenFlow 1.0 Port Types
//! 
//! This module defines the port types used in OpenFlow 1.0 protocol, including
//! both physical ports and special ports used for packet forwarding and control.

use byteorder::{BigEndian, WriteBytesExt};

/// Represents the standard OpenFlow 1.0 port numbers
/// 
/// These values are defined in the OpenFlow 1.0 specification and include
/// both physical port numbers and special port numbers used for packet forwarding.
#[derive(Debug)]
pub enum OfpPort {
    /// Maximum physical port number (0xff00)
    Max = 0xff00,
    /// Port that packet was received on
    InPort = 0xfff8,
    /// Forward to flow table
    Table = 0xfff9,
    /// Forward using normal L2/L3 processing
    Normal = 0xfffa,
    /// Forward to all physical ports except input port
    Flood = 0xfffb,
    /// Forward to all physical ports
    All = 0xfffc,
    /// Forward to controller
    Controller = 0xfffd,
    /// Forward to local port
    Local = 0xfffe,
    /// Drop packet
    None = 0xffff,
}

/// Represents a port in the OpenFlow 1.0 protocol
/// 
/// This enum provides a more ergonomic way to work with ports, handling both
/// physical ports and special ports with their associated data.
#[derive(Clone, Debug)]
pub enum PseudoPort {
    /// Physical port with port number
    PhysicalPort(u16),
    /// Port that packet was received on
    InPort,
    /// Forward to flow table
    Table,
    /// Forward using normal L2/L3 processing
    Normal,
    /// Forward to all physical ports except input port
    Flood,
    /// Forward to all physical ports
    AllPorts,
    /// Forward to controller with queue length
    Controller(u64),
    /// Forward to local port
    Local,
    /// Unsupported port type
    Unsupport,
}

impl PseudoPort {
    /// Parses a port number into a PseudoPort
    /// 
    /// # Arguments
    /// * `byte` - The port number to parse
    /// 
    /// # Returns
    /// Option containing the parsed PseudoPort, or None if port is OFPP_NONE
    pub fn parse(byte: u16) -> Option<PseudoPort> {
        if (OfpPort::None as u16) == byte {
            None
        } else {
            Some(PseudoPort::new(byte, Some(0)))
        }
    }

    /// Creates a new PseudoPort from a port number and optional length
    /// 
    /// # Arguments
    /// * `port` - The port number to create
    /// * `len` - Optional queue length for controller ports
    /// 
    /// # Returns
    /// A new PseudoPort instance
    pub fn new(port: u16, len: Option<u64>) -> PseudoPort {
        match port {
            p if p == (OfpPort::InPort as u16) => PseudoPort::InPort,
            p if p == (OfpPort::Table as u16) => PseudoPort::Table,
            p if p == (OfpPort::Normal as u16) => PseudoPort::Normal,
            p if p == (OfpPort::Flood as u16) => PseudoPort::Flood,
            p if p == (OfpPort::All as u16) => PseudoPort::AllPorts,
            p if p == (OfpPort::Controller as u16) => match len {
                Some(len) => PseudoPort::Controller(len),
                None => PseudoPort::Unsupport,
            },
            p if p == (OfpPort::Local as u16) => PseudoPort::InPort,
            _ => {
                if port <= (OfpPort::Max as u16) {
                    PseudoPort::PhysicalPort(port)
                } else {
                    PseudoPort::Unsupport
                }
            }
        }
    }

    /// Serializes the port into a byte buffer
    /// 
    /// # Arguments
    /// * `bytes` - Mutable reference to the byte buffer to write to
    pub fn marshal(&self, bytes: &mut Vec<u8>) {
        let port = match *self {
            PseudoPort::PhysicalPort(p) => p,
            PseudoPort::InPort => OfpPort::InPort as u16,
            PseudoPort::Table => OfpPort::Table as u16,
            PseudoPort::Normal => OfpPort::Normal as u16,
            PseudoPort::Flood => OfpPort::Flood as u16,
            PseudoPort::AllPorts => OfpPort::All as u16,
            PseudoPort::Controller(_) => OfpPort::Controller as u16,
            PseudoPort::Local => OfpPort::Local as u16,
            // not sure how to handle unsupport
            PseudoPort::Unsupport => OfpPort::Flood as u16,
        };
        let _ = bytes.write_u16::<BigEndian>(port);
    }
}
