//! OpenFlow v1.3 Features Request Message Implementation
//! 
//! This module implements the Features Request message type used in OpenFlow v1.3 protocol.
//! The Features Request message is sent by the controller to query the switch about its
//! capabilities and features.

use crate::openflow::ofp13::{MessageMarshal, Msg};

/// Represents an OpenFlow v1.3 Features Request message
/// 
/// The Features Request message is used by the controller to query the switch about its
/// capabilities, including supported OpenFlow versions, datapath ID, and port information.
/// This message has no payload and is part of the initial handshake process.
pub struct FeaturesReqEvent {}

impl FeaturesReqEvent {
    /// Creates a new Features Request message
    /// 
    /// # Returns
    /// A new FeaturesReqEvent instance
    pub fn new() -> Self {
        FeaturesReqEvent {}
    }
}

/// Implements message marshaling for FeaturesReqEvent
impl MessageMarshal for FeaturesReqEvent {
    /// Marshals the Features Request message into a byte vector
    /// 
    /// # Arguments
    /// * `_` - The target byte vector (unused as Features Request has no payload)
    fn marshal(&self, _: &mut Vec<u8>) {}

    /// Returns the OpenFlow message code for Features Request
    /// 
    /// # Returns
    /// The Msg::FeaturesRequest enum variant
    fn msg_code(&self) -> Msg {
        Msg::FeaturesRequest
    }

    /// Returns the size of the Features Request message
    /// 
    /// # Returns
    /// 0 as Features Request message has no payload
    fn size_of(&self) -> usize {
        0
    }

    /// Returns the message code as a usize
    /// 
    /// # Returns
    /// The numeric value of the Features Request message code
    fn msg_usize(&self) -> usize {
        Msg::FeaturesRequest as usize
    }
}
