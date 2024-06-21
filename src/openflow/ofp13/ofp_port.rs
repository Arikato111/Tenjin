use byteorder::{BigEndian, WriteBytesExt};

#[repr(u32)]
pub enum OfpPort {
    Max = 0xffffff00,
    InPort = 0xfffffff8,
    Table = 0xfffffff9,
    Normal = 0xfffffffa,
    Flood = 0xfffffffb,
    All = 0xfffffffc,
    Controller = 0xfffffffd,
    Local = 0xfffffffe,
    Any = 0xffffffff,
}

#[derive(Clone)]
pub enum PseudoPort {
    PhysicalPort(u32),
    InPort,
    Table,
    Normal,
    Flood,
    AllPorts,
    Controller(u64),
    Local,
    Unsupport,
}

impl PseudoPort {
    pub fn parse(byte: u32) -> Option<PseudoPort> {
        Some(PseudoPort::new(byte, Some(0)))
    }
    pub fn new(port: u32, len: Option<u64>) -> PseudoPort {
        match port {
            p if p == (OfpPort::InPort as u32) => PseudoPort::InPort,
            p if p == (OfpPort::Table as u32) => PseudoPort::Table,
            p if p == (OfpPort::Normal as u32) => PseudoPort::Normal,
            p if p == (OfpPort::Flood as u32) => PseudoPort::Flood,
            p if p == (OfpPort::All as u32) => PseudoPort::AllPorts,
            p if p == (OfpPort::Controller as u32) => match len {
                Some(len) => PseudoPort::Controller(len),
                None => PseudoPort::Unsupport,
            },
            p if p == (OfpPort::Local as u32) => PseudoPort::InPort,
            _ => {
                if port <= (OfpPort::Max as u32) {
                    PseudoPort::PhysicalPort(port)
                } else {
                    PseudoPort::Unsupport
                }
            }
        }
    }
    pub fn marshal(&self, bytes: &mut Vec<u8>) {
        let port = match *self {
            PseudoPort::PhysicalPort(p) => p,
            PseudoPort::InPort => OfpPort::InPort as u32,
            PseudoPort::Table => OfpPort::Table as u32,
            PseudoPort::Normal => OfpPort::Normal as u32,
            PseudoPort::Flood => OfpPort::Flood as u32,
            PseudoPort::AllPorts => OfpPort::All as u32,
            PseudoPort::Controller(_) => OfpPort::Controller as u32,
            PseudoPort::Local => OfpPort::Local as u32,
            // not sure how to handle unsupport
            PseudoPort::Unsupport => OfpPort::Flood as u32,
        };
        let _ = bytes.write_u32::<BigEndian>(port);
    }
}
