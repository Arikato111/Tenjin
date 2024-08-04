use crate::openflow::ofp13::PseudoPort;
use crate::openflow::ofp13::{ofp_port::OfpPort, MessageMarshal, Msg};
use byteorder::{BigEndian, WriteBytesExt};

use super::{Action, Payload};

pub struct PacketOutEvent {
    // buffer_id is in Payload
    pub in_port: Option<u32>,
    pub actions: Vec<Action>,
    pub payload: Payload,
}

impl MessageMarshal for PacketOutEvent {
    fn marshal(&self, bytes: &mut Vec<u8>) {
        // buffer id
        let _ = bytes.write_i32::<BigEndian>(match self.payload {
            Payload::Buffered(n, _) => n as i32,
            Payload::NoBuffered(_) => -1,
        });
        // in_port
        match self.in_port {
            Some(id) => {
                PseudoPort::PhysicalPort(id).marshal(bytes);
            }
            None => {
                let _ = bytes.write_u32::<BigEndian>(OfpPort::Any as u32);
            }
        }
        let mut action_byte: Vec<u8> = Vec::new();
        for act in self.actions.iter() {
            let _ = act.marshal(&mut action_byte);
        }
        let _ = bytes.write_u16::<BigEndian>(action_byte.len() as u16);
        // padding 48 bit
        let _ = bytes.write_u32::<BigEndian>(0);
        let _ = bytes.write_u16::<BigEndian>(0);

        bytes.append(&mut action_byte);
        self.payload.marshal(bytes);
    }

    fn msg_code(&self) -> Msg {
        Msg::PacketOut
    }

    fn msg_usize(&self) -> usize {
        Msg::PacketOut as usize
    }

    fn size_of(&self) -> usize {
        24
    }
}

impl PacketOutEvent {
    pub fn new(in_port: Option<u32>, payload: Payload, actions: Vec<Action>) -> Self {
        Self {
            in_port,
            payload,
            actions,
        }
    }
    /* TODO
    pub fn parse(buf: &Vec<u8>) -> Result<Self, Error> {
        let mut bytes = Cursor::new(buf);
        let buf_id = match bytes.read_i32::<BigEndian>()? {
            -1 => None,
            n => Some(n),
        };
        let in_port = bytes.read_u32::<BigEndian>()?;
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
    } */
}
