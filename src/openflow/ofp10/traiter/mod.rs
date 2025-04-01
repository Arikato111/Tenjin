//! OpenFlow 1.0 Traits Module
//!
//! This module provides the core traits used throughout the OpenFlow 1.0 implementation.
//! These traits define the interfaces for handling messages, events, and headers in a
//! type-safe and consistent manner.
//!
//! The module is organized into two main components:
//! - `header_trait`: Defines the interface for OpenFlow message headers
//! - `event_trait`: Defines the interfaces for message marshaling and event handling

pub mod header_trait;
pub use header_trait::OpenflowHeader;

pub mod event_trait;
pub use event_trait::{MessageMarshal, OfpMsgEvent};
