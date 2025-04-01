//! OpenFlow 1.0 Payload
//!
//! This module implements the payload handling for OpenFlow 1.0 messages.
//! The payload can be either buffered on the switch or sent directly to the controller.
//!
//! The module provides:
//! - Payload type enumeration
//! - Payload length calculation
//! - Payload serialization

use std::io::Write;

/// Represents the payload of an OpenFlow message
///
/// The payload can be either buffered on the switch (with a buffer ID)
/// or sent directly to the controller without buffering.
#[derive(Debug)]
pub enum Payload {
    /// Payload buffered on the switch with buffer ID
    Buffered(u32, Vec<u8>),
    /// Payload sent directly without buffering
    NoBuffered(Vec<u8>),
}

impl Payload {
    /// Returns the length of the payload in bytes
    ///
    /// # Returns
    /// The length of the payload data
    pub fn length(&self) -> usize {
        match self {
            Payload::Buffered(_, p) | Payload::NoBuffered(p) => p.len(),
        }
    }

    /// Serializes the payload into a byte buffer
    ///
    /// # Arguments
    /// * `bytes` - Mutable reference to the byte buffer to write to
    pub fn marshal(&self, bytes: &mut Vec<u8>) {
        match self {
            Payload::Buffered(_, buf) | Payload::NoBuffered(buf) => {
                let _ = bytes.write_all(&buf);
            }
        }
    }
}
