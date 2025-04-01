//! OpenFlow 1.0 Echo Reply
//! 
//! This module implements the echo reply message handling for OpenFlow 1.0.
//! Echo replies are used to verify the connection between the controller and switch.
//! 
//! The module provides:
//! - Echo reply event structure
//! - Message marshaling implementation
//! - Payload handling

use std::io::Write;

use crate::openflow::ofp10::{self, MessageMarshal};

/// Represents an echo reply message from the switch
/// 
/// Echo replies are sent in response to echo requests to verify the connection
/// between the controller and switch. The payload is typically echoed back
/// unchanged from the echo request.
#[derive(Debug)]
pub struct EchoReplyEvent {
    /// The payload data to be echoed back
    pub payload: Vec<u8>,
}

impl EchoReplyEvent {
    /// Creates a new echo reply event
    /// 
    /// # Arguments
    /// * `payload` - The payload data to be echoed back
    /// 
    /// # Returns
    /// A new EchoReplyEvent instance
    pub fn new(payload: Vec<u8>) -> Self {
        Self { payload }
    }
}

impl MessageMarshal for EchoReplyEvent {
    /// Serializes the echo reply message into a byte buffer
    /// 
    /// # Arguments
    /// * `bytes` - Mutable reference to the byte buffer to write to
    fn marshal(&self, bytes: &mut Vec<u8>) {
        let _ = bytes.write_all(&self.payload);
    }

    /// Returns the message type code for echo reply
    /// 
    /// # Returns
    /// The Msg::EchoReply variant
    fn msg_code(&self) -> ofp10::Msg {
        ofp10::Msg::EchoReply
    }

    /// Returns the message type code as a usize
    /// 
    /// # Returns
    /// The numeric value of the echo reply message type
    fn msg_usize(&self) -> usize {
        ofp10::Msg::EchoReply as usize
    }

    /// Returns the size of the message payload
    /// 
    /// # Returns
    /// The length of the payload in bytes
    fn size_of(&self) -> usize {
        self.payload.len()
    }
}
