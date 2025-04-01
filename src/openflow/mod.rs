//! OpenFlow protocol implementation module
//!
//! This module provides implementations for different versions of the OpenFlow protocol.
//! Currently supported versions:
//! - OpenFlow 1.0 (ofp10)
//! - OpenFlow 1.3 (ofp13)
//!
//! Each version is implemented in its own submodule with specific message types,
//! event handling, and protocol-specific functionality.

pub mod ofp10;

pub mod ofp13;
