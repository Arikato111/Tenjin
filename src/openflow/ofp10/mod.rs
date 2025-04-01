//! OpenFlow 1.0 Protocol Implementation
//! 
//! This module implements the OpenFlow 1.0 protocol specification, providing:
//! - Message types and structures
//! - Port definitions and handling
//! - Event processing for various OpenFlow messages
//! - Protocol header management
//! - Connection management and TCP handling
//! - Message marshaling and event handling traits

pub mod message;
pub use message::Msg;

pub mod ofp_port;
pub use ofp_port::PseudoPort;

pub mod events;
pub use events::{
    Action, EchoReplyEvent, EchoRequestEvent, ErrorEvent, FlowModEvent, HelloEvent, MatchFields,
    PacketInEvent, PacketOutEvent,
};

pub mod ofp_header;
pub use ofp_header::OfpHeader;

pub mod ofp_manager;
pub use ofp_manager::Openflow10;

pub mod controller_frame;
pub use controller_frame::ControllerFrame10;

pub mod tcp_listener;
pub use tcp_listener::tcp_listener_handler;

pub mod traiter;
pub use traiter::{MessageMarshal, OfpMsgEvent, OpenflowHeader};
