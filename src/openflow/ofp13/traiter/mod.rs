//! OpenFlow v1.3 Traits
//!
//! This module defines the core traits used throughout the OpenFlow v1.3 implementation.
//! These traits provide common functionality for message handling, event processing,
//! and header management.

/// Header trait implementation
pub mod header_trait;
pub use header_trait::OpenflowHeader;

/// Event trait implementation
pub mod event_trait;
pub use event_trait::{MessageMarshal, OfpMsgEvent};
