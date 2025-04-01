//! OpenFlow 1.3 Controller Frame
//! 
//! This module provides the controller frame implementation for OpenFlow 1.3,
//! handling the communication between the controller and switches. It includes
//! message parsing, event handling, and TCP communication functionality.
//! 
//! The controller frame is responsible for:
//! - Managing TCP connections with switches
//! - Parsing and handling OpenFlow messages
//! - Processing various OpenFlow events
//! - Sending responses and commands to switches
//! 
//! The main component is the `ControllerFrame13` trait which defines the interface
//! for implementing OpenFlow 1.3 controllers. Implementors can customize the behavior
//! of message handling and event processing while maintaining compatibility with
//! the OpenFlow 1.3 protocol specification.
//! 
//! OpenFlow 1.3 introduces several improvements over 1.0, including:
//! - Enhanced message types and event handling
//! - Improved error reporting with payload information
//! - More sophisticated switch feature negotiation
//! - Better support for multiple tables and groups

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

/// Trait defining the controller frame functionality for OpenFlow 1.3
/// 
/// This trait provides the core functionality needed to implement an OpenFlow 1.3
/// controller, including message handling, TCP communication, and event processing.
/// Implementors of this trait can create custom controllers with specific behaviors
/// for handling different OpenFlow messages and events.
pub trait ControllerFrame13: Send {
    /// Returns a new OpenFlow 1.3 instance
    fn ofp(&self) -> Openflow13 {
        Openflow13::new()
    }

    /// Handles incoming packet-in events
    /// 
    /// # Arguments
    /// * `xid` - Transaction ID
    /// * `packetin` - The packet-in event to handle
    /// * `stream` - TCP stream for communication
    fn packet_in_handler(
        &mut self,
        xid: u32,
        packetin: PacketInEvent,
        stream: &mut TcpStream,
    ) -> impl Future<Output = ()> + Send;

    /// Creates a new instance of the controller frame
    fn new() -> Self;

    /// Starts the TCP listener for accepting switch connections
    /// 
    /// # Arguments
    /// * `address` - The address to listen on
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

    /// Parses the OpenFlow header from a buffer
    /// 
    /// # Arguments
    /// * `buf` - Buffer containing the header data
    /// 
    /// # Returns
    /// Option containing tuple of (message type, payload size, transaction ID)
    fn handle_header(&mut self, buf: &mut Vec<u8>) -> Option<(u8, usize, u32)> {
        let ofp_header = self.ofp().header_parse(buf);
        match ofp_header {
            Ok(header) => Some((header.message(), header.pkt_size(), header.xid())),
            Err(_) => None,
        }
    }

    /// Handles incoming OpenFlow messages
    /// 
    /// # Arguments
    /// * `buf` - Buffer containing the message data
    /// * `stream` - TCP stream for communication
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

    /// Sends an OpenFlow message over the TCP stream
    /// 
    /// # Arguments
    /// * `msg` - The message to send
    /// * `xid` - Transaction ID
    /// * `stream` - TCP stream for communication
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

    /// Handles OpenFlow Hello messages
    /// 
    /// # Arguments
    /// * `xid` - Transaction ID
    /// * `stream` - TCP stream for communication
    fn hello_handler(&self, xid: u32, stream: &mut TcpStream) -> impl Future<Output = ()> + Send
    where
        Self: Sync,
    {
        async move {
            self.send_msg(self.ofp().fetures_req(), xid, stream).await;
        }
    }

    /// Handles OpenFlow Error messages
    /// 
    /// # Arguments
    /// * `error` - The error event to handle
    fn error_handler(&self, error: ErrorEvent) {
        println!("Error {:?} payload: {:x?}", error.error_type, error.payload);
    }

    /// Handles OpenFlow Echo Request messages
    /// 
    /// # Arguments
    /// * `xid` - Transaction ID
    /// * `echo` - The echo request event to handle
    /// * `stream` - TCP stream for communication
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

    /// Handles OpenFlow Features Reply messages
    /// 
    /// # Arguments
    /// * `xid` - Transaction ID
    /// * `features_reply` - The features reply event to handle
    /// * `stream` - TCP stream for communication
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
