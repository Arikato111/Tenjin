//! OpenFlow 1.0 Event Traits
//!
//! This module defines the core traits used for handling OpenFlow 1.0 events
//! and message marshaling. These traits provide the interface for working with
//! OpenFlow messages and events in a type-safe manner.
//!
//! The module provides:
//! - Message marshaling functionality
//! - OpenFlow message event handling
//! - Header parsing and creation
//! - Event creation and management

use std::io::Error;

use crate::openflow::ofp10::{
    events::{Action, FeaturesReqEvent, HelloEvent, PacketOutEvent, Payload},
    ofp_header::OfpHeader,
    Msg,
};

/// Trait for marshaling OpenFlow messages to bytes
///
/// This trait provides functionality for converting OpenFlow messages
/// into their byte representation for transmission over the network.
/// It is used with the Controller's send_msg functionality.
pub trait MessageMarshal {
    /// Serializes the message into a byte buffer
    ///
    /// # Arguments
    /// * `bytes` - Mutable reference to the byte buffer to write to
    fn marshal(&self, bytes: &mut Vec<u8>);

    /// Returns the OpenFlow message type code
    ///
    /// # Returns
    /// The Msg variant representing the message type
    fn msg_code(&self) -> Msg;

    /// Returns the message type code as a usize
    ///
    /// # Returns
    /// The numeric value of the message type
    fn msg_usize(&self) -> usize;

    /// Returns the size of the message payload
    ///
    /// # Returns
    /// The size of the message in bytes
    fn size_of(&self) -> usize;
}

/// Trait for working with OpenFlow message events
///
/// This trait provides functionality for creating and managing OpenFlow
/// message events, including header handling and event creation.
pub trait OfpMsgEvent {
    /// Creates a new OpenFlow header
    ///
    /// # Arguments
    /// * `message` - The message type code
    /// * `length` - The length of the message
    /// * `xid` - The transaction ID
    ///
    /// # Returns
    /// A new OfpHeader instance
    fn header(&self, message: u8, length: u16, xid: u32) -> OfpHeader;

    /// Parses an OpenFlow header from bytes
    ///
    /// # Arguments
    /// * `bytes` - The byte buffer containing the header data
    ///
    /// # Returns
    /// Result containing either the parsed OfpHeader or an error
    fn header_parse(&self, bytes: &Vec<u8>) -> Result<OfpHeader, Error>;

    /// Returns the OpenFlow version number
    ///
    /// # Returns
    /// The version number as a usize
    fn version(&self) -> usize;

    /// Returns the OpenFlow version number as a static value
    ///
    /// # Returns
    /// The version number as a usize
    fn ofp_version() -> usize;

    /// Returns the size of the OpenFlow header
    ///
    /// # Returns
    /// The header size in bytes
    fn header_size(&self) -> usize;

    /// Returns the message type code as a usize
    ///
    /// # Arguments
    /// * `msg` - The message type
    ///
    /// # Returns
    /// The numeric value of the message type
    fn msg_usize(&self, msg: Msg) -> usize;

    /// Parses a message type from a byte value
    ///
    /// # Arguments
    /// * `msg` - The byte value containing the message type
    ///
    /// # Returns
    /// The corresponding Msg variant
    fn msg_parse(&self, msg: u8) -> Msg;

    /// Creates a new hello event
    ///
    /// # Returns
    /// A new HelloEvent instance
    fn hello_event(&self) -> HelloEvent;

    /// Creates a new features request event
    ///
    /// # Returns
    /// A new FeaturesReqEvent instance
    fn fetures_req(&self) -> FeaturesReqEvent;

    /// Creates a new packet out event
    ///
    /// # Arguments
    /// * `port_id` - Optional output port ID
    /// * `payload` - The packet payload
    /// * `actions` - List of actions to apply
    ///
    /// # Returns
    /// A new PacketOutEvent instance
    fn packet_out(
        &self,
        port_id: Option<u16>,
        payload: Payload,
        actions: Vec<Action>,
    ) -> PacketOutEvent;
}
