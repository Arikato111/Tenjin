use std::future::Future;

use super::{
    events::{echo_reply::EchoReplyEvent, EchoRequestEvent},
    tcp_listener_handler, FeaturesReplyEvent, MessageMarshal, OfpMsgEvent, Openflow13,
    OpenflowHeader,
};
use crate::openflow::ofp13::{ErrorEvent, Msg, PacketInEvent};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

pub trait ControllerFrame13: Send {
    fn ofp(&self) -> Openflow13 {
        Openflow13::new()
    }
    fn packet_in_handler(
        &mut self,
        xid: u32,
        packetin: PacketInEvent,
        stream: &mut TcpStream,
    ) -> impl Future<Output = ()> + Send;
    fn new() -> Self;

    fn listener(&self, address: &str) -> impl Future<Output = ()> + Send
    where
        Self: Sized + 'static,
        Self: Clone,
        Self: Sync,
    {
        async move {
            println!("server run at {}", address);
            let _ = tcp_listener_handler(address, self).await;
        }
    }

    fn handle_header(&mut self, buf: &mut Vec<u8>) -> Option<(u8, usize, u32)> {
        let ofp_header = self.ofp().header_parse(buf);
        match ofp_header {
            Ok(header) => Some((header.message(), header.pkt_size(), header.xid())),
            Err(_) => None,
        }
    }

    fn request_handler(
        &mut self,
        buf: &mut Vec<u8>,
        stream: &mut TcpStream,
    ) -> impl Future<Output = ()> + Send
    where
        Self: Sync,
    {
        async move {
            let ofp = self.ofp();
            let (message, pkt_size, xid) = match self.handle_header(buf) {
                Some(header) => header,
                None => return,
            };
            let mut payload = vec![0u8; pkt_size];
            let _ = stream.read(&mut payload).await;
            let message = ofp.msg_parse(message);
            match message {
                Msg::Hello => self.hello_handler(xid, stream).await,
                Msg::Error => {
                    if let Ok(error) = ErrorEvent::parse(&payload) {
                        self.error_handler(error)
                    }
                }
                Msg::EchoRequest => {
                    self.echo_request_handler(xid, EchoRequestEvent::new(payload), stream)
                        .await
                }
                Msg::FeaturesReply => {
                    if let Ok(features) = FeaturesReplyEvent::parse(&payload) {
                        self.switch_features_handler(xid, features, stream).await
                    }
                }
                Msg::PacketIn => {
                    if let Ok(pkt_in) = PacketInEvent::parse(&payload) {
                        self.packet_in_handler(xid, pkt_in, stream).await
                    }
                }
                _ => (),
            }
        }
    }

    fn send_msg<MSM: MessageMarshal + std::marker::Send>(
        &self,
        msg: MSM,
        xid: u32,
        stream: &mut TcpStream,
    ) -> impl Future<Output = ()> + Send
    where
        Self: Sync,
    {
        async move {
            let ofp = self.ofp();
            let mut header_bytes: Vec<u8> = Vec::new();
            let mut body_bytes: Vec<u8> = Vec::new();

            msg.marshal(&mut body_bytes);
            let ofp_header = ofp.header(msg.msg_usize() as u8, body_bytes.len() as u16, xid);
            ofp_header.marshal(&mut header_bytes);
            header_bytes.append(&mut body_bytes);
            let _ = stream.write_all(&header_bytes).await;
        }
    }

    /**
     * for handle message
     */
    fn hello_handler(&self, xid: u32, stream: &mut TcpStream) -> impl Future<Output = ()> + Send
    where
        Self: Sync,
    {
        async move {
            self.send_msg(self.ofp().fetures_req(), xid, stream).await;
        }
    }
    fn error_handler(&self, error: ErrorEvent) {
        println!("Error {:?} payload: {:x?}", error.error_type, error.payload);
    }
    fn echo_request_handler(
        &self,
        xid: u32,
        echo: EchoRequestEvent,
        stream: &mut TcpStream,
    ) -> impl Future<Output = ()> + Send
    where
        Self: Sync,
    {
        async move {
            self.send_msg(EchoReplyEvent::new(echo.payload), xid, stream)
                .await;
        }
    }
    #[allow(unused)]
    fn switch_features_handler(
        &self,
        xid: u32,
        features_reply: FeaturesReplyEvent,
        stream: &mut TcpStream,
    ) -> impl std::future::Future<Output = ()> + Send {
        async {}
    }
}
