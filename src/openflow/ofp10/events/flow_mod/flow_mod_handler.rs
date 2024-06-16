use std::io::{Cursor, Error};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::openflow::ofp10::{
    events::{actions::SizeCheck, Action},
    ofp_port::OfpPort,
    MessageMarshal, Msg, PseudoPort,
};

use super::{FlowModCommand, FlowModFlags, MatchFields};

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
    command: FlowModCommand,
    match_fields: MatchFields,
    priority: u16,
    actions: Vec<Action>,
    cookie: u64,
    idle_timeout: Timeout,
    hard_timeout: Timeout,
    flags: FlowModFlags,
    buffer_id: Option<u32>,
    out_port: Option<PseudoPort>,
}

impl FlowModEvent {
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
