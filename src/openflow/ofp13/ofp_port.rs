//! OpenFlow 1.3 Port Types
//! 
//! This module defines the port types used in OpenFlow 1.3 protocol, including
//! both physical ports and special ports used for packet forwarding and control.
//! OpenFlow 1.3 uses 32-bit port numbers instead of 16-bit as in version 1.0.

use byteorder::{BigEndian, WriteBytesExt};

/// Represents the standard OpenFlow 1.3 port numbers
/// 
/// These values are defined in the OpenFlow 1.3 specification and include
/// both physical port numbers and special port numbers used for packet forwarding.
/// All port numbers are 32-bit values in OpenFlow 1.3.
#[repr(u32)]
#[derive(Debug)]
pub enum OfpPort {
    /// Maximum physical port number (0xffffff00)
    Max = 0xffffff00,
    /// Port that packet was received on
    InPort = 0xfffffff8,
    /// Forward to flow table
    Table = 0xfffffff9,
    /// Forward using normal L2/L3 processing
    Normal = 0xfffffffa,
    /// Forward to all physical ports except input port
    Flood = 0xfffffffb,
    /// Forward to all physical ports
    All = 0xfffffffc,
    /// Forward to controller
    Controller = 0xfffffffd,
    /// Forward to local port
    Local = 0xfffffffe,
    /// Wildcard port (any port)
    Any = 0xffffffff,
}

/// Represents a port in the OpenFlow 1.3 protocol
/// 
/// This enum provides a more ergonomic way to work with ports, handling both
/// physical ports and special ports with their associated data. All port numbers
/// are 32-bit values in OpenFlow 1.3.
#[derive(Clone, Debug)]
pub enum PseudoPort {
    /// Physical port with port number
    PhysicalPort(u32),
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
    /// Option containing the parsed PseudoPort
    pub fn parse(byte: u32) -> Option<PseudoPort> {
        Some(PseudoPort::new(byte, Some(0)))
    }

    /// Creates a new PseudoPort from a port number and optional length
    /// 
    /// # Arguments
    /// * `port` - The port number to create
    /// * `len` - Optional queue length for controller ports
    /// 
    /// # Returns
    /// A new PseudoPort instance
    pub fn new(port: u32, len: Option<u64>) -> PseudoPort {
        match port {
            p if p == (OfpPort::InPort as u32) => PseudoPort::InPort,
            p if p == (OfpPort::Table as u32) => PseudoPort::Table,
            p if p == (OfpPort::Normal as u32) => PseudoPort::Normal,
            p if p == (OfpPort::Flood as u32) => PseudoPort::Flood,
            p if p == (OfpPort::All as u32) => PseudoPort::AllPorts,
            p if p == (OfpPort::Controller as u32) => match len {
                Some(len) => PseudoPort::Controller(len),
                None => PseudoPort::Unsupport,
            },
            p if p == (OfpPort::Local as u32) => PseudoPort::InPort,
            _ => {
                if port <= (OfpPort::Max as u32) {
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
            PseudoPort::InPort => OfpPort::InPort as u32,
            PseudoPort::Table => OfpPort::Table as u32,
            PseudoPort::Normal => OfpPort::Normal as u32,
            PseudoPort::Flood => OfpPort::Flood as u32,
            PseudoPort::AllPorts => OfpPort::All as u32,
            PseudoPort::Controller(_) => OfpPort::Controller as u32,
            PseudoPort::Local => OfpPort::Local as u32,
            // not sure how to handle unsupport
            PseudoPort::Unsupport => OfpPort::Flood as u32,
        };
        let _ = bytes.write_u32::<BigEndian>(port);
    }
}
