//! OpenFlow 1.0 Events
//! 
//! This module implements the event handling system for OpenFlow 1.0.
//! Events represent various messages and notifications exchanged between
//! the controller and switch.
//! 
//! The module provides implementations for:
//! - Error handling and reporting
//! - Packet-in and packet-out events
//! - Flow table modifications
//! - Action handling
//! - Connection management (hello, echo)
//! - Switch feature negotiation
//! - Payload handling

pub mod error;
pub use error::ErrorEvent;

pub mod packet_in;
pub use packet_in::{PacketInEvent, PacketInReason};

pub mod packet_out;
pub use packet_out::PacketOutEvent;

pub mod flow_mod;
pub use flow_mod::{FlowModEvent, MatchFields};

pub mod actions;
pub use actions::Action;

pub mod hello;
pub use hello::HelloEvent;

pub mod features_req;
pub use features_req::FeaturesReqEvent;

pub mod payload;
pub use payload::Payload;

pub mod echo_request;
pub use echo_request::EchoRequestEvent;

pub mod echo_reply;
pub use echo_reply::EchoReplyEvent;