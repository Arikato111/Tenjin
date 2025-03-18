//! Small tools is here. `utils` is the module includes functions, traits, struct and others
//! that works with some features or library.
// private
pub mod net;
pub use net::MacAddr;
pub mod value_converter;

// public
pub mod log;
pub use value_converter::*;
