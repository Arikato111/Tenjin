//! OpenFlow v1.3 Packet-In Message Implementation
//!
//! This module implements the Packet-In message type used in OpenFlow v1.3 protocol.
//! Packet-In messages are sent by the switch to the controller when a packet
//! matches a table-miss flow entry or when explicitly instructed to do so.

use etherparse::err::packet::SliceError;

use super::{MatchFields, Payload};
use byteorder::{BigEndian, ReadBytesExt};
use etherparse::SlicedPacket;
use std::io::{BufRead, Cursor, Error};

/// Represents the reason why a packet was sent to the controller
#[repr(u8)]
#[derive(Debug)]
pub enum PacketInReason {
    /// No matching flow entry found
    NoMatch,
    /// Action explicitly requested sending to controller
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
    /// * `code` - The numeric reason code
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

/// Represents an OpenFlow v1.3 Packet-In message
///
/// Contains information about a packet that was sent to the controller,
/// including buffer ID, packet length, reason, table ID, cookie, match fields,
/// and the actual packet payload.
pub struct PacketInEvent {
    /// Optional buffer ID assigned by the switch
    pub buf_id: Option<u32>,
    /// Total length of the packet
    pub total_len: u16,
    /// Reason why the packet was sent to the controller
    pub reason: PacketInReason,
    /// ID of the table where the packet was processed
    pub table_id: u8,
    /// Cookie value from the matching flow entry
    pub cookie: u64,
    /// Match fields that triggered this packet-in
    pub matchs: MatchFields,
    /// The actual packet payload
    pub payload: Payload,
}

impl PacketInEvent {
    /// Parses the packet payload as an Ethernet frame
    ///
    /// # Returns
    /// * `Result<SlicedPacket<'_>, SliceError>` - The parsed Ethernet packet or an error
    pub fn ether_parse(&self) -> Result<SlicedPacket<'_>, SliceError> {
        match &self.payload {
            Payload::Buffered(_, p) | Payload::NoBuffered(p) => SlicedPacket::from_ethernet(&p),
        }
    }

    /// Parses a Packet-In message from a byte vector
    ///
    /// # Arguments
    /// * `payload` - The byte vector containing the message data
    ///
    /// # Returns
    /// * `Result<PacketInEvent, Error>` - The parsed PacketInEvent or an error
    pub fn parse(payload: &Vec<u8>) -> Result<PacketInEvent, Error> {
        let mut bytes = Cursor::new(payload.to_vec());
        let buf_id = match bytes.read_i32::<BigEndian>()? {
            -1 => None,
            n => Some(n as u32),
        };
        let total_len = bytes.read_u16::<BigEndian>()?;
        let reason = PacketInReason::new(bytes.read_u8()?);
        let table_id = bytes.read_u8()?;
        let cookie = bytes.read_u64::<BigEndian>()?;
        let matchs = MatchFields::parse(&mut bytes)?;
        // padding
        bytes.consume(2);
        let packet = bytes.fill_buf()?.to_vec();
        let payload = match buf_id {
            Some(n) => Payload::Buffered(n as u32, packet),
            None => Payload::NoBuffered(packet),
        };
        Ok(PacketInEvent {
            buf_id,
            total_len,
            reason,
            table_id,
            cookie,
            matchs,
            payload,
        })
    }
}
