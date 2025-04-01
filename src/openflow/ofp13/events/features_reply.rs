//! OpenFlow v1.3 Features Reply Message Implementation
//!
//! This module implements the Features Reply message type used in OpenFlow v1.3 protocol.
//! The Features Reply message is sent by the switch in response to a Features Request,
//! providing information about its capabilities and configuration.

use std::io::{BufRead, Cursor, Error};

use byteorder::{BigEndian, ReadBytesExt};

/// Represents an OpenFlow v1.3 Features Reply message
///
/// Contains information about the switch's capabilities and configuration,
/// including datapath ID, buffer count, number of tables, and various capabilities.
pub struct FeaturesReplyEvent {
    /// Unique identifier for the datapath (switch)
    pub datapath_id: u64,
    /// Number of buffers supported by the switch
    pub n_buffers: u32,
    /// Number of flow tables supported by the switch
    pub n_tables: u8,
    /// Number of auxiliary connections supported
    pub auxiliary: u8,
    // pad 16 bit
    /// Bitmap of switch capabilities
    pub capabilities: Capabilities,
    /// Reserved field for future use
    pub reserved: u32,
}

impl FeaturesReplyEvent {
    /// Parses a Features Reply message from a byte vector
    ///
    /// # Arguments
    /// * `bytes` - The byte vector containing the message data
    ///
    /// # Returns
    /// * `Result<Self, Error>` - The parsed FeaturesReplyEvent or an error if parsing fails
    pub fn parse(bytes: &Vec<u8>) -> Result<Self, Error> {
        let mut bytes = Cursor::new(bytes);
        let datapath_id = bytes.read_u64::<BigEndian>()?;
        let n_buffers = bytes.read_u32::<BigEndian>()?;
        let n_tables = bytes.read_u8()?;
        let auxiliary = bytes.read_u8()?;
        bytes.consume(2);
        let capabilities: Capabilities = bytes.read_u32::<BigEndian>()?.into();
        let reserved = bytes.read_u32::<BigEndian>()?;
        Ok(Self {
            datapath_id,
            n_buffers,
            n_tables,
            auxiliary,
            capabilities,
            reserved,
        })
    }
}

/// Represents the capabilities of an OpenFlow switch
///
/// Contains boolean flags indicating which features and statistics
/// are supported by the switch.
pub struct Capabilities {
    /// Support for flow statistics
    pub flow_stats: bool,
    /// Support for table statistics
    pub table_stats: bool,
    /// Support for port statistics
    pub port_stats: bool,
    /// Support for group statistics
    pub group_stats: bool,
    /// Support for IP reassembly
    pub ip_reasm: bool,
    /// Support for queue statistics
    pub queue_stats: bool,
    /// Support for port blocking
    pub port_blocked: bool,
}

/// Converts a 32-bit integer into a Capabilities struct
///
/// # Arguments
/// * `value` - The 32-bit integer containing the capability flags
///
/// # Returns
/// A new Capabilities struct with flags set based on the input value
impl From<u32> for Capabilities {
    fn from(value: u32) -> Self {
        Self {
            flow_stats: value & 1 == 1,
            table_stats: value >> 1 & 1 == 1,
            port_stats: value >> 2 & 1 == 1,
            group_stats: value >> 3 & 1 == 1,
            ip_reasm: value >> 5 & 1 == 1,
            queue_stats: value >> 6 & 1 == 1,
            port_blocked: value >> 8 & 1 == 1,
        }
    }
}

/// Converts a Capabilities struct into a 32-bit integer
///
/// # Arguments
/// * `value` - The Capabilities struct to convert
///
/// # Returns
/// A 32-bit integer containing the capability flags
impl From<Capabilities> for u32 {
    fn from(value: Capabilities) -> Self {
        (value.flow_stats as u32)
            | ((value.table_stats as u32) << 1)
            | (value.port_stats as u32) << 2
            | (value.group_stats as u32) << 3
            | (value.ip_reasm as u32) << 5
            | (value.queue_stats as u32) << 6
            | (value.port_blocked as u32) << 8
    }
}
