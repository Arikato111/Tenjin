//! OpenFlow v1.3 Hello Message Implementation
//!
//! This module implements the Hello message type used in OpenFlow v1.3 protocol.
//! The Hello message is the first message exchanged between the controller and switch
//! to establish a connection.

use crate::openflow::ofp13::{MessageMarshal, Msg};

/// Represents an OpenFlow v1.3 Hello message
///
/// The Hello message is used to establish a connection between the controller and switch.
/// It has no payload and is the first message sent in the OpenFlow protocol handshake.
pub struct HelloEvent {}

impl HelloEvent {
    /// Creates a new Hello message
    ///
    /// # Returns
    /// A new HelloEvent instance
    pub fn new() -> Self {
        HelloEvent {}
    }
}

/// Implements message marshaling for HelloEvent
impl MessageMarshal for HelloEvent {
    /// Marshals the Hello message into a byte vector
    ///
    /// # Arguments
    /// * `_` - The target byte vector (unused as Hello has no payload)
    fn marshal(&self, _: &mut Vec<u8>) {}

    /// Returns the OpenFlow message code for Hello
    ///
    /// # Returns
    /// The Msg::Hello enum variant
    fn msg_code(&self) -> Msg {
        Msg::Hello
    }

    /// Returns the size of the Hello message
    ///
    /// # Returns
    /// 0 as Hello message has no payload
    fn size_of(&self) -> usize {
        0
    }

    /// Returns the message code as a usize
    ///
    /// # Returns
    /// The numeric value of the Hello message code
    fn msg_usize(&self) -> usize {
        Msg::Hello as usize
    }
}
