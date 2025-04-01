//! OpenFlow 1.0 Flow Modification Commands
//! 
//! This module implements the flow modification commands used in OpenFlow 1.0
//! for managing flow entries in the switch's flow tables.
//! 
//! The module provides:
//! - Flow modification command enumeration
//! - Command parsing and conversion
//! - Command type validation

/// Represents the different types of flow modification commands
/// 
/// Each variant corresponds to a specific operation that can be performed
/// on flow entries in the switch's flow tables.
#[derive(Debug)]
pub enum FlowModCommand {
    /// Add a new flow entry
    Add = 0,
    /// Modify existing flow entries
    Modify = 1,
    /// Modify flow entries with strict matching
    ModifyStrict = 2,
    /// Delete flow entries
    Delete = 3,
    /// Delete flow entries with strict matching
    DeleteStrict = 4,
    /// Command that could not be parsed
    Unparsable = -1,
}

impl FlowModCommand {
    /// Converts the command to its numeric value
    /// 
    /// # Returns
    /// The command value as a usize
    pub fn to_number(&self) -> usize {
        match self {
            FlowModCommand::Add => Self::Add as usize,
            FlowModCommand::Modify => Self::Modify as usize,
            FlowModCommand::ModifyStrict => Self::ModifyStrict as usize,
            FlowModCommand::Delete => Self::Delete as usize,
            FlowModCommand::DeleteStrict => Self::DeleteStrict as usize,
            FlowModCommand::Unparsable => Self::Unparsable as usize,
        }
    }

    /// Parses a command from a byte value
    /// 
    /// # Arguments
    /// * `byte` - The byte value containing the command code
    /// 
    /// # Returns
    /// The corresponding FlowModCommand variant
    pub fn parse(byte: u16) -> Self {
        match byte {
            0 => Self::Add,
            1 => Self::Modify,
            2 => Self::ModifyStrict,
            3 => Self::Delete,
            4 => Self::DeleteStrict,
            _ => Self::Unparsable,
        }
    }
}
