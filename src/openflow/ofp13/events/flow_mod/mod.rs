//! OpenFlow v1.3 Flow Modification Implementation
//!
//! This module implements the flow modification functionality for OpenFlow v1.3 protocol.
//! It provides functionality for adding, modifying, and deleting flow entries in the
//! OpenFlow switch's flow tables.

/// Flow modification handler implementation
pub mod flow_mod_handler;
pub use flow_mod_handler::FlowModEvent;

/// Flow modification command definitions
pub mod command;
pub use command::FlowModCommand;

/// Match fields and match type definitions
pub mod match_fields;
pub use match_fields::{MatchFields, MatchType, OfpMatch};

/// Flow modification flags definitions
pub mod flow_mod_flags;
pub use flow_mod_flags::FlowModFlags;

/// Flow modification instruction definitions
pub mod instructions;
