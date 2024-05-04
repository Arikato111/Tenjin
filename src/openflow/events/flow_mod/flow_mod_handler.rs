use std::io::Cursor;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::openflow::{OfpPort, PseudoPort};

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

pub struct FlowMod {
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

impl FlowMod {
    pub fn get_controller_last(actions: Vec<FlowAction>) -> Vec<FlowAction> {
        let mut not_ctrl: Vec<FlowAction> = Vec::new();
        let mut is_ctrl: Vec<FlowAction> = Vec::new();
        for act in actions {
            match act {
                FlowAction::Oputput(PseudoPort::Controller(_)) => {
                    is_ctrl.push(act);
                }
                _ => not_ctrl.push(act),
            }
        }
        not_ctrl.append(&mut is_ctrl);
        not_ctrl
    }

    pub fn size_of() -> usize {
        24
    }
    pub fn parse(buf: &[u8]) -> FlowMod {
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
        FlowMod {
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
    pub fn marshal(flowmod: FlowMod, bytes: &mut Vec<u8>) {
        flowmod.match_fields.marshal(bytes);
        let _ = bytes.write_u64::<BigEndian>(flowmod.cookie);
        let _ = bytes.write_u16::<BigEndian>(flowmod.command as u16);
        let _ = bytes.write_u16::<BigEndian>(flowmod.idle_timeout.to_int());
        let _ = bytes.write_u16::<BigEndian>(flowmod.hard_timeout.to_int());
        let _ = bytes.write_u16::<BigEndian>(flowmod.priority);
        let _ = bytes.write_i32::<BigEndian>(match flowmod.apply_to_packet {
            None => -1,
            Some(buf_id) => buf_id as i32,
        });
        match flowmod.out_port {
            None => bytes.write_u16::<BigEndian>(OfpPort::None as u16).unwrap(),
            Some(p) => p.marshal(bytes),
        }
        let _ = bytes.write_u16::<BigEndian>(
            (if flowmod.check_overlap { 1 << 1 } else { 0 })
                | (if flowmod.notify_when_removed {
                    1 << 0
                } else {
                    0
                }),
        );
        for act in FlowMod::get_controller_last(flowmod.actions) {
            match act {
                FlowAction::Oputput(PseudoPort::Table) => {
                    panic!("Openflow table not allowed")
                }
                _ => (),
            }
        }
    }
}
