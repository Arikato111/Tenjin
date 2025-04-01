//! OpenFlow 1.0 error type definitions and implementations.
//! This module provides enums and implementations for various OpenFlow error types
//! that can occur during controller-switch communication.

/// Main error type enum that categorizes different types of OpenFlow errors.
/// Each variant contains a specific error type with its own set of error codes.
#[derive(Debug)]
pub enum ErrorType {
    /// Errors that occur during the hello phase of OpenFlow connection
    HelloFailed(HelloFailed),
    /// Errors related to malformed or invalid requests
    BadRequest(BadRequest),
    /// Errors related to invalid or unsupported actions
    BadAction(BadAction),
    /// Errors that occur during flow table modifications
    FlowModFailed(FlowModFailed),
    /// Errors that occur during port modifications
    PortModFailed(PortModFailed),
    /// Errors that occur during queue operations
    QueueOpFailed(QueueOpFailed),
}

impl ErrorType {
    /// Creates a new ErrorType based on the error type and code received from the switch.
    /// 
    /// # Arguments
    /// * `error_type` - The main error type identifier
    /// * `error_code` - The specific error code within that error type
    /// 
    /// # Returns
    /// A new ErrorType instance with the appropriate variant and error details
    pub fn new(error_type: u16, error_code: u16) -> ErrorType {
        match error_type {
            0 => ErrorType::HelloFailed(HelloFailed::new(error_code)),
            1 => ErrorType::BadRequest(BadRequest::new(error_code)),
            2 => ErrorType::BadAction(BadAction::new(error_code)),
            3 => ErrorType::FlowModFailed(FlowModFailed::new(error_code)),
            4 => ErrorType::PortModFailed(PortModFailed::new(error_code)),
            5 => ErrorType::QueueOpFailed(QueueOpFailed::new(error_code)),
            _ => panic!("bad error_type in error {}", error_type),
        }
    }
}

/// Specific errors that can occur during the hello phase of OpenFlow connection
#[derive(Debug)]
pub enum HelloFailed {
    /// Protocol version incompatibility between controller and switch
    Incompatible,
    /// Permission denied during hello phase
    EPerm,
}

impl HelloFailed {
    /// Creates a new HelloFailed error based on the error code
    pub fn new(error_code: u16) -> Self {
        match error_code {
            0 => Self::Incompatible,
            _ => Self::EPerm,
        }
    }
}

/// Errors related to malformed or invalid requests from the controller
#[derive(Debug)]
pub enum BadRequest {
    /// Unsupported OpenFlow version
    BadVersion,
    /// Unsupported message type
    BadType,
    /// Unsupported statistics request
    BadStat,
    /// Unsupported vendor
    BadVendor,
    /// Unsupported vendor subtype
    BadSubType,
    /// Permission denied
    EPerm,
    /// Invalid message length
    BadLen,
    /// Buffer is empty
    BufferEmpty,
    /// Unknown buffer
    BufferUnkown,
}

impl BadRequest {
    /// Creates a new BadRequest error based on the error code
    pub fn new(error_code: u16) -> Self {
        match error_code {
            0 => Self::BadVersion,
            1 => Self::BadType,
            2 => Self::BadStat,
            3 => Self::BadVendor,
            4 => Self::BadSubType,
            5 => Self::EPerm,
            6 => Self::BadLen,
            7 => Self::BufferEmpty,
            8 => Self::BufferUnkown,
            _ => Self::BadVersion,
        }
    }
}

/// Errors related to invalid or unsupported actions in flow entries
#[derive(Debug)]
pub enum BadAction {
    /// Unsupported action type
    BadType,
    /// Invalid action length
    BadLen,
    /// Unsupported vendor
    BadVendor,
    /// Unsupported vendor type
    BadVendotType,
    /// Invalid output port
    BadOutPort,
    /// Invalid action argument
    BadArgument,
    /// Permission denied
    EPerm,
    /// Too many actions
    TooMany,
    /// Invalid queue
    BadQueue,
}

impl BadAction {
    /// Creates a new BadAction error based on the error code
    pub fn new(error_code: u16) -> Self {
        match error_code {
            0 => Self::BadType,
            1 => Self::BadLen,
            2 => Self::BadVendor,
            3 => Self::BadVendotType,
            4 => Self::BadOutPort,
            5 => Self::BadArgument,
            6 => Self::EPerm,
            7 => Self::TooMany,
            _ => Self::BadQueue,
        }
    }
}

/// Errors that occur during flow table modifications
#[derive(Debug)]
pub enum FlowModFailed {
    /// All flow tables are full
    AllTablesFull,
    /// Flow entry overlaps with existing entry
    Overlap,
    /// Permission denied
    EPerm,
    /// Invalid emergency flow timeout
    BadEmergTimeout,
    /// Invalid flow modification command
    BadCommand,
    /// Unsupported flow modification
    Unsupported,
}

impl FlowModFailed {
    /// Creates a new FlowModFailed error based on the error code
    pub fn new(error_code: u16) -> Self {
        match error_code {
            0 => Self::AllTablesFull,
            1 => Self::Overlap,
            2 => Self::EPerm,
            3 => Self::BadEmergTimeout,
            4 => Self::BadCommand,
            _ => Self::Unsupported,
        }
    }
}

/// Errors that occur during port modifications
#[derive(Debug)]
pub enum PortModFailed {
    /// Invalid port
    BadPort,
    /// Invalid hardware address
    BadHwAddr,
}

impl PortModFailed {
    /// Creates a new PortModFailed error based on the error code
    pub fn new(error_code: u16) -> Self {
        match error_code {
            0 => Self::BadPort,
            _ => Self::BadHwAddr,
        }
    }
}

/// Errors that occur during queue operations
#[derive(Debug)]
pub enum QueueOpFailed {
    /// Invalid port
    BadPort,
    /// Invalid queue
    BadQueue,
    /// Permission denied
    EPerm,
}

impl QueueOpFailed {
    /// Creates a new QueueOpFailed error based on the error code
    pub fn new(error_code: u16) -> Self {
        match error_code {
            0 => Self::BadPort,
            1 => Self::BadQueue,
            _ => Self::EPerm,
        }
    }
}
