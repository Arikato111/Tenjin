//! OpenFlow 1.0 Message Types
//! 
//! This module defines the message types used in OpenFlow 1.0 protocol communication.
//! Each message type corresponds to a specific OpenFlow protocol message that can be
//! exchanged between the controller and switch.

use std::mem::transmute;

/// Represents all possible OpenFlow 1.0 message types
/// 
/// Each variant corresponds to a specific message type in the OpenFlow 1.0 protocol.
/// The values match the official OpenFlow 1.0 specification message type codes.
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
    /// Vendor-specific message
    Vendor = 4,
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
    /// Port configuration modification
    PortMod = 15,
    /// Statistics request
    StatsRequest = 16,
    /// Statistics reply
    StateReply = 17,
    /// Request to ensure all previous messages are processed
    BarrierRequest = 18,
    /// Barrier reply confirmation
    BarrierReply = 19,
    /// Queue configuration request
    QueueGetConfigRequest = 20,
    /// Queue configuration reply
    QueueGetConfigReply = 21,
    /// Unknown or unsupported message type
    NotFound = 0xff,
}

impl Msg {
    /// Converts the message type to its corresponding integer value
    pub fn to_int(&self) -> u8 {
        self.clone() as u8
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
