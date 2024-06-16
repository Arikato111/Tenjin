use crate::etherparser::ethernet::EthernetFrame;

use super::Payload;
use byteorder::{BigEndian, ReadBytesExt};
use std::io::{BufRead, Cursor, Error};

#[derive(Debug)]
pub enum PacketInReason {
    NoMatch,
    Action,
    InvalidTTL,
    Unknown(u8),
}

impl PacketInReason {
    fn new(code: u8) -> Self {
        match code {
            0 => PacketInReason::NoMatch,
            1 => PacketInReason::Action,
            2 => PacketInReason::InvalidTTL,
            t => PacketInReason::Unknown(t),
        }
    }
}

pub struct PacketInEvent {
    pub buf_id: Option<u32>,
    pub total_len: u16,
    pub in_port: u16,
    pub reason: PacketInReason,
    pub table_id: u8,
    pub payload: Payload,
}

impl PacketInEvent {
    pub fn ether_parse(&self) -> Result<EthernetFrame, Error> {
        match &self.payload {
            Payload::Buffered(_, p) | Payload::NoBuffered(p) => EthernetFrame::parse(&p),
        }
    }
    pub fn parse(payload: &Vec<u8>) -> Result<PacketInEvent, Error> {
        let mut bytes = Cursor::new(payload.to_vec());
        let buf_id = match bytes.read_i32::<BigEndian>()? {
            -1 => None,
            n => Some(n as u32),
        };
        let total_len = bytes.read_u16::<BigEndian>()?;
        let in_port = bytes.read_u16::<BigEndian>()?;
        let reason = PacketInReason::new(bytes.read_u8()?);
        let table_id = bytes.read_u8()?;
        let packet = bytes.fill_buf()?.to_vec();
        let payload = match buf_id {
            Some(n) => Payload::Buffered(n as u32, packet),
            None => Payload::NoBuffered(packet),
        };
        Ok(PacketInEvent {
            buf_id,
            total_len,
            in_port,
            reason,
            table_id,
            payload,
        })
    }
}
