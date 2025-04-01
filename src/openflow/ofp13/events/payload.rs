//! OpenFlow v1.3 Payload Implementation
//! 
//! This module implements the payload types used in OpenFlow v1.3 protocol messages.
//! It supports both buffered and non-buffered payloads for different message types.

use std::io::Write;

/// Represents a payload in an OpenFlow v1.3 message
/// 
/// The payload can be either buffered (with a buffer ID) or non-buffered.
/// This is used for various message types that need to carry packet data.
pub enum Payload {
    /// Buffered payload with a buffer ID and data
    /// 
    /// # Fields
    /// * `u32` - The buffer ID assigned by the switch
    /// * `Vec<u8>` - The actual payload data
    Buffered(u32, Vec<u8>),
    
    /// Non-buffered payload containing only data
    /// 
    /// # Fields
    /// * `Vec<u8>` - The actual payload data
    NoBuffered(Vec<u8>),
}

impl Payload {
    /// Returns the length of the payload data in bytes
    /// 
    /// # Returns
    /// The length of the payload data vector
    pub fn length(&self) -> usize {
        match self {
            Payload::Buffered(_, p) | Payload::NoBuffered(p) => p.len(),
        }
    }

    /// Marshals the payload data into a byte vector
    /// 
    /// # Arguments
    /// * `bytes` - The target byte vector to write the payload data to
    pub fn marshal(&self, bytes: &mut Vec<u8>) {
        match self {
            Payload::Buffered(_, buf) | Payload::NoBuffered(buf) => {
                let _ = bytes.write_all(buf);
            }
        }
    }
}
