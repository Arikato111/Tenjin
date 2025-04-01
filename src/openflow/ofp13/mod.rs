//! OpenFlow 1.3 Protocol Implementation
//!
//! This module implements the OpenFlow 1.3 protocol specification, providing:
//! - Message types and structures
//! - Port definitions and handling
//! - Event processing for various OpenFlow messages
//! - Protocol header management
//! - Connection management and TCP handling
//! - Message marshaling and event handling traits
//!
//! OpenFlow 1.3 introduces several improvements over 1.0, including:
//! - Enhanced match fields and actions
//! - Improved flow table management
//! - Better support for multiple tables
//! - More sophisticated packet processing capabilities

pub mod message;
pub use message::Msg;

pub mod ofp_port;
pub use ofp_port::PseudoPort;

pub mod events;
pub use events::{
    Action, EchoReplyEvent, EchoRequestEvent, ErrorEvent, FeaturesReplyEvent, FlowModEvent,
    HelloEvent, MatchFields, PacketInEvent, PacketOutEvent,
};

pub mod ofp_header;
pub use ofp_header::OfpHeader;

pub mod ofp_manager;
pub use ofp_manager::Openflow13;

pub mod controller_frame;
pub use controller_frame::ControllerFrame13;

pub mod tcp_listener;
pub use tcp_listener::tcp_listener_handler;

pub mod traiter;
pub use traiter::{MessageMarshal, OfpMsgEvent, OpenflowHeader};
