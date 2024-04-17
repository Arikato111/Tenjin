use std::io::Cursor;

use byteorder::{BigEndian, ReadBytesExt};

pub enum ArpOperation {
    Query = 0x0001,
    Reply = 0x0002,
    Unparse,
}

pub struct ARP {
    pub hardware_type: u16,
    pub protocol_type: u16,
    pub hardware_length: u8,
    pub protocol_length: u8,
    pub operation: ArpOperation,
    pub sender_mac: [u8; 6],
    pub sender_address: u32,
    pub target_mac: [u8; 6],
    pub target_address: u32,
}

impl ARP {
    pub fn size_of() -> usize {
        28
    }
    pub fn parse(bytes: &mut Cursor<Vec<u8>>) -> Option<ARP> {
        if bytes.get_ref().len() < ARP::size_of() {
            return None;
        }

        let hardware_type = bytes.read_u16::<BigEndian>().unwrap();
        let protocol_type = bytes.read_u16::<BigEndian>().unwrap();
        let hardware_length = bytes.read_u8().unwrap();
        let protocol_length = bytes.read_u8().unwrap();
        let operation = match bytes.read_u16::<BigEndian>().unwrap() {
            0x0001 => ArpOperation::Query,
            0x0002 => ArpOperation::Reply,
            _ => ArpOperation::Unparse,
        };
        if let ArpOperation::Unparse = operation {
            return None;
        }

        let mut sender_mac = [0u8; 6];
        for i in 0..6 {
            sender_mac[i] = bytes.read_u8().unwrap();
        }
        let sender_address = bytes.read_u32::<BigEndian>().unwrap();
        let mut target_mac = [0u8; 6];
        for i in 0..6 {
            target_mac[i] = bytes.read_u8().unwrap();
        }
        let target_address = bytes.read_u32::<BigEndian>().unwrap();
        Some(ARP {
            hardware_type,
            protocol_type,
            hardware_length,
            protocol_length,
            operation,
            sender_mac,
            sender_address,
            target_mac,
            target_address,
        })
    }
}
