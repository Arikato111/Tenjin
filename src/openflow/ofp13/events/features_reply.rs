use std::io::{Cursor, Error};

use byteorder::{BigEndian, ReadBytesExt};

pub struct FeaturesReplyEvent {
    pub datapath_id: u64,
    pub n_buffers: u32,
    pub n_tables: u8,
    pub auxiliary: u8,
    // pad 16 bit
    pub capabilities: Capabilities,
    pub reserved: u32,
}

impl FeaturesReplyEvent {
    pub fn parse(bytes: &mut Vec<u8>) -> Result<Self, Error> {
        let mut bytes = Cursor::new(bytes);
        let datapath_id = bytes.read_u64::<BigEndian>()?;
        let n_buffers = bytes.read_u32::<BigEndian>()?;
        let n_tables = bytes.read_u8()?;
        let auxiliary = bytes.read_u8()?;
        let capabilities: Capabilities = bytes.read_u32::<BigEndian>()?.into();
        let reserved = bytes.read_u32::<BigEndian>()?;
        Ok(Self {
            datapath_id,
            n_buffers,
            n_tables,
            auxiliary,
            capabilities,
            reserved,
        })
    }
}

pub struct Capabilities {
    pub flow_stats: bool,
    pub table_stats: bool,
    pub port_stats: bool,
    pub group_stats: bool,
    pub ip_reasm: bool,
    pub queue_stats: bool,
    pub port_blocked: bool,
}

impl From<u32> for Capabilities {
    fn from(value: u32) -> Self {
        Self {
            flow_stats: value & 1 == 1,
            table_stats: value >> 1 & 1 == 1,
            port_stats: value >> 2 & 1 == 1,
            group_stats: value >> 3 & 1 == 1,
            ip_reasm: value >> 5 & 1 == 1,
            queue_stats: value >> 6 & 1 == 1,
            port_blocked: value >> 8 & 1 == 1,
        }
    }
}

impl From<Capabilities> for u32 {
    fn from(value: Capabilities) -> Self {
        (value.flow_stats as u32)
            | ((value.table_stats as u32) << 1)
            | (value.port_stats as u32) << 2
            | (value.group_stats as u32) << 3
            | (value.ip_reasm as u32) << 5
            | (value.queue_stats as u32) << 6
            | (value.port_blocked as u32) << 8
    }
}
