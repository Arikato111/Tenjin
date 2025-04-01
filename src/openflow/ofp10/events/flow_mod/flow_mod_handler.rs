//! OpenFlow 1.0 Flow Modification Event Handler
//!
//! This module implements the flow modification event handling for OpenFlow 1.0.
//! Flow modification events are used to add, modify, or delete flow entries
//! in the switch's flow tables.
//!
//! The module provides:
//! - Flow modification event structure
//! - Timeout handling for flow entries
//! - Flow entry creation and parsing
//! - Message marshaling implementation

use std::io::{Cursor, Error};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::openflow::ofp10::{
    events::{actions::SizeCheck, Action},
    ofp_port::OfpPort,
    MessageMarshal, Msg, PseudoPort,
};

use super::{FlowModCommand, FlowModFlags, MatchFields};

/// Represents the timeout settings for a flow entry
///
/// Flow entries can be either permanent or have a specific timeout duration.
#[derive(Debug)]
pub enum Timeout {
    /// Flow entry never expires
    Permanent,
    /// Flow entry expires after specified seconds
    ExpireAfter(u16),
}

impl Timeout {
    /// Parses a timeout value from a byte value
    ///
    /// # Arguments
    /// * `tm` - The timeout value in seconds (0 for permanent)
    ///
    /// # Returns
    /// A new Timeout instance
    pub fn parse(tm: u16) -> Self {
        match tm {
            0 => Self::Permanent,
            d => Timeout::ExpireAfter(d),
        }
    }

    /// Converts the timeout to its numeric value
    ///
    /// # Returns
    /// The timeout value in seconds (0 for permanent)
    pub fn to_int(&self) -> u16 {
        match self {
            Timeout::Permanent => 0,
            Timeout::ExpireAfter(d) => *d,
        }
    }
}

/// Represents a flow modification event
///
/// Contains all the information needed to add, modify, or delete a flow entry
/// in the switch's flow tables.
pub struct FlowModEvent {
    /// The type of flow modification command
    command: FlowModCommand,
    /// The match fields for the flow entry
    match_fields: MatchFields,
    /// Priority of the flow entry
    priority: u16,
    /// Actions to apply to matching packets
    actions: Vec<Action>,
    /// Cookie value for the flow entry
    cookie: u64,
    /// Timeout for idle flows
    idle_timeout: Timeout,
    /// Timeout for all flows
    hard_timeout: Timeout,
    /// Flow modification flags
    flags: FlowModFlags,
    /// Buffer ID if packet is buffered
    buffer_id: Option<u32>,
    /// Output port for the flow entry
    out_port: Option<PseudoPort>,
}

impl FlowModEvent {
    /// Creates a new flow modification event for adding a flow entry
    ///
    /// # Arguments
    /// * `priority` - Priority of the flow entry
    /// * `match_fileds` - Match fields for the flow entry
    /// * `actions` - Actions to apply to matching packets
    /// * `buffer_id` - Optional buffer ID if packet is buffered
    ///
    /// # Returns
    /// A new FlowModEvent instance configured for adding a flow entry
    pub fn add_flow(
        priority: u16,
        match_fileds: MatchFields,
        actions: Vec<Action>,
        buffer_id: Option<u32>,
    ) -> Self {
        Self {
            command: FlowModCommand::Add,
            match_fields: match_fileds,
            priority,
            actions,
            cookie: 0,
            idle_timeout: Timeout::Permanent,
            hard_timeout: Timeout::Permanent,
            flags: FlowModFlags::all_false(),
            buffer_id,
            out_port: None,
        }
    }

    /// Parses a flow modification event from a byte buffer
    ///
    /// # Arguments
    /// * `buf` - The byte buffer containing the flow modification event data
    ///
    /// # Returns
    /// Result containing either the parsed FlowModEvent or an error
    pub fn parse(buf: &[u8]) -> Result<FlowModEvent, Error> {
        let mut bytes = Cursor::new(buf.to_vec());
        let match_fields = MatchFields::parse(&mut bytes)?;
        let cookie = bytes.read_u64::<BigEndian>()?;
        let command = FlowModCommand::parse(bytes.read_u16::<BigEndian>()?);
        let idle_timeout = Timeout::parse(bytes.read_u16::<BigEndian>()?);
        let hard_timeout = Timeout::parse(bytes.read_u16::<BigEndian>()?);
        let priority = bytes.read_u16::<BigEndian>()?;
        let buffer_id = bytes.read_i32::<BigEndian>()?;
        let out_port = PseudoPort::parse(bytes.read_u16::<BigEndian>()?);
        let flags = bytes.read_u16::<BigEndian>()?;
        let actions = Action::parse_sequence(&mut bytes);
        Ok(FlowModEvent {
            command,
            match_fields,
            cookie,
            actions,
            priority,
            idle_timeout,
            hard_timeout,
            flags: FlowModFlags::parse(flags),
            buffer_id: {
                match buffer_id {
                    -1 => None,
                    n => Some(n as u32),
                }
            },
            out_port,
        })
    }
}

/// Implementation of MessageMarshal trait for FlowModEvent
///
/// Provides functionality for serializing and handling OpenFlow flow modification messages.
impl MessageMarshal for FlowModEvent {
    /// Returns the message type code as a usize
    ///
    /// # Returns
    /// The numeric value of the flow modification message type
    fn msg_usize(&self) -> usize {
        Msg::FlowMod as usize
    }

    /// Returns the size of the message payload
    ///
    /// # Returns
    /// The size of the flow modification message in bytes
    fn size_of(&self) -> usize {
        24
    }

    /// Returns the message type code
    ///
    /// # Returns
    /// The Msg::FlowMod variant
    fn msg_code(&self) -> Msg {
        Msg::FlowMod
    }

    /// Serializes the flow modification message into a byte buffer
    ///
    /// # Arguments
    /// * `bytes` - Mutable reference to the byte buffer to write to
    fn marshal(&self, bytes: &mut Vec<u8>) {
        self.match_fields.marshal(bytes);
        let _ = bytes.write_u64::<BigEndian>(self.cookie);
        let _ = bytes.write_u16::<BigEndian>(self.command.to_number() as u16);
        let _ = bytes.write_u16::<BigEndian>(self.idle_timeout.to_int());
        let _ = bytes.write_u16::<BigEndian>(self.hard_timeout.to_int());
        let _ = bytes.write_u16::<BigEndian>(self.priority);
        let _ = bytes.write_i32::<BigEndian>(match self.buffer_id {
            None => -1,
            Some(buf_id) => buf_id as i32,
        });
        match self.out_port.as_ref() {
            Some(p) => p.marshal(bytes),
            None => {
                let _ = bytes.write_u16::<BigEndian>(OfpPort::None as u16);
            }
        }
        self.flags.marshal(bytes);
        for act in self.actions.move_controller_last() {
            match act {
                Action::Oputput(PseudoPort::Table) => {
                    panic!("Openflow table not allowed")
                }
                _ => (),
            }
            act.marshal(bytes);
        }
    }
}
