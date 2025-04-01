//! OpenFlow 1.0 Echo Request
//! 
//! This module implements the echo request message handling for OpenFlow 1.0.
//! Echo requests are used to verify the connection between the controller and switch.
//! 
//! The module provides:
//! - Echo request event structure
//! - Message marshaling implementation
//! - Payload handling

use std::io::Write;

use crate::openflow::ofp10::{self, MessageMarshal, Msg};

/// Represents an echo request message to the switch
/// 
/// Echo requests are sent to verify the connection between the controller and switch.
/// The switch should respond with an echo reply containing the same payload.
#[derive(Debug)]
pub struct EchoRequestEvent {
    /// The payload data to be echoed back by the switch
    pub payload: Vec<u8>,
}

impl EchoRequestEvent {
    /// Creates a new echo request event
    /// 
    /// # Arguments
    /// * `payload` - The payload data to be echoed back
    /// 
    /// # Returns
    /// A new EchoRequestEvent instance
    pub fn new(payload: Vec<u8>) -> Self {
        Self { payload }
    }
}

impl MessageMarshal for EchoRequestEvent {
    /// Serializes the echo request message into a byte buffer
    /// 
    /// # Arguments
    /// * `bytes` - Mutable reference to the byte buffer to write to
    fn marshal(&self, bytes: &mut Vec<u8>) {
        let _ = bytes.write_all(&self.payload);
    }

    /// Returns the message type code for echo request
    /// 
    /// # Returns
    /// The Msg::EchoRequest variant
    fn msg_code(&self) -> ofp10::Msg {
        Msg::EchoRequest
    }

    /// Returns the message type code as a usize
    /// 
    /// # Returns
    /// The numeric value of the echo request message type
    fn msg_usize(&self) -> usize {
        Msg::EchoRequest as usize
    }

    /// Returns the size of the message payload
    /// 
    /// # Returns
    /// The length of the payload in bytes
    fn size_of(&self) -> usize {
        self.payload.len()
    }
}
