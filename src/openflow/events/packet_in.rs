use std::io::{BufRead, Cursor};

use byteorder::{BigEndian, ReadBytesExt};

use crate::etherparser::ethernet::EthernetFrame;

pub enum PacketInReason {
    NoMatch,
    Action,
    Unknown,
}

pub struct PacketInEvent {
    pub buf_id: Option<i32>,
    pub total_len: u16,
    pub port: u16,
    pub reason: PacketInReason,
    pub table_id: u8,
    pub payload: EthernetFrame,
}

impl PacketInEvent {
    pub fn parse(payload: &Vec<u8>) -> PacketInEvent {
        let mut bytes = Cursor::new(payload.to_vec());
        let buf_id = match bytes.read_i32::<BigEndian>().unwrap() {
            -1 => None,
            n => Some(n),
        };
        let total_len = bytes.read_u16::<BigEndian>().unwrap();
        let port = bytes.read_u16::<BigEndian>().unwrap();
        let reason = match bytes.read_u8().unwrap() {
            1 => PacketInReason::NoMatch,
            2 => PacketInReason::Action,
            _ => PacketInReason::Unknown,
        };
        let table_id = bytes.read_u8().unwrap();
        let packet = bytes.fill_buf().unwrap().to_vec();
        let payload = EthernetFrame::parse(&packet);
        PacketInEvent {
            buf_id,
            total_len,
            port,
            reason,
            table_id,
            payload,
        }
    }
}
