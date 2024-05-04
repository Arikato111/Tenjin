pub enum FlowModCommand {
    Add,
    Modify,
    ModifyStrict,
    Delete,
    DeleteStrict,
    Unparsable,
}

impl FlowModCommand {
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
