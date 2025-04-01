//! OpenFlow v1.3 Flow Modification Commands
//!
//! This module defines the different commands that can be used to modify
//! flow entries in the OpenFlow switch's flow tables.

/// Commands for modifying flow entries in the OpenFlow switch
#[repr(u8)]
pub enum FlowModCommand {
    /// Add a new flow entry
    Add = 0,
    /// Modify all matching flow entries
    Modify = 1,
    /// Modify flow entries with exactly matching fields
    ModifyStrict = 2,
    /// Delete all matching flow entries
    Delete = 3,
    /// Delete flow entries with exactly matching fields
    DeleteStrict = 4,
    /// Command could not be parsed
    Unparsable = 0xff,
}

impl FlowModCommand {
    /// Converts the command to its numeric representation
    ///
    /// # Returns
    /// * `usize` - The numeric value of the command
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

    /// Parses a command from its numeric representation
    ///
    /// # Arguments
    /// * `byte` - The numeric value to parse
    ///
    /// # Returns
    /// * `FlowModCommand` - The parsed command or Unparsable if invalid
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
