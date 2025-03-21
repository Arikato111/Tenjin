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
}
