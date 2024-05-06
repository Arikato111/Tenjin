use std::{
    collections::HashMap,
    io::{Read, Write},
    net::TcpStream,
};

use crate::tcp_listener::tcp_listener_handler;

use super::{
    events::PacketInEvent,
    messages::{
        traiter::{MessageMarshal, OfpMsgEvent},
        OfpMsg,
    },
    OfpHeader,
};

pub struct Controller<OME: OfpMsgEvent> {
    /*
     * pub is temporary, remove soon;
     * for test in main func
     */
    pub ofp: OME,
    mac_to_port: HashMap<u64, u16>,
}

impl<OME: OfpMsgEvent> Controller<OME> {
    pub const OFP_1_0: u8 = 1;
    pub fn new(ofp: OME) -> Self {
        Self {
            ofp,
            mac_to_port: HashMap::new(),
        }
    }

    pub fn listener(address: &str, ofp: OME) {
        tcp_listener_handler(ofp.version() as u8, address);
    }

    pub fn request_handler(&mut self, buf: &mut Vec<u8>, stream: &mut TcpStream) {
        let ofp_header = OfpHeader::parse(&buf);
        let mut payload = vec![0u8; ofp_header.pkt_size()];
        let _ = stream.read(&mut payload);
        let message = self.ofp.msg_parse(ofp_header.message as u16);
        match message {
            OfpMsg::Hello => self.send_msg(self.ofp.fetures_req(), ofp_header.xid, stream),
            OfpMsg::FeaturesReq => todo!(),
            OfpMsg::PacketIn => {
                self.packet_in_handler(ofp_header.xid, PacketInEvent::parse(&payload), stream);
            }
            OfpMsg::FlowMod => todo!(),
            OfpMsg::NotFound => todo!(),
        }
    }

    pub fn send_msg<T: MessageMarshal>(&self, msg: T, xid: u32, stream: &mut TcpStream) {
        let mut header_bytes: Vec<u8> = Vec::new();
        let mut body_bytes: Vec<u8> = Vec::new();

        msg.marshal(&mut body_bytes);
        let ofp_header =
            self.ofp
                .header(msg.msg_usize(&self.ofp) as u8, body_bytes.len() as u16, xid);
        ofp_header.marshal(&mut header_bytes);
        header_bytes.append(&mut body_bytes);
        let _ = stream.write_all(&header_bytes);
    }

    /**
     * example of sending message
     */
    pub fn hello(&self, stream: &mut TcpStream) {
        let hello_msg = self.ofp.hello_event();
        self.send_msg(hello_msg, 0, stream);
    }

    pub fn fetures_req(&self, xid: u32, stream: &mut TcpStream) {
        let fetreq_msg = self.ofp.fetures_req();
        self.send_msg(fetreq_msg, xid, stream);
    }

    pub fn packet_in_handler(&mut self, xid: u32, packetin: PacketInEvent, stream: &mut TcpStream) {
        let ether = packetin.payload;
        self.mac_to_port.insert(ether.mac_src, packetin.port);
    }
}
