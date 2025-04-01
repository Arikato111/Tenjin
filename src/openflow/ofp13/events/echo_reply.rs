//! OpenFlow v1.3 Echo Reply Message Implementation
//!
//! This module implements the Echo Reply message type used in OpenFlow v1.3 protocol.
//! The Echo Reply message is sent by the switch in response to an Echo Request,
//! echoing back the payload to verify connection liveness.

use std::io::Write;

use crate::openflow::ofp13::{self, MessageMarshal, Msg};

/// Represents an OpenFlow v1.3 Echo Reply message
///
/// The Echo Reply message is sent by the switch in response to an Echo Request.
/// It contains the same payload as the Echo Request message, allowing the controller
/// to verify the connection is still alive and functioning correctly.
pub struct EchoReplyEvent {
    /// The payload data echoed back from the Echo Request
    pub payload: Vec<u8>,
}

impl EchoReplyEvent {
    /// Creates a new Echo Reply message
    ///
    /// # Arguments
    /// * `payload` - The payload data to echo back
    ///
    /// # Returns
    /// A new EchoReplyEvent instance
    pub fn new(payload: Vec<u8>) -> Self {
        Self { payload }
    }
}

/// Implements message marshaling for EchoReplyEvent
impl MessageMarshal for EchoReplyEvent {
    /// Marshals the Echo Reply message into a byte vector
    ///
    /// # Arguments
    /// * `bytes` - The target byte vector to write the message data to
    fn marshal(&self, bytes: &mut Vec<u8>) {
        let _ = bytes.write_all(&self.payload);
    }

    /// Returns the OpenFlow message code for Echo Reply
    ///
    /// # Returns
    /// The Msg::EchoReply enum variant
    fn msg_code(&self) -> ofp13::Msg {
        Msg::EchoReply
    }

    /// Returns the message code as a usize
    ///
    /// # Returns
    /// The numeric value of the Echo Reply message code
    fn msg_usize(&self) -> usize {
        Msg::EchoReply as usize
    }

    /// Returns the size of the Echo Reply message
    ///
    /// # Returns
    /// The length of the payload in bytes
    fn size_of(&self) -> usize {
        self.payload.len()
    }
}
