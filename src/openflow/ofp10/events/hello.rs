//! OpenFlow 1.0 Hello Message
//!
//! This module implements the hello message handling for OpenFlow 1.0.
//! Hello messages are exchanged between the controller and switch during
//! the initial connection establishment.
//!
//! The module provides:
//! - Hello message event structure
//! - Message marshaling implementation
//! - Empty payload handling

use crate::openflow::ofp10::{MessageMarshal, Msg};

/// Represents a hello message for OpenFlow 1.0
///
/// Hello messages are exchanged between the controller and switch during
/// the initial connection establishment. They are used to verify protocol
/// version compatibility and initiate the connection.
#[derive(Debug)]
pub struct HelloEvent {}

impl HelloEvent {
    /// Creates a new hello event
    ///
    /// # Returns
    /// A new HelloEvent instance
    pub fn new() -> Self {
        HelloEvent {}
    }
}

impl MessageMarshal for HelloEvent {
    /// Serializes the hello message into a byte buffer
    ///
    /// Hello messages have no payload, so this is a no-op.
    ///
    /// # Arguments
    /// * `_` - Unused byte buffer
    fn marshal(&self, _: &mut Vec<u8>) {}

    /// Returns the message type code for hello message
    ///
    /// # Returns
    /// The Msg::Hello variant
    fn msg_code(&self) -> Msg {
        Msg::Hello
    }

    /// Returns the size of the message payload
    ///
    /// Hello messages have no payload, so this returns 0.
    ///
    /// # Returns
    /// 0 (no payload)
    fn size_of(&self) -> usize {
        0
    }

    /// Returns the message type code as a usize
    ///
    /// # Returns
    /// The numeric value of the hello message type
    fn msg_usize(&self) -> usize {
        Msg::Hello as usize
    }
}
