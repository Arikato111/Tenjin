use etherparse::err::packet::SliceError;

use super::{MatchFields, Payload};
use byteorder::{BigEndian, ReadBytesExt};
use etherparse::SlicedPacket;
use std::io::{BufRead, Cursor, Error};

#[repr(u8)]
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
    pub reason: PacketInReason,
    pub table_id: u8,
    pub cookie: u64,
    pub matchs: MatchFields,
    pub payload: Payload,
}

impl PacketInEvent {
    pub fn ether_parse(&self) -> Result<SlicedPacket<'_>, SliceError> {
        match &self.payload {
            Payload::Buffered(_, p) | Payload::NoBuffered(p) => SlicedPacket::from_ethernet(&p),
        }
    }
    pub fn parse(payload: &Vec<u8>) -> Result<PacketInEvent, Error> {
        let mut bytes = Cursor::new(payload.to_vec());
        let buf_id = match bytes.read_i32::<BigEndian>()? {
            -1 => None,
            n => Some(n as u32),
        };
        let total_len = bytes.read_u16::<BigEndian>()?;
        let reason = PacketInReason::new(bytes.read_u8()?);
        let table_id = bytes.read_u8()?;
        let cookie = bytes.read_u64::<BigEndian>()?;
        let matchs = MatchFields::parse(&mut bytes)?;
        // padding
        bytes.consume(2);
        let packet = bytes.fill_buf()?.to_vec();
        let payload = match buf_id {
            Some(n) => Payload::Buffered(n as u32, packet),
            None => Payload::NoBuffered(packet),
        };
        Ok(PacketInEvent {
            buf_id,
            total_len,
            reason,
            table_id,
            cookie,
            matchs,
            payload,
        })
    }
}
