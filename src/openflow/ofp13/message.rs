//! OpenFlow 1.3 Message Types
//! 
//! This module defines the message types used in OpenFlow 1.3 protocol communication.
//! OpenFlow 1.3 introduces several new message types and features compared to 1.0,
//! including support for multiple tables, groups, meters, and enhanced statistics.

use std::mem::transmute;

/// Represents all possible OpenFlow 1.3 message types
/// 
/// Each variant corresponds to a specific message type in the OpenFlow 1.3 protocol.
/// The values match the official OpenFlow 1.3 specification message type codes.
#[repr(u8)]
#[derive(Clone)]
pub enum Msg {
    /// Initial handshake message
    Hello = 0,
    /// Error notification message
    Error = 1,
    /// Echo request for connection testing
    EchoRequest = 2,
    /// Echo reply for connection testing
    EchoReply = 3,
    /// Experimenter-specific message
    Experimenter = 4,
    /// Request switch features
    FeaturesRequest = 5,
    /// Switch features reply
    FeaturesReply = 6,
    /// Request switch configuration
    ConfigRequest = 7,
    /// Switch configuration reply
    ConfigReply = 8,
    /// Set switch configuration
    SetConfig = 9,
    /// Packet received by switch
    PacketIn = 10,
    /// Flow removed notification
    FlowRemove = 11,
    /// Port status change notification
    PortStatus = 12,
    /// Packet to be sent by switch
    PacketOut = 13,
    /// Flow table modification
    FlowMod = 14,
    /// Group table modification
    GroupMod = 15,
    /// Port configuration modification
    PortMod = 16,
    /// Table configuration modification
    TableMod = 17,
    /// Multipart message request
    MultipartRequest = 18,
    /// Multipart message reply
    MultipartReply = 19,
    /// Request to ensure all previous messages are processed
    BarrierRequest = 20,
    /// Barrier reply confirmation
    BarrierReply = 21,
    /// Get configuration request
    GetConfigRequest = 22,
    /// Get configuration reply
    GetConfigReply = 23,
    /// Controller role request
    RoleRequest = 24,
    /// Controller role reply
    RoleReply = 25,
    /// Get asynchronous message configuration request
    GetAsyncRequest = 26,
    /// Get asynchronous message configuration reply
    GetAsyncReply = 27,
    /// Set asynchronous message configuration
    SetAsync = 28,
    /// Meter table modification
    MeterMod = 29,
    /// Unknown or unsupported message type
    NotFound = 0xff,
}

impl Msg {
    /// Converts the message type to its corresponding integer value
    pub fn to_int(&self) -> u8 {
        self.clone().into()
    }

    /// Creates a message type from an integer value
    /// 
    /// # Arguments
    /// * `msg_code` - The integer code representing the message type
    /// 
    /// # Returns
    /// The corresponding Msg enum variant, or NotFound if the code is invalid
    pub fn from(msg_code: u8) -> Self {
        if msg_code > 21 {
            return Self::NotFound;
        }
        unsafe { transmute::<u8, Msg>(msg_code) }
    }
}

/// Implementation of From trait to convert Msg to u8
impl From<Msg> for u8 {
    fn from(value: Msg) -> Self {
        value as u8
    }
}
