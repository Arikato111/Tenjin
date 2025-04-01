//! OpenFlow v1.3 Protocol Events Module
//!
//! This module contains all the event types and structures used in OpenFlow v1.3 protocol communication.
//! It includes message types for flow modification, packet handling, features negotiation,
//! and various control messages.

/// Error message handling module
pub mod error;
pub use error::ErrorEvent;

/// Packet-in message handling module
pub mod packet_in;
pub use packet_in::{PacketInEvent, PacketInReason};

/// Packet-out message handling module
pub mod packet_out;
pub use packet_out::PacketOutEvent;

/// Flow modification message handling module
pub mod flow_mod;
pub use flow_mod::{FlowModCommand, FlowModEvent, FlowModFlags, MatchFields};

/// Action definitions for flow entries
pub mod actions;
pub use actions::Action;

/// Hello message handling module
pub mod hello;
pub use hello::HelloEvent;

/// Features request message handling module
pub mod features_req;
pub use features_req::FeaturesReqEvent;

/// Features reply message handling module
pub mod features_reply;
pub use features_reply::FeaturesReplyEvent;

/// Generic payload handling module
pub mod payload;
pub use payload::Payload;

/// Echo request message handling module
pub mod echo_request;
pub use echo_request::EchoRequestEvent;

/// Echo reply message handling module
pub mod echo_reply;
pub use echo_reply::EchoReplyEvent;
