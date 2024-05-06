use std::{
    io::{Read, Write},
    net::TcpStream,
};

use super::{
    events::PacketInEvent,
    messages::{
        traiter::{MessageMarshal, OfpMsgEvent},
        OfpMsg,
    },
    tcp_listener_handler, OfpHeader,
};

pub trait ControllerFrame<OME: OfpMsgEvent> {
    fn get_ofp(&self) -> &impl OfpMsgEvent;
    fn packet_in_handler(&mut self, xid: u32, packetin: PacketInEvent, stream: &mut TcpStream);
    fn new(ofp: OME) -> Self;
    
    fn listener(address: &str, ofp: OME) {
        tcp_listener_handler::<OME>(address, ofp.version() as u8);
    }

    fn request_handler(&mut self, buf: &mut Vec<u8>, stream: &mut TcpStream) {
        let ofp = self.get_ofp();
        let ofp_header = OfpHeader::parse(&buf);
        let mut payload = vec![0u8; ofp_header.pkt_size()];
        let _ = stream.read(&mut payload);
        let message = ofp.msg_parse(ofp_header.message as u16);
        match message {
            OfpMsg::Hello => self.send_msg(ofp.fetures_req(), ofp_header.xid, stream),
            OfpMsg::FeaturesReq => todo!(),
            OfpMsg::PacketIn => {
                self.packet_in_handler(ofp_header.xid, PacketInEvent::parse(&payload), stream);
            }
            OfpMsg::FlowMod => todo!(),
            OfpMsg::NotFound => todo!(),
        }
    }

    fn send_msg<MSM: MessageMarshal>(&self, msg: MSM, xid: u32, stream: &mut TcpStream) {
        let ofp = self.get_ofp();
        let mut header_bytes: Vec<u8> = Vec::new();
        let mut body_bytes: Vec<u8> = Vec::new();

        msg.marshal(&mut body_bytes);
        let ofp_header = ofp.header(msg.msg_usize(ofp) as u8, body_bytes.len() as u16, xid);
        ofp_header.marshal(&mut header_bytes);
        header_bytes.append(&mut body_bytes);
        let _ = stream.write_all(&header_bytes);
    }
}

