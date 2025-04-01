//! OpenFlow 1.0 Flow Modification Flags
//!
//! This module implements the flags used in OpenFlow 1.0 flow modification messages
//! to control various aspects of flow entry management.
//!
//! The module provides:
//! - Flow modification flag structure
//! - Flag parsing and serialization
//! - Flag combination handling

use byteorder::{BigEndian, WriteBytesExt};

/// Represents the flags that can be set in a flow modification message
///
/// These flags control various aspects of how flow entries are managed
/// and how the switch should handle flow modifications.
pub struct FlowModFlags {
    /// Send flow removed message when flow entry is removed
    pub send_flow_rem: bool,
    /// Check for overlapping entries when adding/modifying flows
    pub check_overlap: bool,
    /// Treat this as an emergency flow entry
    pub emerg: bool,
}

impl FlowModFlags {
    /// Creates a new FlowModFlags instance with specified values
    ///
    /// # Arguments
    /// * `send_flow_rem` - Whether to send flow removed messages
    /// * `check_overlap` - Whether to check for overlapping entries
    /// * `emerg` - Whether this is an emergency flow entry
    ///
    /// # Returns
    /// A new FlowModFlags instance
    pub fn new(send_flow_rem: bool, check_overlap: bool, emerg: bool) -> Self {
        Self {
            send_flow_rem,
            check_overlap,
            emerg,
        }
    }

    /// Creates a FlowModFlags instance with all flags set to false
    ///
    /// # Returns
    /// A new FlowModFlags instance with all flags disabled
    pub fn all_false() -> Self {
        Self {
            check_overlap: false,
            emerg: false,
            send_flow_rem: false,
        }
    }

    /// Parses flags from a byte value
    ///
    /// # Arguments
    /// * `byte` - The byte value containing the flag bits
    ///
    /// # Returns
    /// A new FlowModFlags instance with parsed flag values
    pub fn parse(byte: u16) -> Self {
        let send_flow_rem = byte >> 0 & 1 != 0;
        let check_overlap = byte >> 1 & 1 != 0;
        let emerg = byte >> 2 & 1 != 0;
        Self {
            send_flow_rem,
            check_overlap,
            emerg,
        }
    }

    /// Serializes flags to a byte buffer
    ///
    /// # Arguments
    /// * `bytes` - Mutable reference to the byte buffer to write to
    pub fn marshal(&self, bytes: &mut Vec<u8>) {
        let mut value = 0u16;
        if self.send_flow_rem {
            value |= 1 << 0;
        }
        if self.check_overlap {
            value |= 1 << 1;
        }
        if self.emerg {
            value |= 1 << 2;
        }
        let _ = bytes.write_u16::<BigEndian>(value);
    }
}
