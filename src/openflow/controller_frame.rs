use crate::openflow::ofp10::{
    traiter::{MessageMarshal, OfpMsgEvent},
    ErrorEvent, Msg, PacketInEvent,
};
use std::{
    io::{Read, Write},
    net::TcpStream,
};

use super::tcp_listener::tcp_listener_handler;

pub trait ControllerFrame<OME: OfpMsgEvent> {
    fn get_ofp(&self) -> &impl OfpMsgEvent;
    fn packet_in_handler(&mut self, xid: u32, packetin: PacketInEvent, stream: &mut TcpStream);
    fn new() -> Self;

    fn listener(address: &str) {
        tcp_listener_handler::<OME>(address);
    }

    fn handle_header(&mut self, buf: &mut Vec<u8>) -> (u8, usize, u32) {
        let ofp_header = self.get_ofp().header_parse(&buf);
        (
            ofp_header.message(),
            ofp_header.pkt_size(),
            ofp_header.xid(),
        )
    }

    fn request_handler(&mut self, buf: &mut Vec<u8>, stream: &mut TcpStream) {
        let (message, pkt_size, xid) = self.handle_header(buf);
        let mut payload = vec![0u8; pkt_size];
        let _ = stream.read(&mut payload);
        let message = self.get_ofp().msg_parse(message as u16);
        match message {
            Msg::Hello => self.send_msg(self.get_ofp().fetures_req(), xid, stream),
            Msg::Error => {
                let error = ErrorEvent::parse(&payload);
                println!("Error {:?}", error.error_type);
                ()
            }
            Msg::FeaturesReq => (),
            Msg::PacketIn => {
                self.packet_in_handler(xid, PacketInEvent::parse(&payload), stream);
            }
            Msg::PacketOut => (),
            Msg::FlowMod => (),
            Msg::NotFound => (),
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
