//! OpenFlow v1.3 Flow Modification Flags
//! 
//! This module defines the flags that can be set when modifying flow entries
//! in the OpenFlow switch's flow tables.

use byteorder::{BigEndian, WriteBytesExt};

/// Flags for flow modification operations
pub struct FlowModFlags {
    /// Send flow removed message when flow expires or is deleted
    pub send_flow_rem: bool,
    /// Check for overlapping entries first
    pub check_overlap: bool,
    /// Reset flow packet and byte counts
    pub reset_counts: bool,
    /// Don't send packet-in messages for the first packet of a flow
    pub no_pkt_counts: bool,
    /// Don't send packet-in messages for the first packet of a flow
    pub no_byt_counts: bool,
}

impl FlowModFlags {
    /// Creates a new set of flow modification flags
    /// 
    /// # Arguments
    /// * `send_flow_rem` - Whether to send flow removed message
    /// * `check_overlap` - Whether to check for overlapping entries
    /// * `reset_counts` - Whether to reset flow counts
    /// * `no_pkt_counts` - Whether to disable packet-in messages
    /// * `no_byt_counts` - Whether to disable byte-in messages
    /// 
    /// # Returns
    /// * `FlowModFlags` - The new flags instance
    pub fn new(
        send_flow_rem: bool,
        check_overlap: bool,
        reset_counts: bool,
        no_pkt_counts: bool,
        no_byt_counts: bool,
    ) -> Self {
        Self {
            send_flow_rem,
            check_overlap,
            reset_counts,
            no_pkt_counts,
            no_byt_counts,
        }
    }

    /// Creates a new set of flow modification flags with all flags set to false
    /// 
    /// # Returns
    /// * `FlowModFlags` - The new flags instance with all flags disabled
    pub fn all_false() -> Self {
        Self {
            send_flow_rem: false,
            check_overlap: false,
            reset_counts: false,
            no_pkt_counts: false,
            no_byt_counts: false,
        }
    }

    /// Parses flow modification flags from a byte value
    /// 
    /// # Arguments
    /// * `byte` - The byte value containing the flags
    /// 
    /// # Returns
    /// * `FlowModFlags` - The parsed flags instance
    pub fn parse(byte: u16) -> Self {
        let send_flow_rem = byte >> 0 & 1 != 0;
        let check_overlap = byte >> 1 & 1 != 0;
        let reset_counts = byte >> 2 & 1 != 0;
        let no_pkt_counts = byte >> 3 & 1 != 0;
        let no_byt_counts = byte >> 4 & 1 == 1;

        Self {
            send_flow_rem,
            check_overlap,
            reset_counts,
            no_pkt_counts,
            no_byt_counts,
        }
    }

    /// Marshals the flags into a byte buffer
    /// 
    /// # Arguments
    /// * `bytes` - The buffer to write the flags to
    pub fn marshal(&self, bytes: &mut Vec<u8>) {
        let mut value = 0u16;
        if self.send_flow_rem {
            value |= 1 << 0;
        }
        if self.check_overlap {
            value |= 1 << 1;
        }
        if self.reset_counts {
            value |= 1 << 2;
        }
        if self.no_pkt_counts {
            value |= 1 << 3;
        }
        if self.no_byt_counts {
            value |= 1 << 4;
        }
        let _ = bytes.write_u16::<BigEndian>(value);
    }
}
