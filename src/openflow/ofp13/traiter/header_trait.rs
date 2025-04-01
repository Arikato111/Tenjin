//! OpenFlow v1.3 Header Trait
//!
//! This module defines the OpenFlow header trait that provides common functionality
//! for handling OpenFlow message headers, including version, message type, length,
//! and transaction ID.

use std::io::Error;

/// Trait for handling OpenFlow message headers
///
/// This trait provides methods for working with OpenFlow message headers,
/// including version information, message type, length, and transaction ID.
/// It also provides functionality for creating new headers, parsing existing ones,
/// and marshaling headers into wire format.
pub trait OpenflowHeader {
    /// Returns the OpenFlow protocol version
    ///
    /// # Returns
    /// * `usize` - The protocol version number
    fn version(&self) -> usize;

    /// Returns the message type
    ///
    /// # Returns
    /// * `u8` - The message type code
    fn message(&self) -> u8;

    /// Returns the total message length
    ///
    /// # Returns
    /// * `usize` - The length in bytes
    fn length(&self) -> usize;

    /// Returns the transaction ID
    ///
    /// # Returns
    /// * `u32` - The transaction ID
    fn xid(&self) -> u32;

    /// Returns the packet size
    ///
    /// # Returns
    /// * `usize` - The packet size in bytes
    fn pkt_size(&self) -> usize;

    /// Creates a new header instance
    ///
    /// # Arguments
    /// * `message` - The message type code
    /// * `length` - The total message length
    /// * `xid` - The transaction ID
    ///
    /// # Returns
    /// * `Self` - A new header instance
    fn new(message: u8, length: usize, xid: usize) -> Self;

    /// Returns the size of the header in bytes
    ///
    /// # Returns
    /// * `usize` - The header size in bytes
    fn header_size(&self) -> usize;

    /// Parses a header from a byte buffer
    ///
    /// # Arguments
    /// * `buf` - The buffer containing the header data
    ///
    /// # Returns
    /// * `Result<Self, Error>` - The parsed header or an error
    fn parse(buf: &Vec<u8>) -> Result<Self, Error>
    where
        Self: Sized;

    /// Marshals the header into a byte buffer
    ///
    /// # Arguments
    /// * `bytes` - The buffer to write the header to
    fn marshal(&self, bytes: &mut Vec<u8>);
}
