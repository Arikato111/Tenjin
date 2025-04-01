//! OpenFlow v1.3 Error Message Implementation
//!
//! This module implements the error message types and handling for OpenFlow v1.3 protocol.
//! It provides functionality for handling various error conditions that may occur
//! during OpenFlow communication between the controller and switch.

/// Error handler implementation module
pub mod error_handler;
pub use error_handler::ErrorEvent;

/// Error type definitions module
pub mod error_type;
