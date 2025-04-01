//! OpenFlow 1.0 Features Request
//!
//! This module implements the features request message handling for OpenFlow 1.0.
//! Features requests are used to query the switch about its capabilities and
//! supported features.
//!
//! The module provides:
//! - Features request event structure
//! - Message marshaling implementation
//! - Empty payload handling

use crate::openflow::ofp10::{MessageMarshal, Msg};

/// Represents a features request message to the switch
///
/// Features requests are sent to query the switch about its capabilities,
/// including supported ports, tables, and actions. The switch responds with
/// a features reply containing this information.
#[derive(Debug)]
pub struct FeaturesReqEvent {}

impl FeaturesReqEvent {
    /// Creates a new features request event
    ///
    /// # Returns
    /// A new FeaturesReqEvent instance
    pub fn new() -> Self {
        FeaturesReqEvent {}
    }
}

impl MessageMarshal for FeaturesReqEvent {
    /// Serializes the features request message into a byte buffer
    ///
    /// Features requests have no payload, so this is a no-op.
    ///
    /// # Arguments
    /// * `_` - Unused byte buffer
    fn marshal(&self, _: &mut Vec<u8>) {}

    /// Returns the message type code for features request
    ///
    /// # Returns
    /// The Msg::FeaturesRequest variant
    fn msg_code(&self) -> Msg {
        Msg::FeaturesRequest
    }

    /// Returns the size of the message payload
    ///
    /// Features requests have no payload, so this returns 0.
    ///
    /// # Returns
    /// 0 (no payload)
    fn size_of(&self) -> usize {
        0
    }

    /// Returns the message type code as a usize
    ///
    /// # Returns
    /// The numeric value of the features request message type
    fn msg_usize(&self) -> usize {
        Msg::FeaturesRequest as usize
    }
}
