use byteorder::{BigEndian, WriteBytesExt};

pub enum OfpPort {
    Max = 0xff00,
    InPort = 0xfff8,
    Table = 0xfff9,
    Normal = 0xfffa,
    Flood = 0xfffb,
    All = 0xfffc,
    Controller = 0xfffd,
    Local = 0xfffe,
    None = 0xffff,
}

pub enum PseudoPort {
    PhysicalPort(u16),
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
    pub fn parse(byte: u16) -> Option<PseudoPort> {
        if (OfpPort::None as u16) == byte {
            None
        } else {
            Some(PseudoPort::new(byte, Some(0)))
        }
    }
    pub fn new(port: u16, len: Option<u64>) -> PseudoPort {
        match port {
            p if p == (OfpPort::InPort as u16) => PseudoPort::InPort,
            p if p == (OfpPort::Table as u16) => PseudoPort::Table,
            p if p == (OfpPort::Normal as u16) => PseudoPort::Normal,
            p if p == (OfpPort::Flood as u16) => PseudoPort::Flood,
            p if p == (OfpPort::All as u16) => PseudoPort::AllPorts,
            p if len.is_some() && p == (OfpPort::Controller as u16) => {
                PseudoPort::Controller(len.unwrap())
            }
            p if p == (OfpPort::Local as u16) => PseudoPort::InPort,
            _ => {
                if port <= (OfpPort::Max as u16) {
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
            PseudoPort::InPort => OfpPort::InPort as u16,
            PseudoPort::Table => OfpPort::Table as u16,
            PseudoPort::Normal => OfpPort::Normal as u16,
            PseudoPort::Flood => OfpPort::Flood as u16,
            PseudoPort::AllPorts => OfpPort::All as u16,
            PseudoPort::Controller(_) => OfpPort::Controller as u16,
            PseudoPort::Local => OfpPort::Local as u16,
            // not sure how to handle unsupport
            PseudoPort::Unsupport => OfpPort::Flood as u16,
        };
        let _ = bytes.write_u16::<BigEndian>(port);
    }
}
