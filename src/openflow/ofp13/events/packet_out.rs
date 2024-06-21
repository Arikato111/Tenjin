use std::{
    io::{BufRead, Cursor, Error, Read},
    mem::size_of,
};

use crate::openflow::ofp13::PseudoPort;
use crate::openflow::ofp13::{ofp_port::OfpPort, MessageMarshal, Msg};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use super::{actions::SizeCheck, Action, Payload};

pub struct PacketOutEvent {
    pub payload: Payload,
    pub in_port: Option<u16>,
    pub actions: Vec<Action>,
}

impl MessageMarshal for PacketOutEvent {
    fn marshal(&self, bytes: &mut Vec<u8>) {
        let _ = bytes.write_i32::<BigEndian>(match self.payload {
            Payload::Buffered(n, _) => n as i32,
            Payload::NoBuffered(_) => -1,
        });
        match self.in_port {
            Some(id) => {
                PseudoPort::PhysicalPort(id).marshal(bytes);
            }
            None => {
                let _ = bytes.write_u16::<BigEndian>(OfpPort::None as u16);
            }
        }
        let _ = bytes.write_u16::<BigEndian>(self.actions.size_of_sequence() as u16);
        for act in self.actions.move_controller_last() {
            act.marshal(bytes);
        }
        self.payload.marshal(bytes);
    }

    fn msg_code(&self) -> Msg {
        Msg::PacketOut
    }

    fn msg_usize(&self) -> usize {
        Msg::PacketOut as usize
    }

    fn size_of(&self) -> usize {
        size_of::<(u32, u16, u16)>() + self.actions.size_of_sequence() + self.payload.length()
    }
}

impl PacketOutEvent {
    pub fn new(in_port: Option<u16>, payload: Payload, actions: Vec<Action>) -> Self {
        Self {
            in_port,
            payload,
            actions,
        }
    }
    pub fn parse(buf: &Vec<u8>) -> Result<Self, Error> {
        let mut bytes = Cursor::new(buf);
        let buf_id = match bytes
            .read_i32::<BigEndian>()
            .expect("cannot parse buf id in packetout")
        {
            -1 => None,
            n => Some(n),
        };
        let in_port = bytes.read_u16::<BigEndian>()?;
        let action_len = bytes.read_u16::<BigEndian>()?;
        let mut actions_buf = vec![0; action_len as usize];
        let _ = bytes.read_exact(&mut actions_buf);
        let mut action_bytes = Cursor::new(actions_buf);
        let actions = Action::parse_sequence(&mut action_bytes);
        Ok(Self {
            payload: match buf_id {
                None => Payload::NoBuffered(bytes.fill_buf()?.to_vec()),
                Some(n) => Payload::Buffered(n as u32, bytes.fill_buf()?.to_ascii_lowercase()),
            },
            in_port: {
                if in_port == OfpPort::None as u16 {
                    None
                } else {
                    Some(in_port)
                }
            },
            actions,
        })
    }
}
