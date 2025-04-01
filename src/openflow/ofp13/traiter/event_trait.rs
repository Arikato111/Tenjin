//! OpenFlow v1.3 Event Traits
//! 
//! This module defines the traits for handling OpenFlow events and message marshaling.
//! These traits provide functionality for working with OpenFlow messages, including
//! parsing, marshaling, and creating various types of events.

use std::io::Error;

use crate::openflow::ofp13::{
    events::{Action, FeaturesReqEvent, HelloEvent, PacketOutEvent, Payload},
    ofp_header::OfpHeader,
    Msg,
};

/// Trait for marshaling OpenFlow messages into wire format
/// 
/// This trait provides methods for converting OpenFlow messages into their
/// wire format representation, including message type information and size.
pub trait MessageMarshal {
    /// Marshals the message into a byte buffer
    /// 
    /// # Arguments
    /// * `bytes` - The buffer to write the message to
    fn marshal(&self, bytes: &mut Vec<u8>);

    /// Returns the message type
    /// 
    /// # Returns
    /// * `Msg` - The message type
    fn msg_code(&self) -> Msg;

    /// Returns the message type as a usize
    /// 
    /// # Returns
    /// * `usize` - The message type value
    fn msg_usize(&self) -> usize;

    /// Returns the size of the message in bytes
    /// 
    /// # Returns
    /// * `usize` - The message size in bytes
    fn size_of(&self) -> usize;
}

/// Trait for working with OpenFlow message events
/// 
/// This trait provides methods for creating and parsing OpenFlow message events,
/// including header handling, version information, and various event types.
pub trait OfpMsgEvent {
    /// Creates a new OpenFlow header
    /// 
    /// # Arguments
    /// * `message` - The message type code
    /// * `length` - The message length
    /// * `xid` - The transaction ID
    /// 
    /// # Returns
    /// * `OfpHeader` - The new header instance
    fn header(&self, message: u8, length: u16, xid: u32) -> OfpHeader;

    /// Parses an OpenFlow header from a byte buffer
    /// 
    /// # Arguments
    /// * `bytes` - The buffer containing the header data
    /// 
    /// # Returns
    /// * `Result<OfpHeader, Error>` - The parsed header or an error
    fn header_parse(&self, bytes: &Vec<u8>) -> Result<OfpHeader, Error>;

    /// Returns the OpenFlow protocol version
    /// 
    /// # Returns
    /// * `usize` - The protocol version number
    fn version(&self) -> usize;

    /// Returns the OpenFlow protocol version as a static value
    /// 
    /// # Returns
    /// * `usize` - The protocol version number
    fn ofp_version() -> usize;

    /// Returns the size of the header in bytes
    /// 
    /// # Returns
    /// * `usize` - The header size in bytes
    fn header_size(&self) -> usize;

    /// Returns the message type as a usize
    /// 
    /// # Arguments
    /// * `msg` - The message type
    /// 
    /// # Returns
    /// * `usize` - The message type value
    fn msg_usize(&self, msg: Msg) -> usize;

    /// Parses a message type from a byte value
    /// 
    /// # Arguments
    /// * `msg` - The message type byte
    /// 
    /// # Returns
    /// * `Msg` - The parsed message type
    fn msg_parse(&self, msg: u8) -> Msg;

    /// Creates a new hello event
    /// 
    /// # Returns
    /// * `HelloEvent` - The new hello event
    fn hello_event(&self) -> HelloEvent;

    /// Creates a new features request event
    /// 
    /// # Returns
    /// * `FeaturesReqEvent` - The new features request event
    fn fetures_req(&self) -> FeaturesReqEvent;

    /// Creates a new packet out event
    /// 
    /// # Arguments
    /// * `port_id` - Optional output port ID
    /// * `payload` - The packet payload
    /// * `actions` - List of actions to apply
    /// 
    /// # Returns
    /// * `PacketOutEvent` - The new packet out event
    fn packet_out(
        &self,
        port_id: Option<u32>,
        payload: Payload,
        actions: Vec<Action>,
    ) -> PacketOutEvent;
}
