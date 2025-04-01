//! OpenFlow v1.3 Echo Request Message Implementation
//!
//! This module implements the Echo Request message type used in OpenFlow v1.3 protocol.
//! The Echo Request message is used to verify the liveness of the connection between
//! the controller and switch.

use std::io::Write;

use crate::openflow::ofp13::{self, MessageMarshal, Msg};

/// Represents an OpenFlow v1.3 Echo Request message
///
/// The Echo Request message is used to verify the liveness of the connection
/// between the controller and switch. It can contain an optional payload that
/// will be echoed back in the Echo Reply message.
pub struct EchoRequestEvent {
    /// Optional payload data to be echoed back
    pub payload: Vec<u8>,
}

impl EchoRequestEvent {
    /// Creates a new Echo Request message
    ///
    /// # Arguments
    /// * `payload` - Optional payload data to be echoed back
    ///
    /// # Returns
    /// A new EchoRequestEvent instance
    pub fn new(payload: Vec<u8>) -> Self {
        Self { payload }
    }
}

/// Implements message marshaling for EchoRequestEvent
impl MessageMarshal for EchoRequestEvent {
    /// Marshals the Echo Request message into a byte vector
    ///
    /// # Arguments
    /// * `bytes` - The target byte vector to write the message data to
    fn marshal(&self, bytes: &mut Vec<u8>) {
        let _ = bytes.write_all(&self.payload);
    }

    /// Returns the OpenFlow message code for Echo Request
    ///
    /// # Returns
    /// The Msg::EchoRequest enum variant
    fn msg_code(&self) -> ofp13::Msg {
        Msg::EchoRequest
    }

    /// Returns the message code as a usize
    ///
    /// # Returns
    /// The numeric value of the Echo Request message code
    fn msg_usize(&self) -> usize {
        Msg::EchoRequest as usize
    }

    /// Returns the size of the Echo Request message
    ///
    /// # Returns
    /// The length of the payload in bytes
    fn size_of(&self) -> usize {
        self.payload.len()
    }
}
