use std::mem::transmute;

#[repr(u16)]
#[derive(Debug)]
pub enum ErrorType {
    HelloFailed(HelloFailed) = 0,                /* Hello protocol failed. */
    BadRequest(BadRequest) = 1,                  /* Request was not understood. */
    BadAction(BadAction) = 2,                    /* Error in action description. */
    BadInstruction(BadInstruction) = 3,          /* Error in instruction list. */
    BadMatch(BadMatch) = 4,                      /* Error in match. */
    FlowModFailed(FlowModFailed) = 5,            /* Problem modifying flow entry. */
    GroupModFailed(GroupModFailed) = 6,          /* Problem modifying group entry. */
    PortModFailed(PortModFailed) = 7,            /* Port mod request failed. */
    TableModFailed(TableModFailed) = 8,          /* Table mod request failed. */
    QueueOpFailed(QueueOpFailed) = 9,            /* Queue operation failed. */
    SwitchConfigFailed(SwitchConfigFailed) = 10, /* Switch config request failed. */
    RoleRequestFailed(RoleRequestFailed) = 11,   /* Controller Role request failed. */
    MeterModFailed(MeterModFailed) = 12,         /* Error in meter. */
    TableFeaturesFailed(TableFeaturesFailed) = 13, /* Setting table features failed. */
    EXPERIMENTER = 0xffff,                       /* Experimenter error messages. */
}

impl ErrorType {
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
#[derive(Debug)]
pub enum HelloFailed {
    Incompatible,
    EPerm,
}

impl HelloFailed {
    pub fn new(error_code: u16) -> Self {
        match error_code {
            0 => Self::Incompatible,
            _ => Self::EPerm,
        }
    }
}

#[repr(u16)]
#[derive(Debug)]
pub enum BadRequest {
    BadVersion = 0,               /* ofp_header.version not supported. */
    BadType = 1,                  /* ofp_header.type not supported. */
    BadMultipart = 2,             /* ofp_multipart_request.type not supported. */
    BadExperimenter = 3, /* Experimenter id not supported in ofp_experimenter_header or ofp_multipart_request or ofp_multipart_reply). */
    BadExpType = 4,      /* Experimenter type not supported. */
    EPERM = 5,           /* Permissions error. */
    BadLen = 6,          /* Wrong request length for type. */
    BufferEmpty = 7,     /* Specified buffer has already been used. */
    BufferUnknown = 8,   /* Specified buffer does not exist. */
    BadTableId = 9,      /* Specified table-id invalid or does not exist. */
    IsSlave = 10,        /* Denied because controller is slave. */
    BadPort = 11,        /* Invalid port. */
    BadPacket = 12,      /* Invalid packet in packet-out. */
    MultipartBufferOverflow = 13, /* ofp_multipart_request overflowed the assigned buffer. */
}

impl BadRequest {
    pub fn new(error_code: u16) -> Self {
        if error_code < 14 {
            unsafe { transmute::<u16, BadRequest>(error_code) }
        } else {
            BadRequest::BadType
        }
    }
}

#[repr(u16)]
#[derive(Debug)]
pub enum BadAction {
    BadType = 0,            /* Unknown action type. */
    BadLen = 1,             /* Length problem in actions. */
    BadExperimenter = 2,    /* Unknown experimenter id specified. */
    BadExpType = 3,         /* Unknown action for experimenter id. */
    BadOutPort = 4,         /* Problem validating output port. */
    BadArgument = 5,        /* Bad action argument. */
    EPERM = 6,              /* Permissions error. */
    TooMany = 7,            /* Can’t handle this many actions. */
    BadQueue = 8,           /* Problem validating output queue. */
    BadOutGroup = 9,        /* Invalid group id in forward action. */
    MatchInconsistent = 10, /* Action can’t apply for this match, or Set-Field missing prerequisite. */
    UnsupportedOrder = 11, /* Action order is unsupported for the action list in an Apply-Actions instruction */
    BadTag = 12,           /* Actions uses an unsupported tag/encap. */
    BadSetType = 13,       /* Unsupported type in SET_FIELD action. */
    BadSetLen = 14,        /* Length problem in SET_FIELD action. */
    BadSetArgument = 15,   /* Bad argument in SET_FIELD action. */
}

impl Default for BadAction {
    fn default() -> Self {
        Self::EPERM
    }
}

impl BadAction {
    pub fn new(error_code: u16) -> Self {
        if error_code < 16 {
            unsafe { transmute::<u16, BadAction>(error_code) }
        } else {
            BadAction::BadType
        }
    }
}

#[repr(u16)]
#[derive(Debug)]
pub enum BadInstruction {
    UnknownInst = 0,       /* Unknown instruction. */
    UnsupInst = 1,         /* Switch or table does not support the instruction. */
    BadTableId = 2,        /* Invalid Table-ID specified. */
    UnsupMetadata = 3,     /* Metadata value unsupported by datapath. */
    UnsupMetadataMask = 4, /* Metadata mask value unsupported by datapath. */
    BadExperimenter = 5,   /* Unknown experimenter id specified. */
    BadExpType = 6,        /* Unknown instruction for experimenter id. */
    BadLen = 7,            /* Length problem in instructions. */
    EPERM = 8,             /* Permissions error. */
}

impl BadInstruction {
    pub fn new(error_code: u16) -> Self {
        if error_code < 9 {
            unsafe { transmute::<u16, BadInstruction>(error_code) }
        } else {
            BadInstruction::EPERM
        }
    }
}

#[repr(u16)]
#[derive(Debug)]
pub enum BadMatch {
    BadType = 0,       /* Unsupported match type specified by the match */
    BadLen = 1,        /* Length problem in match. */
    BadTag = 2,        /* Match uses an unsupported tag/encap. */
    BadDlAddrMask = 3, /* Unsupported datalink addr mask - switch does not support arbitrary datalink address mask. */
    BadNwAddrMask = 4, /* Unsupported network addr mask - switch does not support arbitrary network address mask. */
    BadWildcards = 5,  /* Unsupported combination of fields masked or omitted in the match. */
    BadField = 6,      /* Unsupported field type in the match. */
    BadValue = 7,      /* Unsupported value in a match field. */
    BadMask = 8, /* Unsupported mask specified in the match, field is not dl-address or nw-address. */
    BadPrereq = 9, /* A prerequisite was not met. */
    DupField = 10, /* A field type was duplicated. */
    EPERM = 11,  /* Permissions error. */
}

impl BadMatch {
    pub fn new(error_code: u16) -> Self {
        if error_code < 12 {
            unsafe { transmute(error_code) }
        } else {
            BadMatch::EPERM
        }
    }
}

#[repr(u16)]
#[derive(Debug)]
pub enum FlowModFailed {
    UNKNOWN = 0,    /* Unspecified error. */
    TableFull = 1,  /* Flow not added because table was full. */
    BadTableId = 2, /* Table does not exist */
    OVERLAP = 3,    /* Attempted to add overlapping flow with CHECK_OVERLAP flag set. */
    EPERM = 4,      /* Permissions error. */
    BadTimeout = 5, /* Flow not added because of unsupported idle/hard timeout. */
    BadCommand = 6, /* Unsupported or unknown command. */
    BadFlags = 7,   /* Unsupported or unknown flags. */
}

impl FlowModFailed {
    pub fn new(error_code: u16) -> Self {
        if error_code < 8 {
            unsafe { transmute(error_code) }
        } else {
            FlowModFailed::EPERM
        }
    }
}

#[repr(u16)]
#[derive(Debug)]
pub enum GroupModFailed {
    GroupExists = 0, /* Group not added because a group ADD attempted to replace an already-present group. */
    InvalidGroup = 1, /* Group not added because Group specified is invalid. */
    WeightUnsupported = 2, /* Switch does not support unequal load sharing with select groups. */
    OutOfGroups = 3, /* The group table is full. */
    OutOfBuckets = 4, /* The maximum number of action buckets for a group has been exceeded. */
    ChainingUnsupported = 5, /* Switch does not support groups that forward to groups. */
    WatchUnsupported = 6, /* This group cannot watch the watch_port or watch_group specified. */
    Loop = 7,        /* Group entry would cause a loop. */
    UnknownGroup = 8, /* Group not modified because a group MODIFY attempted to modify a non-existent group. */
    ChainedGroup = 9, /* Group not deleted because another group is forwarding to it. */
    BadType = 10,     /* Unsupported or unknown group type. */
    BadCommand = 11,  /* Unsupported or unknown command. */
    BadBucket = 12,   /* Error in bucket. */
    BadWatch = 13,    /* Error in watch port/group. */
    EPERM = 14,       /* Permissions error. */
}

impl GroupModFailed {
    pub fn new(error_code: u16) -> Self {
        if error_code < 15 {
            unsafe { transmute(error_code) }
        } else {
            Self::EPERM
        }
    }
}

#[repr(u16)]
#[derive(Debug)]
pub enum PortModFailed {
    BadPort = 0,      /* Specified port number does not exist. */
    BadHwAddr = 1,    /* Specified hardware address does not * match the port number. */
    BadConfig = 2,    /* Specified config is invalid. */
    BadAdvertise = 3, /* Specified advertise is invalid. */
    EPERM = 4,        /* Permissions error. */
}

impl PortModFailed {
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

#[repr(u16)]
#[derive(Debug)]
pub enum TableModFailed {
    BadTable = 0,
    BadConfig = 1,
    EPERM = 2,
}

impl TableModFailed {
    pub fn new(error_code: u16) -> Self {
        match error_code {
            0 => Self::BadTable,
            1 => Self::BadConfig,
            _ => Self::EPERM,
        }
    }
}

#[derive(Debug)]
pub enum QueueOpFailed {
    BadPort,
    BadQueue,
    EPerm,
}

impl QueueOpFailed {
    pub fn new(error_code: u16) -> Self {
        match error_code {
            0 => Self::BadPort,
            1 => Self::BadQueue,
            _ => Self::EPerm,
        }
    }
}

#[repr(u16)]
#[derive(Debug)]
pub enum SwitchConfigFailed {
    BadFlags = 0,
    BadLen = 1,
    EPERM = 2,
}

impl SwitchConfigFailed {
    pub fn new(error_code: u16) -> Self {
        match error_code {
            0 => Self::BadFlags,
            1 => Self::BadLen,
            _ => Self::EPERM,
        }
    }
}

#[repr(u16)]
#[derive(Debug)]
pub enum RoleRequestFailed {
    Stale = 0,
    Unsup = 1,
    BadRole = 2,
}

impl RoleRequestFailed {
    pub fn new(error_code: u16) -> Self {
        match error_code {
            0 => Self::Stale,
            1 => Self::Unsup,
            _ => Self::BadRole,
        }
    }
}

#[repr(u16)]
#[derive(Debug)]
pub enum MeterModFailed {
    Unknown = 0,      /* Unspecified error. */
    MeterExists = 1, /* Meter not added because a Meter ADD * attempted to replace an existing Meter. */
    InvalidMeter = 2, /* Meter not added because Meter specified * is invalid. */
    UnknownMeter = 3, /* Meter not modified because a Meter MODIFY attempted to modify a non-existent Meter. */
    BadCommand = 4,   /* Unsupported or unknown command. */
    BadFlags = 5,     /* Flag configuration unsupported. */
    BadRate = 6,      /* Rate unsupported. */
    BadBurst = 7,     /* Burst size unsupported. */
    BadBand = 8,      /* Band unsupported. */
    BadBandValue = 9, /* Band value unsupported. */
    OutOfMeters = 10, /* No more meters available. */
    OutOfBands = 11,  /* The maximum number of properties * for a meter has been exceeded. */
}

impl MeterModFailed {
    pub fn new(error_code: u16) -> Self {
        if error_code < 12 {
            unsafe { transmute(error_code) }
        } else {
            Self::OutOfBands
        }
    }
}

#[repr(u16)]
#[derive(Debug)]
pub enum TableFeaturesFailed {
    BadTable = 0,    /* Specified table does not exist. */
    BadMetadata = 1, /* Invalid metadata mask. */
    BadType = 2,     /* Unknown property type. */
    BadLen = 3,      /* Length problem in properties. */
    BadArgument = 4, /* Unsupported property value. */
    EPERM = 5,       /* Permissions error. */
}

impl TableFeaturesFailed {
    pub fn new(error_code: u16) -> Self {
        if error_code < 6 {
            unsafe { transmute(error_code) }
        } else {
            Self::EPERM
        }
    }
}

#[derive(Debug)]
pub struct ErrorExperimenter {
    typ: u16,
    exp_typ: u16,
    experimenter: u32,
}
