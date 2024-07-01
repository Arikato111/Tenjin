use crate::openflow::ofp13::Action;

#[repr(u16)]
pub enum InstructType {
    GotoTable = 1,
    WriteMetadata = 2,
    WriteActions = 3,
    ApplyActions = 4,
    ClearActions = 5,
    Meter = 6,
    Experimenter = 0xffff,
}

impl From<InstructType> for u16 {
    fn from(value: InstructType) -> Self {
        value as u16
    }
}

pub struct GotoTable {
    typ: InstructType,
    len: u16,
    table_id: u8,
    pad: [u8; 3],
}

pub struct WriteMetadata {
    typ: InstructType,
    len: u16,
    pad: [u8; 4],
    metadata: u64,
    meta_mask: u64,
}

pub struct InstructActions {
    typ: InstructType,
    len: u16,
    pad: [u8; 4],
    action_header: Vec<Action>,
}
