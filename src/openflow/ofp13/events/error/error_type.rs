//! OpenFlow v1.3 Error Type Definitions
//!
//! This module defines the various error types and codes that can be returned
//! by an OpenFlow switch to indicate different types of failures or errors.

use std::mem::transmute;

/// Main error type enum that categorizes different types of OpenFlow errors
#[repr(u16)]
#[derive(Debug)]
pub enum ErrorType {
    /// Hello protocol failed
    HelloFailed(HelloFailed) = 0,
    /// Request was not understood
    BadRequest(BadRequest) = 1,
    /// Error in action description
    BadAction(BadAction) = 2,
    /// Error in instruction list
    BadInstruction(BadInstruction) = 3,
    /// Error in match
    BadMatch(BadMatch) = 4,
    /// Problem modifying flow entry
    FlowModFailed(FlowModFailed) = 5,
    /// Problem modifying group entry
    GroupModFailed(GroupModFailed) = 6,
    /// Port mod request failed
    PortModFailed(PortModFailed) = 7,
    /// Table mod request failed
    TableModFailed(TableModFailed) = 8,
    /// Queue operation failed
    QueueOpFailed(QueueOpFailed) = 9,
    /// Switch config request failed
    SwitchConfigFailed(SwitchConfigFailed) = 10,
    /// Controller Role request failed
    RoleRequestFailed(RoleRequestFailed) = 11,
    /// Error in meter
    MeterModFailed(MeterModFailed) = 12,
    /// Setting table features failed
    TableFeaturesFailed(TableFeaturesFailed) = 13,
    /// Experimenter error messages
    EXPERIMENTER = 0xffff,
}

impl ErrorType {
    /// Creates a new error type from the raw error type and code values
    pub fn new(error_type: u16, error_code: u16) -> ErrorType {
        match error_type {
            0 => Self::HelloFailed(HelloFailed::new(error_code)),
            1 => Self::BadRequest(BadRequest::new(error_code)),
            2 => Self::BadAction(BadAction::new(error_code)),
            3 => Self::BadInstruction(BadInstruction::new(error_code)),
            4 => Self::BadMatch(BadMatch::new(error_code)),
            5 => Self::FlowModFailed(FlowModFailed::new(error_code)),
            6 => Self::GroupModFailed(GroupModFailed::new(error_code)),
            7 => Self::PortModFailed(PortModFailed::new(error_code)),
            8 => Self::TableModFailed(TableModFailed::new(error_code)),
            9 => Self::QueueOpFailed(QueueOpFailed::new(error_code)),
            10 => Self::SwitchConfigFailed(SwitchConfigFailed::new(error_code)),
            11 => Self::RoleRequestFailed(RoleRequestFailed::new(error_code)),
            12 => Self::MeterModFailed(MeterModFailed::new(error_code)),
            13 => Self::TableFeaturesFailed(TableFeaturesFailed::new(error_code)),
            _ => Self::EXPERIMENTER,
        }
    }
}

/// Specific error codes for Hello protocol failures
#[derive(Debug)]
pub enum HelloFailed {
    /// Incompatible version
    Incompatible,
    /// Permission error
    EPerm,
}

impl HelloFailed {
    /// Creates a new HelloFailed error from the error code
    pub fn new(error_code: u16) -> Self {
        match error_code {
            0 => Self::Incompatible,
            _ => Self::EPerm,
        }
    }
}

/// Specific error codes for bad request errors
#[repr(u16)]
#[derive(Debug)]
pub enum BadRequest {
    /// ofp_header.version not supported
    BadVersion = 0,
    /// ofp_header.type not supported
    BadType = 1,
    /// ofp_multipart_request.type not supported
    BadMultipart = 2,
    /// Experimenter id not supported
    BadExperimenter = 3,
    /// Experimenter type not supported
    BadExpType = 4,
    /// Permissions error
    EPERM = 5,
    /// Wrong request length for type
    BadLen = 6,
    /// Specified buffer has already been used
    BufferEmpty = 7,
    /// Specified buffer does not exist
    BufferUnknown = 8,
    /// Specified table-id invalid or does not exist
    BadTableId = 9,
    /// Denied because controller is slave
    IsSlave = 10,
    /// Invalid port
    BadPort = 11,
    /// Invalid packet in packet-out
    BadPacket = 12,
    /// ofp_multipart_request overflowed the assigned buffer
    MultipartBufferOverflow = 13,
}

impl BadRequest {
    /// Creates a new BadRequest error from the error code
    pub fn new(error_code: u16) -> Self {
        if error_code < 14 {
            unsafe { transmute::<u16, BadRequest>(error_code) }
        } else {
            BadRequest::BadType
        }
    }
}

/// Specific error codes for bad action errors
#[repr(u16)]
#[derive(Debug)]
pub enum BadAction {
    /// Unknown action type
    BadType = 0,
    /// Length problem in actions
    BadLen = 1,
    /// Unknown experimenter id specified
    BadExperimenter = 2,
    /// Unknown action for experimenter id
    BadExpType = 3,
    /// Problem validating output port
    BadOutPort = 4,
    /// Bad action argument
    BadArgument = 5,
    /// Permissions error
    EPERM = 6,
    /// Can't handle this many actions
    TooMany = 7,
    /// Problem validating output queue
    BadQueue = 8,
    /// Invalid group id in forward action
    BadOutGroup = 9,
    /// Action can't apply for this match, or Set-Field missing prerequisite
    MatchInconsistent = 10,
    /// Action order is unsupported for the action list in an Apply-Actions instruction
    UnsupportedOrder = 11,
    /// Actions uses an unsupported tag/encap
    BadTag = 12,
    /// Unsupported type in SET_FIELD action
    BadSetType = 13,
    /// Length problem in SET_FIELD action
    BadSetLen = 14,
    /// Bad argument in SET_FIELD action
    BadSetArgument = 15,
}

impl Default for BadAction {
    fn default() -> Self {
        BadAction::BadType
    }
}

impl BadAction {
    /// Creates a new BadAction error from the error code
    pub fn new(error_code: u16) -> Self {
        if error_code < 16 {
            unsafe { transmute::<u16, BadAction>(error_code) }
        } else {
            BadAction::BadType
        }
    }
}

/// Specific error codes for bad instruction errors
#[repr(u16)]
#[derive(Debug)]
pub enum BadInstruction {
    /// Unknown instruction
    UnknownInst = 0,
    /// Switch or table does not support the instruction
    UnsupInst = 1,
    /// Invalid Table-ID specified
    BadTableId = 2,
    /// Metadata value unsupported by datapath
    UnsupMetadata = 3,
    /// Metadata mask value unsupported by datapath
    UnsupMetadataMask = 4,
    /// Unknown experimenter id specified
    BadExperimenter = 5,
    /// Unknown instruction for experimenter id
    BadExpType = 6,
    /// Length problem in instructions
    BadLen = 7,
    /// Permissions error
    EPERM = 8,
}

impl BadInstruction {
    /// Creates a new BadInstruction error from the error code
    pub fn new(error_code: u16) -> Self {
        if error_code < 9 {
            unsafe { transmute::<u16, BadInstruction>(error_code) }
        } else {
            BadInstruction::UnknownInst
        }
    }
}

/// Specific error codes for bad match errors
#[repr(u16)]
#[derive(Debug)]
pub enum BadMatch {
    /// Unsupported match type specified by the match
    BadType = 0,
    /// Length problem in match
    BadLen = 1,
    /// Match uses an unsupported tag/encap
    BadTag = 2,
    /// Unsupported datalink addr mask
    BadDlAddrMask = 3,
    /// Unsupported network addr mask
    BadNwAddrMask = 4,
    /// Unsupported combination of fields masked or omitted in the match
    BadWildcards = 5,
    /// Unsupported field type in the match
    BadField = 6,
    /// Unsupported value in a match field
    BadValue = 7,
    /// Unsupported mask specified in the match
    BadMask = 8,
    /// A prerequisite was not met
    BadPrereq = 9,
    /// A field type was duplicated
    DupField = 10,
    /// Permissions error
    EPERM = 11,
}

impl BadMatch {
    /// Creates a new BadMatch error from the error code
    pub fn new(error_code: u16) -> Self {
        if error_code < 12 {
            unsafe { transmute::<u16, BadMatch>(error_code) }
        } else {
            BadMatch::BadType
        }
    }
}

/// Specific error codes for flow modification failures
#[repr(u16)]
#[derive(Debug)]
pub enum FlowModFailed {
    /// Unspecified error
    UNKNOWN = 0,
    /// Flow not added because table was full
    TableFull = 1,
    /// Table does not exist
    BadTableId = 2,
    /// Attempted to add overlapping flow with CHECK_OVERLAP flag set
    OVERLAP = 3,
    /// Permissions error
    EPERM = 4,
    /// Flow not added because of unsupported idle/hard timeout
    BadTimeout = 5,
    /// Unsupported or unknown command
    BadCommand = 6,
    /// Unsupported or unknown flags
    BadFlags = 7,
}

impl FlowModFailed {
    /// Creates a new FlowModFailed error from the error code
    pub fn new(error_code: u16) -> Self {
        if error_code < 8 {
            unsafe { transmute::<u16, FlowModFailed>(error_code) }
        } else {
            FlowModFailed::UNKNOWN
        }
    }
}

/// Specific error codes for group modification failures
#[repr(u16)]
#[derive(Debug)]
pub enum GroupModFailed {
    /// Group not added because a group ADD attempted to replace an already-present group
    GroupExists = 0,
    /// Group not added because Group specified is invalid
    InvalidGroup = 1,
    /// Switch does not support unequal load sharing with select groups
    WeightUnsupported = 2,
    /// The group table is full
    OutOfGroups = 3,
    /// The maximum number of action buckets for a group has been exceeded
    OutOfBuckets = 4,
    /// Switch does not support groups that forward to groups
    ChainingUnsupported = 5,
    /// This group cannot watch the watch_port or watch_group specified
    WatchUnsupported = 6,
    /// Group entry would cause a loop
    Loop = 7,
    /// Group not modified because a group MODIFY attempted to modify a non-existent group
    UnknownGroup = 8,
    /// Group not deleted because another group is forwarding to it
    ChainedGroup = 9,
    /// Unsupported or unknown group type
    BadType = 10,
    /// Unsupported or unknown command
    BadCommand = 11,
    /// Error in bucket
    BadBucket = 12,
    /// Error in watch port/group
    BadWatch = 13,
    /// Permissions error
    EPERM = 14,
}

impl GroupModFailed {
    /// Creates a new GroupModFailed error from the error code
    pub fn new(error_code: u16) -> Self {
        if error_code < 15 {
            unsafe { transmute::<u16, GroupModFailed>(error_code) }
        } else {
            GroupModFailed::GroupExists
        }
    }
}

/// Specific error codes for port modification failures
#[repr(u16)]
#[derive(Debug)]
pub enum PortModFailed {
    /// Specified port number does not exist
    BadPort = 0,
    /// Specified hardware address does not match the port number
    BadHwAddr = 1,
    /// Specified config is invalid
    BadConfig = 2,
    /// Specified advertise is invalid
    BadAdvertise = 3,
    /// Permissions error
    EPERM = 4,
}

impl PortModFailed {
    /// Creates a new PortModFailed error from the error code
    pub fn new(error_code: u16) -> Self {
        match error_code {
            0 => Self::BadPort,
            1 => Self::BadHwAddr,
            2 => Self::BadConfig,
            3 => Self::BadAdvertise,
            _ => Self::EPERM,
        }
    }
}

/// Specific error codes for table modification failures
#[repr(u16)]
#[derive(Debug)]
pub enum TableModFailed {
    /// Bad table
    BadTable = 0,
    /// Bad configuration
    BadConfig = 1,
    /// Permissions error
    EPERM = 2,
}

impl TableModFailed {
    /// Creates a new TableModFailed error from the error code
    pub fn new(error_code: u16) -> Self {
        match error_code {
            0 => Self::BadTable,
            1 => Self::BadConfig,
            _ => Self::EPERM,
        }
    }
}

/// Specific error codes for queue operation failures
#[derive(Debug)]
pub enum QueueOpFailed {
    /// Bad port
    BadPort,
    /// Bad queue
    BadQueue,
    /// Permissions error
    EPerm,
}

impl QueueOpFailed {
    /// Creates a new QueueOpFailed error from the error code
    pub fn new(error_code: u16) -> Self {
        match error_code {
            0 => Self::BadPort,
            1 => Self::BadQueue,
            _ => Self::EPerm,
        }
    }
}

/// Specific error codes for switch configuration failures
#[repr(u16)]
#[derive(Debug)]
pub enum SwitchConfigFailed {
    /// Bad flags
    BadFlags = 0,
    /// Bad length
    BadLen = 1,
    /// Permissions error
    EPERM = 2,
}

impl SwitchConfigFailed {
    /// Creates a new SwitchConfigFailed error from the error code
    pub fn new(error_code: u16) -> Self {
        match error_code {
            0 => Self::BadFlags,
            1 => Self::BadLen,
            _ => Self::EPERM,
        }
    }
}

/// Specific error codes for role request failures
#[repr(u16)]
#[derive(Debug)]
pub enum RoleRequestFailed {
    /// Stale message
    Stale = 0,
    /// Unsupported role
    Unsup = 1,
    /// Bad role
    BadRole = 2,
}

impl RoleRequestFailed {
    /// Creates a new RoleRequestFailed error from the error code
    pub fn new(error_code: u16) -> Self {
        match error_code {
            0 => Self::Stale,
            1 => Self::Unsup,
            _ => Self::BadRole,
        }
    }
}

/// Specific error codes for meter modification failures
#[repr(u16)]
#[derive(Debug)]
pub enum MeterModFailed {
    /// Unspecified error
    Unknown = 0,
    /// Meter not added because a Meter ADD attempted to replace an existing Meter
    MeterExists = 1,
    /// Meter not added because Meter specified is invalid
    InvalidMeter = 2,
    /// Meter not modified because a Meter MODIFY attempted to modify a non-existent Meter
    UnknownMeter = 3,
    /// Unsupported or unknown command
    BadCommand = 4,
    /// Flag configuration unsupported
    BadFlags = 5,
    /// Rate unsupported
    BadRate = 6,
    /// Burst size unsupported
    BadBurst = 7,
    /// Band unsupported
    BadBand = 8,
    /// Band value unsupported
    BadBandValue = 9,
    /// No more meters available
    OutOfMeters = 10,
    /// The maximum number of properties for a meter has been exceeded
    OutOfBands = 11,
}

impl MeterModFailed {
    /// Creates a new MeterModFailed error from the error code
    pub fn new(error_code: u16) -> Self {
        if error_code < 12 {
            unsafe { transmute::<u16, MeterModFailed>(error_code) }
        } else {
            MeterModFailed::Unknown
        }
    }
}

/// Specific error codes for table features failures
#[repr(u16)]
#[derive(Debug)]
pub enum TableFeaturesFailed {
    /// Specified table does not exist
    BadTable = 0,
    /// Invalid metadata mask
    BadMetadata = 1,
    /// Unknown property type
    BadType = 2,
    /// Length problem in properties
    BadLen = 3,
    /// Unsupported property value
    BadArgument = 4,
    /// Permissions error
    EPERM = 5,
}

impl TableFeaturesFailed {
    /// Creates a new TableFeaturesFailed error from the error code
    pub fn new(error_code: u16) -> Self {
        if error_code < 6 {
            unsafe { transmute::<u16, TableFeaturesFailed>(error_code) }
        } else {
            TableFeaturesFailed::BadTable
        }
    }
}
