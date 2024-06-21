#[derive(Debug)]
pub enum ErrorType {
    HelloFailed(HelloFailed),
    BadRequest(BadRequest),
    BadAction(BadAction),
    FlowModFailed(FlowModFailed),
    PortModFailed(PortModFailed),
    QueueOpFailed(QueueOpFailed),
}

impl ErrorType {
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

#[derive(Debug)]
pub enum BadRequest {
    BadVersion,
    BadType,
    BadStat,
    BadVendor,
    BadSubType,
    EPerm,
    BadLen,
    BufferEmpty,
    BufferUnkown,
}

impl BadRequest {
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

#[derive(Debug)]
pub enum BadAction {
    BadType,
    BadLen,
    BadVendor,
    BadVendotType,
    BadOutPort,
    BadArgument,
    EPerm,
    TooMany,
    BadQueue,
}

impl BadAction {
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

#[derive(Debug)]
pub enum FlowModFailed {
    AllTablesFull,
    Overlap,
    EPerm,
    BadEmergTimeout,
    BadCommand,
    Unsupported,
}

impl FlowModFailed {
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

#[derive(Debug)]
pub enum PortModFailed {
    BadPort,
    BadHwAddr,
}

impl PortModFailed {
    pub fn new(error_code: u16) -> Self {
        match error_code {
            0 => Self::BadPort,
            _ => Self::BadHwAddr,
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
