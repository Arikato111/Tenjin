//! OpenFlow 1.0 Packet-In Event
//!
//! This module implements the packet-in event handling for OpenFlow 1.0.
//! Packet-in events are sent by the switch to the controller when a packet
//! needs to be processed according to the flow table rules.
//!
//! The module provides:
//! - Packet-in reason enumeration
//! - Packet-in event structure
//! - Packet parsing and handling
//! - Ethernet packet parsing

use super::Payload;
use byteorder::{BigEndian, ReadBytesExt};
use etherparse::err::packet::SliceError;
use etherparse::SlicedPacket;
use std::io::{BufRead, Cursor, Error};

/// Represents the reason why a packet was sent to the controller
///
/// Each variant corresponds to a specific reason defined in the OpenFlow 1.0
/// specification for why a packet needs controller processing.
#[derive(Debug)]
pub enum PacketInReason {
    /// No matching flow entry found
    NoMatch,
    /// Action explicitly requested controller processing
    Action,
    /// Invalid TTL value
    InvalidTTL,
    /// Unknown reason code
    Unknown(u8),
}

impl PacketInReason {
    /// Creates a new PacketInReason from a reason code
    ///
    /// # Arguments
    /// * `code` - The reason code from the packet-in message
    ///
    /// # Returns
    /// The corresponding PacketInReason variant
    fn new(code: u8) -> Self {
        match code {
            0 => PacketInReason::NoMatch,
            1 => PacketInReason::Action,
            2 => PacketInReason::InvalidTTL,
            t => PacketInReason::Unknown(t),
        }
    }
}

/// Represents a packet-in event from the switch
///
/// Contains all the information about a packet that needs controller processing,
/// including metadata about why it was sent and the packet data itself.
#[derive(Debug)]
pub struct PacketInEvent {
    /// Buffer ID if the packet is buffered on the switch
    pub buf_id: Option<u32>,
    /// Total length of the packet
    pub total_len: u16,
    /// Port the packet was received on
    pub in_port: u16,
    /// Reason why the packet was sent to controller
    pub reason: PacketInReason,
    /// ID of the table that processed the packet
    pub table_id: u8,
    /// The packet payload
    pub payload: Payload,
}

impl PacketInEvent {
    /// Parses the packet payload as an Ethernet packet
    ///
    /// # Returns
    /// Result containing either the parsed Ethernet packet or a parsing error
    pub fn ether_parse(&self) -> Result<SlicedPacket<'_>, SliceError> {
        match &self.payload {
            Payload::Buffered(_, p) | Payload::NoBuffered(p) => SlicedPacket::from_ethernet(&p),
        }
    }

    /// Parses a packet-in event from a byte buffer
    ///
    /// # Arguments
    /// * `payload` - The byte buffer containing the packet-in event data
    ///
    /// # Returns
    /// Result containing either the parsed PacketInEvent or an error
    pub fn parse(payload: &Vec<u8>) -> Result<PacketInEvent, Error> {
        let mut bytes = Cursor::new(payload.to_vec());
        let buf_id = match bytes.read_i32::<BigEndian>()? {
            -1 => None,
            n => Some(n as u32),
        };
        let total_len = bytes.read_u16::<BigEndian>()?;
        let in_port = bytes.read_u16::<BigEndian>()?;
        let reason = PacketInReason::new(bytes.read_u8()?);
        let table_id = bytes.read_u8()?;

        let packet = bytes.fill_buf()?.to_vec();
        let payload = match buf_id {
            Some(n) => Payload::Buffered(n as u32, packet),
            None => Payload::NoBuffered(packet),
        };
        Ok(PacketInEvent {
            buf_id,
            total_len,
            in_port,
            reason,
            table_id,
            payload,
        })
    }
}
