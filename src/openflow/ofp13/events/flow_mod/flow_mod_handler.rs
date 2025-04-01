//! OpenFlow v1.3 Flow Modification Handler
//! 
//! This module implements the handler for flow modification messages in OpenFlow v1.3.
//! It provides functionality to add, modify, and delete flow entries in the switch's flow tables.

use byteorder::{BigEndian, WriteBytesExt};

use crate::openflow::ofp13::{
    events::{actions::ToInstruction, Action},
    ofp_port::OfpPort,
    MessageMarshal, Msg, PseudoPort,
};

use super::{instructions::Instrucion, FlowModCommand, FlowModFlags, MatchFields};

/// Timeout configuration for flow entries
pub enum Timeout {
    /// Flow entry never expires
    Permanent,
    /// Flow entry expires after specified seconds
    ExpireAfter(u16),
}

impl Timeout {
    /// Parses a timeout value from a u16
    /// 
    /// # Arguments
    /// * `tm` - Timeout value in seconds (0 for permanent)
    /// 
    /// # Returns
    /// * `Timeout` - The parsed timeout value
    pub fn parse(tm: u16) -> Self {
        match tm {
            0 => Self::Permanent,
            d => Timeout::ExpireAfter(d),
        }
    }

    /// Converts the timeout to a u16 value
    /// 
    /// # Returns
    /// * `u16` - The timeout value in seconds (0 for permanent)
    pub fn to_int(&self) -> u16 {
        match self {
            Timeout::Permanent => 0,
            Timeout::ExpireAfter(d) => *d,
        }
    }
}

/// Flow modification event structure
pub struct FlowModEvent {
    /// Cookie value for the flow entry
    cookie: u64,
    /// Cookie mask for the flow entry
    cookie_mask: u64,
    /// ID of the table to modify
    table_id: u8,
    /// Command to apply (add, modify, delete)
    command: FlowModCommand,
    /// Timeout for idle flows
    idle_timeout: Timeout,
    /// Timeout for all flows
    hard_timeout: Timeout,
    /// Priority of the flow entry
    priority: u16,
    /// Optional buffer ID for buffered packets
    buffer_id: Option<u32>,
    /// Optional output port
    out_port: Option<PseudoPort>,
    /// Optional output group
    out_group: Option<PseudoPort>,
    /// Flow modification flags
    flags: FlowModFlags,
    /// Match fields for the flow entry
    match_fields: MatchFields,
    /// Instructions to apply to matching packets
    instruction: Instrucion,
}

impl FlowModEvent {
    /// Creates a new flow modification event for adding a flow
    /// 
    /// # Arguments
    /// * `priority` - Priority of the flow entry
    /// * `match_fileds` - Match fields for the flow entry
    /// * `actions` - Actions to apply to matching packets
    /// * `table_id` - ID of the table to add the flow to
    /// * `buffer_id` - Optional buffer ID for buffered packets
    /// 
    /// # Returns
    /// * `FlowModEvent` - The new flow modification event
    pub fn add_flow(
        priority: u16,
        match_fileds: MatchFields,
        actions: Vec<Action>,
        table_id: u8,
        buffer_id: Option<u32>,
    ) -> Self {
        Self {
            cookie: 0,
            cookie_mask: 0,
            table_id,
            command: FlowModCommand::Add,
            idle_timeout: Timeout::Permanent,
            hard_timeout: Timeout::Permanent,
            priority,
            buffer_id,
            out_port: None,
            out_group: None,
            flags: FlowModFlags::all_false(),
            match_fields: match_fileds,
            instruction: Instrucion::InstructActions(actions.to_instruct()),
        }
    }
}

impl MessageMarshal for FlowModEvent {
    /// Returns the message type as a usize
    fn msg_usize(&self) -> usize {
        Msg::FlowMod as usize
    }

    /// Returns the size of the message in bytes
    fn size_of(&self) -> usize {
        24
    }

    /// Returns the message type
    fn msg_code(&self) -> Msg {
        Msg::FlowMod
    }

    /// Marshals the message into a byte buffer
    /// 
    /// # Arguments
    /// * `bytes` - The buffer to write the message to
    fn marshal(&self, bytes: &mut Vec<u8>) {
        let _ = bytes.write_u64::<BigEndian>(self.cookie);
        let _ = bytes.write_u64::<BigEndian>(self.cookie_mask);
        let _ = bytes.write_u8(self.table_id);
        let _ = bytes.write_u8(self.command.to_number() as u8);
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
                let _ = bytes.write_u32::<BigEndian>(OfpPort::Any as u32);
            }
        }
        match self.out_group.as_ref() {
            Some(p) => p.marshal(bytes),
            None => {
                let _ = bytes.write_u32::<BigEndian>(OfpPort::Any as u32);
            }
        }

        self.flags.marshal(bytes);
        // padding
        let _ = bytes.write_u16::<BigEndian>(0);
        let _ = self.match_fields.marshal(bytes);
        self.instruction.marshal(bytes);
    }
}
