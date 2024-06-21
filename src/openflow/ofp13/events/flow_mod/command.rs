pub enum FlowModCommand {
    Add = 0,
    Modify = 1,
    ModifyStrict = 2,
    Delete = 3,
    DeleteStrict = 4,
    Unparsable = -1,
}

impl FlowModCommand {
    pub fn to_number(&self) -> usize {
        match self {
            FlowModCommand::Add => Self::Add as usize,
            FlowModCommand::Modify => Self::Modify as usize,
            FlowModCommand::ModifyStrict => Self::ModifyStrict as usize,
            FlowModCommand::Delete => Self::Delete as usize,
            FlowModCommand::DeleteStrict => Self::DeleteStrict as usize,
            FlowModCommand::Unparsable => Self::Unparsable as usize,
        }
    }
    pub fn parse(byte: u16) -> Self {
        match byte {
            0 => Self::Add,
            1 => Self::Modify,
            2 => Self::ModifyStrict,
            3 => Self::Delete,
            4 => Self::DeleteStrict,
            _ => Self::Unparsable,
        }
    }
}
