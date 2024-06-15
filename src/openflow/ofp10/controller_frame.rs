use crate::openflow::ofp10::{self, ErrorEvent, Msg, PacketInEvent};
use std::{
    io::{Read, Write},
    net::TcpStream,
};

use super::{
    events::{echo_reply::EchoReplyEvent, EchoRequestEvent},
    tcp_listener_handler, MessageMarshal, OfpMsgEvent, Openflow10, OpenflowHeader,
};

pub trait ControllerFrame10 {
    fn ofp(&self) -> ofp10::Openflow10 {
        Openflow10::new()
    }
    fn packet_in_handler(&mut self, xid: u32, packetin: PacketInEvent, stream: &mut TcpStream);
    fn new() -> Self;

    fn listener(address: &str) {
        tcp_listener_handler(address);
    }

    fn handle_header(&mut self, buf: &mut Vec<u8>) -> (u8, usize, u32) {
        let ofp_header = self.ofp().header_parse(&buf);
        (
            ofp_header.message(),
            ofp_header.pkt_size(),
            ofp_header.xid(),
        )
    }

    fn request_handler(&mut self, buf: &mut Vec<u8>, stream: &mut TcpStream) {
        let ofp = self.ofp();
        let (message, pkt_size, xid) = self.handle_header(buf);
        let mut payload = vec![0u8; pkt_size];
        let _ = stream.read(&mut payload);
        let message = ofp.msg_parse(message as u8);
        match message {
            Msg::Hello => self.hello_handler(xid, stream),
            Msg::Error => self.error_handler(ErrorEvent::parse(&payload)),
            Msg::EchoRequest => {
                self.echo_request_handler(xid, EchoRequestEvent::new(payload), stream)
            }
            Msg::PacketIn => {
                self.packet_in_handler(xid, PacketInEvent::parse(&payload), stream);
            }
            Msg::PacketOut => (),
            _ => (),
        }
    }

    fn send_msg<MSM: MessageMarshal>(&self, msg: MSM, xid: u32, stream: &mut TcpStream) {
        let ofp = self.ofp();
        let mut header_bytes: Vec<u8> = Vec::new();
        let mut body_bytes: Vec<u8> = Vec::new();

        msg.marshal(&mut body_bytes);
        let ofp_header = ofp.header(msg.msg_usize() as u8, body_bytes.len() as u16, xid);
        ofp_header.marshal(&mut header_bytes);
        header_bytes.append(&mut body_bytes);
        let _ = stream.write_all(&header_bytes);
    }

    /**
     * for handle message
     */
    fn hello_handler(&self, xid: u32, stream: &mut TcpStream) {
        self.send_msg(self.ofp().fetures_req(), xid, stream);
    }
    fn error_handler(&self, error: ErrorEvent) {
        println!("Error {:?}", error.error_type);
    }
    fn echo_request_handler(&self, xid: u32, echo: EchoRequestEvent, stream: &mut TcpStream) {
        self.send_msg(EchoReplyEvent::new(echo.payload), xid, stream);
    }
}
