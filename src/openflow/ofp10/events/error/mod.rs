//! OpenFlow 1.0 error handling module.
//! This module provides functionality for handling various types of errors
//! that can occur during OpenFlow communication between controllers and switches.
//! 
//! The module is organized into two main components:
//! - `error_type`: Defines the different types of errors and their specific error codes
//! - `error_handler`: Provides functionality for parsing and handling error events

pub mod error_handler;
pub use error_handler::ErrorEvent;

pub mod error_type;