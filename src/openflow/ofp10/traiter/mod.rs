pub mod header_trait;
pub use header_trait::OpenflowHeader;

pub mod event_trait;
pub use event_trait::{MessageMarshal, OfpMsgEvent};
