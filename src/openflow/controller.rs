use std::{collections::HashMap, io::Write, mem::size_of, net::TcpStream};

use byteorder::{BigEndian, WriteBytesExt};

use crate::etherparser::ethernet::EthernetFrame;

use super::OfpHeader;

pub struct Controller {
    version: u8,
    mac_to_port: HashMap<u64, u16>,
}

impl Controller {
    pub const OFP_1_0: u8 = 1;
    pub fn new(version: u8) -> Self {
        Self {
            version,
            mac_to_port: HashMap::new(),
        }
    }
    pub fn hello(&self, stream: &mut TcpStream) {
        let header = OfpHeader::new(self.version, 0, size_of::<OfpHeader>() as u16, 0);
        let mut bytes: Vec<u8> = Vec::new();
        header.marshal(&mut bytes);
        stream.write_all(&bytes).unwrap();
    }

    pub fn feture_req(&self, xid: u32, stream: &mut TcpStream) {
        let header = OfpHeader::new(self.version, 5, 8, xid);
        let mut bytes: Vec<u8> = Vec::new();
        header.marshal(&mut bytes);
        stream.write_all(&bytes).unwrap();
    }

    pub fn packetIn(&self, xid: u32, payload: EthernetFrame, stream: &mut TcpStream) {}

    pub fn send(&self, xid: u32, message: u8, payload: &Vec<u8>, stream: &mut TcpStream) {
        let length = size_of::<OfpHeader>() + payload.len();
        let header = OfpHeader::new(self.version, message, length as u16, xid);
        let mut bytes: Vec<u8> = Vec::new();

        header.marshal(&mut bytes);

        stream.write_all(&bytes).unwrap();
    }
}
