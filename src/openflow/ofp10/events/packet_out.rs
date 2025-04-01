//! OpenFlow 1.0 Packet-Out Event
//! 
//! This module implements the packet-out event handling for OpenFlow 1.0.
//! Packet-out events are used by the controller to instruct the switch to
//! forward packets through specific ports with optional actions.
//! 
//! The module provides:
//! - Packet-out event structure
//! - Message marshaling implementation
//! - Action sequence handling
//! - Packet payload handling

use std::{
    io::{BufRead, Cursor, Error, Read},
    mem::size_of,
};

use crate::openflow::ofp10::PseudoPort;
use crate::openflow::ofp10::{ofp_port::OfpPort, MessageMarshal, Msg};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use super::{actions::SizeCheck, Action, Payload};

/// Represents a packet-out event from the controller to the switch
/// 
/// Packet-out events are used to instruct the switch to forward packets
/// through specific ports with optional actions. The packet can be either
/// buffered on the switch or sent directly from the controller.
#[derive(Debug)]
pub struct PacketOutEvent {
    /// The packet payload to be forwarded
    pub payload: Payload,
    /// The input port the packet was received on (if applicable)
    pub in_port: Option<u16>,
    /// The sequence of actions to apply to the packet
    pub actions: Vec<Action>,
}

impl MessageMarshal for PacketOutEvent {
    /// Serializes the packet-out message into a byte buffer
    /// 
    /// # Arguments
    /// * `bytes` - Mutable reference to the byte buffer to write to
    fn marshal(&self, bytes: &mut Vec<u8>) {
        let _ = bytes.write_i32::<BigEndian>(match self.payload {
            Payload::Buffered(n, _) => n as i32,
            Payload::NoBuffered(_) => -1,
        });
        match self.in_port {
            Some(id) => {
                PseudoPort::PhysicalPort(id).marshal(bytes);
            }
            None => {
                let _ = bytes.write_u16::<BigEndian>(OfpPort::None as u16);
            }
        }
        let _ = bytes.write_u16::<BigEndian>(self.actions.size_of_sequence() as u16);
        for act in self.actions.move_controller_last() {
            act.marshal(bytes);
        }
        self.payload.marshal(bytes);
    }

    /// Returns the message type code for packet-out
    /// 
    /// # Returns
    /// The Msg::PacketOut variant
    fn msg_code(&self) -> Msg {
        Msg::PacketOut
    }

    /// Returns the message type code as a usize
    /// 
    /// # Returns
    /// The numeric value of the packet-out message type
    fn msg_usize(&self) -> usize {
        Msg::PacketOut as usize
    }

    /// Returns the size of the message payload
    /// 
    /// # Returns
    /// The total size including actions and packet data
    fn size_of(&self) -> usize {
        size_of::<(u32, u16, u16)>() + self.actions.size_of_sequence() + self.payload.length()
    }
}

impl PacketOutEvent {
    /// Creates a new packet-out event
    /// 
    /// # Arguments
    /// * `in_port` - Optional input port the packet was received on
    /// * `payload` - The packet payload to be forwarded
    /// * `actions` - Sequence of actions to apply to the packet
    /// 
    /// # Returns
    /// A new PacketOutEvent instance
    pub fn new(in_port: Option<u16>, payload: Payload, actions: Vec<Action>) -> Self {
        Self {
            in_port,
            payload,
            actions,
        }
    }

    /// Parses a packet-out event from a byte buffer
    /// 
    /// # Arguments
    /// * `buf` - The byte buffer containing the packet-out event data
    /// 
    /// # Returns
    /// Result containing either the parsed PacketOutEvent or an error
    pub fn parse(buf: &Vec<u8>) -> Result<Self, Error> {
        let mut bytes = Cursor::new(buf);
        let buf_id = match bytes
            .read_i32::<BigEndian>()
            .expect("cannot parse buf id in packetout")
        {
            -1 => None,
            n => Some(n),
        };
        let in_port = bytes.read_u16::<BigEndian>()?;
        let action_len = bytes.read_u16::<BigEndian>()?;
        let mut actions_buf = vec![0; action_len as usize];
        let _ = bytes.read_exact(&mut actions_buf);
        let mut action_bytes = Cursor::new(actions_buf);
        let actions = Action::parse_sequence(&mut action_bytes);
        Ok(Self {
            payload: match buf_id {
                None => Payload::NoBuffered(bytes.fill_buf()?.to_vec()),
                Some(n) => Payload::Buffered(n as u32, bytes.fill_buf()?.to_ascii_lowercase()),
            },
            in_port: {
                if in_port == OfpPort::None as u16 {
                    None
                } else {
                    Some(in_port)
                }
            },
            actions,
        })
    }
}
