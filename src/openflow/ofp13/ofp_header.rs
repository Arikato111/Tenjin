//! OpenFlow 1.3 Protocol Header
//! 
//! This module implements the OpenFlow 1.3 protocol header structure and its
//! serialization/deserialization functionality. The header is common to all
//! OpenFlow messages and contains essential metadata about the message.
//! 
//! OpenFlow 1.3 maintains the same header structure as 1.0 but uses version 4
//! to indicate the protocol version.

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::{
    io::{Cursor, Error},
    mem::size_of,
};

use crate::openflow::ofp13::OpenflowHeader;

use super::{OfpMsgEvent, Openflow13};

/// Represents the OpenFlow 1.3 protocol header
/// 
/// The header contains four fields that are common to all OpenFlow messages:
/// - version: Protocol version (4 for OpenFlow 1.3)
/// - message: Type of message
/// - length: Total length of the message including header
/// - xid: Transaction ID for matching requests and replies
#[derive(Debug)]
pub struct OfpHeader {
    /// Protocol version (4 for OpenFlow 1.3)
    pub version: u8,
    /// Message type identifier
    pub message: u8,
    /// Total message length including header
    pub length: u16,
    /// Transaction ID for request/reply matching
    pub xid: u32,
}

impl OpenflowHeader for OfpHeader {
    /// Creates a new OpenFlow header
    /// 
    /// # Arguments
    /// * `message` - The message type identifier
    /// * `length` - Length of the message payload
    /// * `xid` - Transaction ID
    fn new(message: u8, length: usize, xid: usize) -> Self {
        Self {
            version: Openflow13::ofp_version() as u8,
            message,
            length: (size_of::<OfpHeader>() + length) as u16,
            xid: xid as u32,
        }
    }

    /// Returns the protocol version (4 for OpenFlow 1.3)
    fn version(&self) -> usize {
        Openflow13::ofp_version()
    }

    /// Returns the message type identifier
    fn message(&self) -> u8 {
        self.message
    }

    /// Returns the total message length
    fn length(&self) -> usize {
        self.length as usize
    }

    /// Returns the transaction ID
    fn xid(&self) -> u32 {
        self.xid
    }

    /// Returns the size of the header in bytes
    fn header_size(&self) -> usize {
        size_of::<Self>()
    }

    /// Returns the size of the message payload in bytes
    fn pkt_size(&self) -> usize {
        self.length as usize - size_of::<Self>()
    }

    /// Parses a byte buffer into an OpenFlow header
    /// 
    /// # Arguments
    /// * `buf` - Byte buffer containing the header data
    /// 
    /// # Returns
    /// Result containing either the parsed header or an error
    fn parse(buf: &Vec<u8>) -> Result<Self, Error> {
        let mut buf_cursor = Cursor::new(buf);
        let version = buf_cursor.read_u8()?;
        let message = buf_cursor.read_u8()?;
        let length = buf_cursor.read_u16::<BigEndian>()?;
        let xid = buf_cursor.read_u32::<BigEndian>()?;
        Ok(Self {
            version,
            message,
            length,
            xid,
        })
    }

    /// Serializes the header into a byte buffer
    /// 
    /// # Arguments
    /// * `bytes` - Mutable reference to the byte buffer to write to
    fn marshal(&self, bytes: &mut Vec<u8>) {
        let _ = bytes.write_u8(self.version);
        let _ = bytes.write_u8(self.message);
        let _ = bytes.write_u16::<BigEndian>(self.length);
        let _ = bytes.write_u32::<BigEndian>(self.xid);
    }
}
