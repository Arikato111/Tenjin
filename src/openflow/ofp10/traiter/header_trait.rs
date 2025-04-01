//! OpenFlow 1.0 Header Trait
//! 
//! This module defines the trait for handling OpenFlow 1.0 message headers.
//! The header trait provides functionality for creating, parsing, and managing
//! OpenFlow message headers, which contain essential metadata about each message.
//! 
//! The module provides:
//! - Header creation and parsing
//! - Version and message type handling
//! - Length and transaction ID management
//! - Header serialization

use std::io::Error;

/// Trait for handling OpenFlow message headers
/// 
/// This trait provides the interface for working with OpenFlow message headers,
/// including creation, parsing, and serialization of header data.
pub trait OpenflowHeader {
    /// Returns the OpenFlow protocol version
    /// 
    /// # Returns
    /// The version number as a usize
    fn version(&self) -> usize;

    /// Returns the message type code
    /// 
    /// # Returns
    /// The message type as a u8
    fn message(&self) -> u8;

    /// Returns the total message length
    /// 
    /// # Returns
    /// The length in bytes as a usize
    fn length(&self) -> usize;

    /// Returns the transaction ID
    /// 
    /// # Returns
    /// The transaction ID as a u32
    fn xid(&self) -> u32;

    /// Returns the packet size
    /// 
    /// # Returns
    /// The packet size in bytes as a usize
    fn pkt_size(&self) -> usize;

    /// Creates a new header instance
    /// 
    /// # Arguments
    /// * `message` - The message type code
    /// * `length` - The total message length
    /// * `xid` - The transaction ID
    /// 
    /// # Returns
    /// A new header instance
    fn new(message: u8, length: usize, xid: usize) -> Self;

    /// Returns the size of the header
    /// 
    /// # Returns
    /// The header size in bytes as a usize
    fn header_size(&self) -> usize;

    /// Parses a header from a byte buffer
    /// 
    /// # Arguments
    /// * `buf` - The byte buffer containing the header data
    /// 
    /// # Returns
    /// Result containing either the parsed header or an error
    fn parse(buf: &Vec<u8>) -> Result<Self, Error>
    where
        Self: Sized;

    /// Serializes the header into a byte buffer
    /// 
    /// # Arguments
    /// * `bytes` - Mutable reference to the byte buffer to write to
    fn marshal(&self, bytes: &mut Vec<u8>);
}
