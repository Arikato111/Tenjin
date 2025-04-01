//! OpenFlow v1.3 Flow Modification Instructions
//!
//! This module defines the instructions that can be applied to packets
//! matching a flow entry in the OpenFlow switch's flow tables.

use byteorder::{BigEndian, WriteBytesExt};

use crate::openflow::ofp13::Action;

/// Trait for marshaling instructions into wire format
pub trait InstructTrait {
    /// Marshals the instruction into a byte buffer
    ///
    /// # Arguments
    /// * `bytes` - The buffer to write the instruction to
    fn marshal(&self, bytes: &mut Vec<u8>);
}

/// Types of instructions that can be applied to matching packets
#[derive(Clone)]
#[repr(u16)]
pub enum InstructType {
    /// Jump to another table
    GotoTable = 1,
    /// Write metadata to the packet
    WriteMetadata = 2,
    /// Write actions to the packet
    WriteActions = 3,
    /// Apply actions immediately
    ApplyActions = 4,
    /// Clear all actions
    ClearActions = 5,
    /// Apply meter
    Meter = 6,
    /// Experimenter instruction
    Experimenter = 0xffff,
}

impl InstructType {
    /// Marshals the instruction type into a byte buffer
    ///
    /// # Arguments
    /// * `bytes` - The buffer to write the type to
    pub fn marshal(&self, bytes: &mut Vec<u8>) {
        let _ = bytes.write_u16::<BigEndian>(self.clone().into());
    }
}

impl From<InstructType> for u16 {
    fn from(value: InstructType) -> Self {
        value as u16
    }
}

/// Instruction to jump to another table
pub struct GotoTable {
    /// Type of instruction
    typ: InstructType,
    /// Length of instruction in bytes
    len: u16,
    /// ID of the table to jump to
    table_id: u8,
}

impl GotoTable {
    /// Creates a new goto table instruction
    ///
    /// # Arguments
    /// * `table_id` - ID of the table to jump to
    ///
    /// # Returns
    /// * `GotoTable` - The new instruction instance
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

/// Instruction to write metadata to the packet
pub struct WriteMetadata {
    /// Type of instruction
    typ: InstructType,
    /// Length of instruction in bytes
    len: u16,
    /// Metadata value to write
    metadata: u64,
    /// Metadata mask
    meta_mask: u64,
}

impl WriteMetadata {
    /// Creates a new write metadata instruction
    ///
    /// # Arguments
    /// * `metadata` - Metadata value to write
    /// * `meta_mask` - Metadata mask
    ///
    /// # Returns
    /// * `WriteMetadata` - The new instruction instance
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

/// Instruction to apply actions to the packet
pub struct InstructActions {
    /// Type of instruction
    typ: InstructType,
    /// Length of instruction in bytes
    len: u16,
    /// List of actions to apply
    pub actions: Vec<Action>,
}

impl InstructActions {
    /// Write actions instruction type
    pub const WRITE: InstructType = InstructType::WriteActions;
    /// Apply actions instruction type
    pub const APPLY: InstructType = InstructType::ApplyActions;
    /// Clear actions instruction type
    pub const CLEAR: InstructType = InstructType::ClearActions;

    /// Creates a new actions instruction
    ///
    /// # Arguments
    /// * `typ` - Type of actions instruction
    ///
    /// # Returns
    /// * `InstructActions` - The new instruction instance
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

/// Instruction to apply a meter to the packet
pub struct InstructMeter {
    /// Type of instruction
    typ: InstructType,
    /// Length of instruction in bytes
    len: u16,
    /// ID of the meter to apply
    meter_id: u32,
}

impl InstructMeter {
    /// Creates a new meter instruction
    ///
    /// # Arguments
    /// * `meter_id` - ID of the meter to apply
    ///
    /// # Returns
    /// * `InstructMeter` - The new instruction instance
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

/// Enum of all possible flow modification instructions
pub enum Instrucion {
    /// Jump to another table
    GotoTable(GotoTable),
    /// Write metadata to the packet
    WriteMetadata(WriteMetadata),
    /// Apply actions to the packet
    InstructActions(InstructActions),
    /// Apply a meter to the packet
    InstructMeter(InstructMeter),
}

impl Instrucion {
    /// Marshals the instruction into a byte buffer
    ///
    /// # Arguments
    /// * `bytes` - The buffer to write the instruction to
    pub fn marshal(&self, bytes: &mut Vec<u8>) {
        match &self {
            Instrucion::GotoTable(v) => v.marshal(bytes),
            Instrucion::WriteMetadata(v) => v.marshal(bytes),
            Instrucion::InstructActions(v) => v.marshal(bytes),
            Instrucion::InstructMeter(v) => v.marshal(bytes),
        }
    }
}
