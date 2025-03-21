use byteorder::{BigEndian, WriteBytesExt};

use crate::openflow::ofp13::{
    events::{actions::ToInstruction, Action},
    ofp_port::OfpPort,
    MessageMarshal, Msg, PseudoPort,
};

use super::{instructions::Instrucion, FlowModCommand, FlowModFlags, MatchFields};

pub enum Timeout {
    Permanent,
    ExpireAfter(u16),
}
impl Timeout {
    pub fn parse(tm: u16) -> Self {
        match tm {
            0 => Self::Permanent,
            d => Timeout::ExpireAfter(d),
        }
    }
    pub fn to_int(&self) -> u16 {
        match self {
            Timeout::Permanent => 0,
            Timeout::ExpireAfter(d) => *d,
        }
    }
}

pub struct FlowModEvent {
    cookie: u64,
    cookie_mask: u64,

    table_id: u8,
    command: FlowModCommand,
    idle_timeout: Timeout,
    hard_timeout: Timeout,
    priority: u16,
    buffer_id: Option<u32>,
    out_port: Option<PseudoPort>,
    out_group: Option<PseudoPort>,
    flags: FlowModFlags,
    // pad: [u8; 2],
    // ofp_match
    match_fields: MatchFields,
    instruction: Instrucion,
}

impl FlowModEvent {
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
    fn msg_usize(&self) -> usize {
        Msg::FlowMod as usize
    }
    fn size_of(&self) -> usize {
        24
    }
    fn msg_code(&self) -> Msg {
        Msg::FlowMod
    }
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
