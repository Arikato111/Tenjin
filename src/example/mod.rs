//! Example implementations of OpenFlow controllers
//! 
//! This module provides example implementations of OpenFlow controllers for different versions
//! of the OpenFlow protocol. Each controller implements packet forwarding and flow management
//! functionality specific to its OpenFlow version.

/// OpenFlow 1.3 Controller module
/// 
/// Implements a controller compatible with OpenFlow 1.3 switches, providing advanced
/// features like multiple tables and enhanced match fields.
pub mod ctrl13;
pub use ctrl13::Controller13;

/// OpenFlow 1.0 Controller module
/// 
/// Implements a controller compatible with OpenFlow 1.0 switches, providing basic
/// packet forwarding and flow management capabilities.
pub mod ctrl10;
pub use ctrl10::Controller10;