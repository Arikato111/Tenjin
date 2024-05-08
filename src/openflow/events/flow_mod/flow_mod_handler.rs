use std::io::Cursor;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::openflow::{
    ofp_manager::{MessageMarshal, OfpMsg, OfpMsgEvent},
    OfpPort, PseudoPort,
};

use super::{FlowAction, FlowModCommand, MatchFields};

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
    actions: Vec<FlowAction>,
    cookie: u64,
    idle_timeout: Timeout,
    hard_timeout: Timeout,
    notify_when_removed: bool,
    apply_to_packet: Option<u32>,
    out_port: Option<PseudoPort>,
    check_overlap: bool,
}

impl FlowModEvent {
    pub fn get_controller_last(&self) -> Vec<FlowAction> {
        let mut not_ctrl: Vec<FlowAction> = Vec::new();
        let mut is_ctrl: Vec<FlowAction> = Vec::new();
        for act in &self.actions {
            match act {
                FlowAction::Oputput(PseudoPort::Controller(_)) => {
                    is_ctrl.push(act.clone());
                }
                _ => not_ctrl.push(act.clone()),
            }
        }
        not_ctrl.append(&mut is_ctrl);
        not_ctrl
    }

    pub fn parse(buf: &[u8]) -> FlowModEvent {
        let mut bytes = Cursor::new(buf.to_vec());
        let match_fields = MatchFields::parse(&mut bytes);
        let cookie = bytes.read_u64::<BigEndian>().unwrap();
        let command = FlowModCommand::parse(bytes.read_u16::<BigEndian>().unwrap());
        let idle_timeout = Timeout::parse(bytes.read_u16::<BigEndian>().unwrap());
        let hard_timeout = Timeout::parse(bytes.read_u16::<BigEndian>().unwrap());
        let priority = bytes.read_u16::<BigEndian>().unwrap();
        let buffer_id = bytes.read_i32::<BigEndian>().unwrap();
        let out_port = PseudoPort::parse(bytes.read_u16::<BigEndian>().unwrap());
        let flags = bytes.read_u16::<BigEndian>().unwrap();
        let actions = FlowAction::parse_sequence(&mut bytes);
        FlowModEvent {
            command,
            match_fields,
            cookie,
            actions,
            priority,
            idle_timeout,
            hard_timeout,
            notify_when_removed: flags & 1 != 0,
            apply_to_packet: {
                match buffer_id {
                    -1 => None,
                    n => Some(n as u32),
                }
            },
            out_port,
            check_overlap: flags & 2 != 0,
        }
    }
}

impl MessageMarshal for FlowModEvent {
    fn msg_usize<OFP: OfpMsgEvent>(&self, ofp: &OFP) -> usize {
        ofp.msg_usize(OfpMsg::FlowMod)
    }
    fn size_of(&self) -> usize {
        24
    }
    fn msg_code(&self) -> OfpMsg {
        OfpMsg::FlowMod
    }
    fn marshal(&self, bytes: &mut Vec<u8>) {
        self.match_fields.marshal(bytes);
        let _ = bytes.write_u64::<BigEndian>(self.cookie);
        let _ = bytes.write_u16::<BigEndian>(self.command.to_number() as u16);
        let _ = bytes.write_u16::<BigEndian>(self.idle_timeout.to_int());
        let _ = bytes.write_u16::<BigEndian>(self.hard_timeout.to_int());
        let _ = bytes.write_u16::<BigEndian>(self.priority);
        let _ = bytes.write_i32::<BigEndian>(match self.apply_to_packet {
            None => -1,
            Some(buf_id) => buf_id as i32,
        });
        match self.out_port.as_ref() {
            None => bytes.write_u16::<BigEndian>(OfpPort::None as u16).unwrap(),
            Some(p) => p.marshal(bytes),
        }
        let _ = bytes.write_u16::<BigEndian>(
            (if self.check_overlap { 1 << 1 } else { 0 })
                | (if self.notify_when_removed { 1 << 0 } else { 0 }),
        );
        for act in self.get_controller_last() {
            match act {
                FlowAction::Oputput(PseudoPort::Table) => {
                    panic!("Openflow table not allowed")
                }
                _ => (),
            }
        }
    }
}
