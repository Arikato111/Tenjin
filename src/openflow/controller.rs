use std::{collections::HashMap, io::Write, net::TcpStream};

use super::{
    events::{FeaturesReq, HelloEvent, PacketInEvent},
    trait_marshal::MessageMarshal,
    OfpHeader,
};

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

    pub fn send_msg<T: MessageMarshal>(&self, msg: T, xid: u32, stream: &mut TcpStream) {
        let mut header_bytes: Vec<u8> = Vec::new();
        let mut body_bytes: Vec<u8> = Vec::new();
        msg.marshal(&mut body_bytes);
        let ofpheader = OfpHeader::new(
            self.version,
            msg.msg_code() as u8,
            body_bytes.len() as u16,
            xid,
        );
        ofpheader.marshal(&mut header_bytes);
        header_bytes.append(&mut body_bytes);
        let _ = stream.write_all(&header_bytes);
    }

    /**
     * example of sending message
     */
    pub fn hello(&self, stream: &mut TcpStream) {
        let hello_msg = HelloEvent::new();
        self.send_msg(hello_msg, 0, stream);
    }

    pub fn fetures_req(&self, xid: u32, stream: &mut TcpStream) {
        let fetreq_msg = FeaturesReq::new();
        self.send_msg(fetreq_msg, xid, stream);
    }

    pub fn packet_in(&mut self, xid: u32, packetin: PacketInEvent, stream: &mut TcpStream) {
        let ether = packetin.payload;
        self.mac_to_port.insert(ether.mac_src, packetin.port);
    }
}
