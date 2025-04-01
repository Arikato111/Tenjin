// Core OpenFlow protocol implementation module
pub mod openflow;

// Utility functions and helper modules
pub mod utils;

// Example module demonstrating library usage
#[cfg(feature = "example")]
pub mod example;

// Command-line interface module for the library
#[cfg(feature = "cli")]
pub mod cli;
