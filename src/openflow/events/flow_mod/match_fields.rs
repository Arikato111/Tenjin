use std::io::{BufRead, Cursor};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::etherparser::tools::bits::{bit_bool, mac_to_bytes, set_bit};

pub struct Mask<T> {
    pub ip: T,
    pub mask: Option<T>,
}

impl Mask<u32> {
    pub fn to_int(&self) -> u32 {
        match self.mask {
            Some(v) => v,
            None => 0,
        }
    }
}

struct Wildcards {
    pub in_port: bool,
    pub mac_dest: bool,
    pub mac_src: bool,
    pub ethernet_type: bool,

    pub vlan_vid: bool,
    pub vlan_pcp: bool,

    pub ip_src: u32,
    pub ip_dest: u32,
    pub protocol: bool,
    pub tos: bool,
    pub transport_src: bool,
    pub transport_dest: bool,
}

impl Wildcards {
    pub fn parse(byte: u32) -> Wildcards {
        Wildcards {
            in_port: bit_bool(0, byte),
            vlan_vid: bit_bool(1, byte),
            mac_src: bit_bool(2, byte),
            mac_dest: bit_bool(3, byte),
            ethernet_type: bit_bool(4, byte),
            protocol: bit_bool(5, byte),
            transport_src: bit_bool(6, byte),
            transport_dest: bit_bool(7, byte),
            ip_src: Wildcards::get_nw_mask(byte, 8),
            ip_dest: Wildcards::get_nw_mask(byte, 14),
            vlan_pcp: bit_bool(20, byte),
            tos: bit_bool(21, byte),
        }
    }
    pub fn get_nw_mask(f: u32, offset: usize) -> u32 {
        (f >> offset) & 0x3f
    }
}

pub struct MatchFields {
    pub in_port: Option<u16>,
    pub mac_dest: Option<u64>,
    pub mac_src: Option<u64>,
    pub ethernet_type: Option<u16>,

    pub vlan_vid: Option<u16>, // vlan type
    pub vlan_pcp: Option<u8>,

    pub ip_src: Option<Mask<u32>>,
    pub ip_dest: Option<Mask<u32>>,
    pub protocol: Option<u8>,
    pub tos: Option<u8>,
    pub transport_src: Option<u16>,
    pub transport_dest: Option<u16>,
}

impl MatchFields {
    pub fn match_all() -> Self {
        Self {
            ethernet_type: None,
            in_port: None,
            ip_dest: None,
            ip_src: None,
            mac_dest: None,
            mac_src: None,
            protocol: None,
            tos: None,
            transport_dest: None,
            transport_src: None,
            vlan_pcp: None,
            vlan_vid: None,
        }
    }
    pub fn marshal(&self, bytes: &mut Vec<u8>) {
        let mut match_f = 0u32;
        match_f = set_bit(match_f, 0, self.in_port.is_some());
        match_f = set_bit(match_f, 1, self.vlan_vid.is_some());
        match_f = set_bit(match_f, 2, self.mac_src.is_some());
        match_f = set_bit(match_f, 3, self.mac_dest.is_some());
        match_f = set_bit(match_f, 4, self.ethernet_type.is_some());
        match_f = set_bit(match_f, 5, self.protocol.is_some());
        match_f = set_bit(match_f, 6, self.ip_src.is_some());
        match_f = set_bit(match_f, 7, self.ip_dest.is_some());
        match_f = MatchFields::set_nw_mask(match_f, 8, self.ip_src.as_ref().unwrap().ip);
        match_f = MatchFields::set_nw_mask(match_f, 14, self.ip_dest.as_ref().unwrap().ip);
        match_f = set_bit(match_f, 20, self.vlan_pcp.is_some());
        match_f = set_bit(match_f, 21, self.tos.is_some());
        bytes.write_u32::<BigEndian>(match_f).unwrap();
    }

    pub fn set_nw_mask(byte: u32, offset: usize, set: u32) -> u32 {
        let value = (0x3f & set) << offset;
        byte | value
    }
    pub fn get_ns_mask(byte: u32, offset: usize) -> u32 {
        (byte >> offset) & 0x3f
    }
    pub fn parse(bytes: &mut Cursor<Vec<u8>>) -> MatchFields {
        let wildcards = Wildcards::parse(bytes.read_u32::<BigEndian>().unwrap());
        let in_port = if wildcards.in_port {
            None
        } else {
            Some(bytes.read_u16::<BigEndian>().unwrap())
        };
        let mac_src = if wildcards.mac_src {
            None
        } else {
            let mut arr: [u8; 6] = [0; 6];
            for i in 0..6 {
                arr[i] = bytes.read_u8().unwrap();
            }
            Some(mac_to_bytes(arr))
        };
        let mac_dest = if wildcards.mac_dest {
            None
        } else {
            let mut arr: [u8; 6] = [0; 6];
            for i in 0..6 {
                arr[i] = bytes.read_u8().unwrap();
            }
            Some(mac_to_bytes(arr))
        };
        let vlan_vid = if wildcards.vlan_vid {
            None
        } else {
            let vid = bytes.read_u16::<BigEndian>().unwrap();
            if vid == 0xfff {
                None
            } else {
                Some(bytes.read_u16::<BigEndian>().unwrap())
            }
        };
        let vlan_pcp = if wildcards.vlan_pcp {
            None
        } else {
            Some(bytes.read_u8().unwrap())
        };
        bytes.consume(1);
        let ethernet_type = if wildcards.ethernet_type {
            None
        } else {
            Some(bytes.read_u16::<BigEndian>().unwrap())
        };
        let tos = if wildcards.tos {
            None
        } else {
            Some(bytes.read_u8().unwrap())
        };
        let protocol = if wildcards.protocol {
            None
        } else {
            Some(bytes.read_u8().unwrap())
        };
        bytes.consume(2);
        let ip_src = if wildcards.ip_src >= 32 {
            None
        } else if wildcards.ip_src == 0 {
            Some(Mask {
                ip: bytes.read_u32::<BigEndian>().unwrap(),
                mask: None,
            })
        } else {
            Some(Mask {
                ip: bytes.read_u32::<BigEndian>().unwrap(),
                mask: Some(wildcards.ip_src),
            })
        };
        let ip_dest = if wildcards.ip_dest >= 32 {
            None
        } else if wildcards.ip_dest == 0 {
            Some(Mask {
                ip: bytes.read_u32::<BigEndian>().unwrap(),
                mask: None,
            })
        } else {
            Some(Mask {
                ip: bytes.read_u32::<BigEndian>().unwrap(),
                mask: Some(wildcards.ip_dest),
            })
        };
        let transport_src = if wildcards.transport_src {
            None
        } else {
            Some(bytes.read_u16::<BigEndian>().unwrap())
        };
        let transport_dest = if wildcards.transport_dest {
            None
        } else {
            Some(bytes.read_u16::<BigEndian>().unwrap())
        };
        MatchFields {
            in_port,
            mac_src,
            mac_dest,
            ethernet_type,
            vlan_vid,
            vlan_pcp,
            ip_src,
            ip_dest,
            protocol,
            tos,
            transport_src,
            transport_dest,
        }
    }
}
