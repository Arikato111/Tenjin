//! OpenFlow v1.3 Packet-Out Message Implementation
//!
//! This module implements the Packet-Out message type used in OpenFlow v1.3 protocol.
//! Packet-Out messages are sent by the controller to instruct the switch to process
//! and forward a packet according to specified actions.

use crate::openflow::ofp13::PseudoPort;
use crate::openflow::ofp13::{ofp_port::OfpPort, MessageMarshal, Msg};
use byteorder::{BigEndian, WriteBytesExt};

use super::{Action, Payload};

/// Represents an OpenFlow v1.3 Packet-Out message
///
/// Contains information about a packet that should be processed by the switch,
/// including the input port, actions to apply, and the packet payload.
pub struct PacketOutEvent {
    /// Optional input port number (None means ANY port)
    pub in_port: Option<u32>,
    /// List of actions to apply to the packet
    pub actions: Vec<Action>,
    /// The packet payload (can be buffered or non-buffered)
    pub payload: Payload,
}

/// Implements message marshaling for PacketOutEvent
impl MessageMarshal for PacketOutEvent {
    /// Marshals the Packet-Out message into a byte vector
    ///
    /// # Arguments
    /// * `bytes` - The target byte vector to write the message data to
    fn marshal(&self, bytes: &mut Vec<u8>) {
        // Write buffer ID from payload
        let _ = bytes.write_i32::<BigEndian>(match self.payload {
            Payload::Buffered(n, _) => n as i32,
            Payload::NoBuffered(_) => -1,
        });

        // Write input port
        match self.in_port {
            Some(id) => {
                PseudoPort::PhysicalPort(id).marshal(bytes);
            }
            None => {
                let _ = bytes.write_u32::<BigEndian>(OfpPort::Any as u32);
            }
        }

        // Marshal actions
        let mut action_byte: Vec<u8> = Vec::new();
        for act in self.actions.iter() {
            let _ = act.marshal(&mut action_byte);
        }
        let _ = bytes.write_u16::<BigEndian>(action_byte.len() as u16);

        // Write padding (48 bits)
        let _ = bytes.write_u32::<BigEndian>(0);
        let _ = bytes.write_u16::<BigEndian>(0);

        // Append actions and payload
        bytes.append(&mut action_byte);
        self.payload.marshal(bytes);
    }

    /// Returns the OpenFlow message code for Packet-Out
    ///
    /// # Returns
    /// The Msg::PacketOut enum variant
    fn msg_code(&self) -> Msg {
        Msg::PacketOut
    }

    /// Returns the message code as a usize
    ///
    /// # Returns
    /// The numeric value of the Packet-Out message code
    fn msg_usize(&self) -> usize {
        Msg::PacketOut as usize
    }

    /// Returns the size of the Packet-Out message header
    ///
    /// # Returns
    /// The fixed size of the message header (24 bytes)
    fn size_of(&self) -> usize {
        24
    }
}

impl PacketOutEvent {
    /// Creates a new Packet-Out message
    ///
    /// # Arguments
    /// * `in_port` - Optional input port number
    /// * `payload` - The packet payload
    /// * `actions` - List of actions to apply
    ///
    /// # Returns
    /// A new PacketOutEvent instance
    pub fn new(in_port: Option<u32>, payload: Payload, actions: Vec<Action>) -> Self {
        Self {
            in_port,
            payload,
            actions,
        }
    }
}
