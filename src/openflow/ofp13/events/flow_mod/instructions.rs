use byteorder::{BigEndian, WriteBytesExt};

use crate::openflow::ofp13::Action;

pub trait InstructTrait {
    fn marshal(&self, bytes: &mut Vec<u8>);
}

#[derive(Clone)]
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

impl InstructType {
    pub fn marshal(&self, bytes: &mut Vec<u8>) {
        let _ = bytes.write_u16::<BigEndian>(self.clone().into());
    }
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
}

impl GotoTable {
    pub fn new(table_id: u8) -> Self {
        Self {
            typ: InstructType::GotoTable,
            len: 8,
            table_id,
        }
    }
}

impl InstructTrait for GotoTable {
    fn marshal(&self, bytes: &mut Vec<u8>) {
        self.typ.marshal(bytes);
        let _ = bytes.write_u16::<BigEndian>(self.len);
        let _ = bytes.write_u8(self.table_id);
        // padding
        let _ = bytes.write_u16::<BigEndian>(0);
        let _ = bytes.write_u8(0);
    }
}

pub struct WriteMetadata {
    typ: InstructType,
    len: u16,
    metadata: u64,
    meta_mask: u64,
}

impl WriteMetadata {
    pub fn new(metadata: u64, meta_mask: u64) -> Self {
        Self {
            typ: InstructType::WriteMetadata,
            len: 24,
            metadata,
            meta_mask,
        }
    }
}

impl InstructTrait for WriteMetadata {
    fn marshal(&self, bytes: &mut Vec<u8>) {
        self.typ.marshal(bytes);
        let _ = bytes.write_u16::<BigEndian>(self.len);
        // padding
        let _ = bytes.write_u32::<BigEndian>(0);
        // *******
        let _ = bytes.write_u64::<BigEndian>(self.metadata);
        let _ = bytes.write_u64::<BigEndian>(self.meta_mask);
    }
}

pub struct InstructActions {
    typ: InstructType,
    len: u16,
    pub actions: Vec<Action>,
}

impl InstructActions {
    pub const WRITE: InstructType = InstructType::WriteActions;
    pub const APPLY: InstructType = InstructType::ApplyActions;
    pub const CLEAR: InstructType = InstructType::ClearActions;
    pub fn new(typ: InstructType) -> Self {
        Self {
            typ,
            len: 8,
            actions: Vec::new(),
        }
    }
}

impl InstructTrait for InstructActions {
    fn marshal(&self, bytes: &mut Vec<u8>) {
        let mut builder = Vec::new();
        for act in self.actions.iter() {
            let _ = act.marshal(&mut builder);
        }
        self.typ.marshal(bytes);
        let _ = bytes.write_u16::<BigEndian>(self.len + (builder.len() as u16));
        // padding
        let _ = bytes.write_u32::<BigEndian>(0);
        bytes.append(&mut builder);
    }
}

pub struct InstructMeter {
    typ: InstructType,
    len: u16,
    meter_id: u32,
}

impl InstructMeter {
    pub fn new(meter_id: u32) -> Self {
        Self {
            typ: InstructType::Meter,
            len: 8,
            meter_id,
        }
    }
}

impl InstructTrait for InstructMeter {
    fn marshal(&self, bytes: &mut Vec<u8>) {
        self.typ.marshal(bytes);
        let _ = bytes.write_u16::<BigEndian>(self.len);
        let _ = bytes.write_u32::<BigEndian>(self.meter_id);
    }
}

pub enum Instrucion {
    GotoTable(GotoTable),
    WriteMetadata(WriteMetadata),
    InstructActions(InstructActions),
    InstructMeter(InstructMeter),
}

impl Instrucion {
    pub fn marshal(&self, bytes: &mut Vec<u8>) {
        match &self {
            Instrucion::GotoTable(v) => v.marshal(bytes),
            Instrucion::WriteMetadata(v) => v.marshal(bytes),
            Instrucion::InstructActions(v) => v.marshal(bytes),
            Instrucion::InstructMeter(v) => v.marshal(bytes),
        }
    }
}
