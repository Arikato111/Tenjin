//! OpenFlow 1.0 Flow Modification Module
//! 
//! This module implements the flow modification functionality for OpenFlow 1.0.
//! Flow modifications are used to manage flow entries in the switch's flow tables,
//! including adding, modifying, and deleting flow entries.
//! 
//! The module is organized into several components:
//! - `flow_mod_handler`: Main flow modification event handling
//! - `command`: Flow modification command types
//! - `match_fields`: Packet matching criteria
//! - `flow_mod_flags`: Flow modification flags and options

pub mod flow_mod_handler;
pub use flow_mod_handler::FlowModEvent;

pub mod command;
pub use command::FlowModCommand;

pub mod match_fields;
pub use match_fields::{Mask, MatchFields};

pub mod flow_mod_flags;
pub use flow_mod_flags::FlowModFlags;