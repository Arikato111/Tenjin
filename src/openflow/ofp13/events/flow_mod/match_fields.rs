use std::{
    io::{BufRead, Cursor, Error},
    net::{Ipv4Addr, Ipv6Addr},
};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::etherparser::{
    tools::bits::{bit_bool, bytes_to_mac, mac_to_bytes, set_bit},
    MacAddr,
};

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
    pub fn from_match_fields(match_fields: &MatchFields) -> Wildcards {
        Wildcards {
            in_port: match_fields.in_port.is_none(),
            vlan_vid: match_fields.vlan_vid.is_none(),
            mac_src: match_fields.mac_src.is_none(),
            mac_dest: match_fields.mac_dest.is_none(),
            ethernet_type: match_fields.ethernet_type.is_none(),
            protocol: match_fields.protocol.is_none(),
            transport_src: match_fields.transport_src.is_none(),
            transport_dest: match_fields.transport_dest.is_none(),
            ip_src: Wildcards::mask_bits(&match_fields.ip_src),
            ip_dest: Wildcards::mask_bits(&match_fields.ip_dest),
            vlan_pcp: match_fields.vlan_pcp.is_none(),
            tos: match_fields.tos.is_none(),
        }
    }
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
    pub fn marshal(&self, bytes: &mut Vec<u8>) {
        let mut match_field = 0u32;
        match_field = set_bit(match_field, 0, self.in_port);
        match_field = set_bit(match_field, 1, self.vlan_vid);
        match_field = set_bit(match_field, 2, self.mac_src);
        match_field = set_bit(match_field, 3, self.mac_dest);
        match_field = set_bit(match_field, 4, self.ethernet_type);
        match_field = set_bit(match_field, 5, self.protocol);
        match_field = set_bit(match_field, 6, self.transport_src);
        match_field = set_bit(match_field, 7, self.transport_dest);
        match_field = Wildcards::set_nw_mask(match_field, 8, self.ip_src);
        match_field = Wildcards::set_nw_mask(match_field, 14, self.ip_dest);
        match_field = set_bit(match_field, 20, self.vlan_pcp);
        match_field = set_bit(match_field, 21, self.tos);
        let _ = bytes.write_u32::<BigEndian>(match_field);
    }
    pub fn get_nw_mask(f: u32, offset: usize) -> u32 {
        (f >> offset) & 0x3f
    }
    pub fn set_nw_mask(byte: u32, offset: usize, set: u32) -> u32 {
        let value = (0x3f & set) << offset;
        byte | value
    }
    pub fn mask_bits(mask: &Option<Mask<u32>>) -> u32 {
        match mask {
            None => 32,
            Some(m) => match m.mask {
                None => 0,
                Some(m) => m,
            },
        }
    }
}

pub struct OfpMatch {
    typ: MatchType,
    length: u16,
    oxm_fields: OxmClass,
}

pub enum MatchType {
    Standard = 0,
    OXM = 1, //, the OpenFlow 1.1 match type OFPMT_STANDARD is deprecated
}

/**
 * | class | field | length | - |     body     |
 * | :---: | :---: | :----: | - |     :--:     |
 * |  16   |   7   |   1    | - | length bytes |
 */

pub struct OxmHeader {
    class: OxmClass,       // Match class: member class or reserved class
    field: OxmMatchFields, // 7bit Match field within the class
    hasmask: bool,         // 1bit Set if OXM include a bitmask in payload
    length: u8,            // Length of OXM payload
}

/**
 * NXM allocates only two vendors,
 * 0x0000 for fields supported by OpenFlow 1.0
 * and 0x0001 for fields implemented as an Open vSwitch extension
 */

#[repr(u16)]
pub enum OxmClass {
    Nxm0 = 0x0000,          // Backward compatibility with NXM
    Nxm1 = 0x0001,          // Backward compatibility with NXM
    OpenflowBasic = 0x8000, // Basic class for OpenFlow
    Experimenter = 0xffff,  // Experimenter class
}

/* OXM Flow match field types for OpenFlow basic class. */
pub enum OxmMatchFields {
    InPort = 1,
    InPhyPort = 2,
    Metadata = 3,
    MacDest = 4,
    MacSrc = 5,
    EthernetType = 6,
    VlanVid = 7, // vlan type
    VlanPcp = 8,
    // ToS from IPv4 packet
    IpDscp = 9, // IP DSCP (6 bits in ToS field).
    IpEcn = 10, // IP ECN (2 bits in ToS field).
    Protocol = 11,
    IpSrc = 12,
    IpDst = 13,

    TcpSrc = 14,
    TcpDst = 15,
    UdpSrc = 16,
    UdpDst = 17,
    SctpSrc = 18,
    SctpDst = 19,

    Icmpv4Type = 20,
    Icmpv4Code = 21,
    ArpOp = 22,
    ArpSpa = 23,       // ARP source IPv4 address
    ArpTpa = 24,       // ARP target IPv4 address
    ArpSha = 25,       // ARP source Mac
    ArpPha = 26,       // ARP target Mac
    Ipv6Src = 27,      // IPv6 address
    Ipv6Dst = 28,      // IPv6 address
    Ipv6Flabel = 29,   // IPv6 Flow Lable
    Icmpv6Type = 30,   // ICMPv6 type
    Icmpv6Code = 31,   // ICMPv6 code
    Ipv6NdTarget = 32, // Target address for ND
    Ipv6NdSll = 33,    // MAC , source link-layer for ND
    Ipv6NdTll = 34,    // Mac , Target link-layer for ND
    MplsLabel = 35,    // MPLS label
    MplsTc = 36,       // MPLS TC
    MplsBos = 37,      // MPLS Bos bit
    PbbIsid = 38,      // 24bit PBB I-SID
    TunnelId = 39,     // Logical Port Metadata
    Ipv6Exthdr = 40,   // IPv6 Extension Header pseudo-field
    PbbUca = 41,       // PBB UCA Header
    TcpFlags = 42,     // TCP Flags
    ActsetOutput = 43, // Output port from action set metadata
}

// Required match fields.
pub struct MatchFields {
    in_port: Option<u32>, // Ingress port. This may be a physical or switch-defined logical port.
    eth_dst: Option<MacAddr>, // Ethernet source address. Can use arbitrary bitmask
    eth_src: Option<MacAddr>, // Ethernet destination address. Can use arbitrary bitmask
    eth_typ: Option<u16>, // Ethernet type of the OpenFlow packet payload, after VLAN tags.
    ip_proto: Option<u8>, // IPv4 or IPv6 protocol number
    ipv4_src: Option<Ipv4Addr>, // IPv4 source address. Can use subnet mask or arbitrary bitmask
    ipv4_dst: Option<Ipv4Addr>, // IPv4 destination address. Can use subnet mask or arbitrary bitmask
    ipv6_src: Option<Ipv6Addr>, // IPv6 source address. Can use subnet mask or arbitrary bitmask
    ipv6_dst: Option<Ipv6Addr>, // IPv6 destination address. Can use subnet mask or arbitrary bitmask
    tcp_src: Option<u16>,       // TCP source port
    tcp_dst: Option<u16>,       // TCP destination port
    udp_src: Option<u16>,       // UDP source port
    udp_dst: Option<u16>,       // UDP destination port
}

impl MatchFields {
    pub fn match_all() -> Self {
        Self {
            in_port: None,
            eth_dst: None,
            eth_src: None,
            eth_typ: None,
            ip_proto: None,
            ipv4_src: None,
            ipv4_dst: None,
            ipv6_src: None,
            ipv6_dst: None,
            tcp_src: None,
            tcp_dst: None,
            udp_src: None,
            udp_dst: None,
        }
    }
    pub fn marshal(&self, bytes: &mut Vec<u8>) {
        let wildcard = Wildcards::from_match_fields(self);
        wildcard.marshal(bytes);
        let _ = bytes.write_u16::<BigEndian>(match self.in_port {
            Some(p) => p,
            None => 0,
        });
        let mac_src = match self.mac_src {
            Some(mac) => bytes_to_mac(mac),
            None => bytes_to_mac(0),
        };
        for m in mac_src {
            let _ = bytes.write_u8(m);
        }
        let mac_dest = match self.mac_dest {
            Some(mac) => bytes_to_mac(mac),
            None => bytes_to_mac(0),
        };
        for m in mac_dest {
            let _ = bytes.write_u8(m);
        }
        let vlan = match self.vlan_vid {
            Some(v) => v,
            None => 0xffff,
        };
        let _ = bytes.write_u16::<BigEndian>(vlan);
        let _ = bytes.write_u8(match self.vlan_pcp {
            Some(v) => v,
            None => 0,
        });
        let _ = bytes.write_u8(0);
        let _ = bytes.write_u16::<BigEndian>(match self.ethernet_type {
            Some(v) => v,
            None => 0,
        });
        let _ = bytes.write_u8(match self.tos {
            Some(v) => v,
            None => 0,
        });
        let _ = bytes.write_u8(match self.protocol {
            Some(v) => v,
            None => 0,
        });
        let _ = bytes.write_u16::<BigEndian>(0);

        let _ = bytes.write_u32::<BigEndian>(match &self.ip_src {
            Some(ip) => ip.ip,
            None => 0,
        });
        let _ = bytes.write_u32::<BigEndian>(match &self.ip_dest {
            Some(ip) => ip.ip,
            None => 0,
        });
        let _ = bytes.write_u16::<BigEndian>(match self.transport_src {
            Some(v) => v,
            None => 0,
        });
        let _ = bytes.write_u16::<BigEndian>(match self.transport_dest {
            Some(v) => v,
            None => 0,
        });
    }

    pub fn parse(bytes: &mut Cursor<Vec<u8>>) -> Result<MatchFields, Error> {
        let wildcards = Wildcards::parse(bytes.read_u32::<BigEndian>()?);
        let in_port = if wildcards.in_port {
            None
        } else {
            Some(bytes.read_u16::<BigEndian>()?)
        };
        let mac_src = if wildcards.mac_src {
            None
        } else {
            let mut arr: [u8; 6] = [0; 6];
            for i in 0..6 {
                arr[i] = bytes.read_u8()?;
            }
            Some(mac_to_bytes(arr))
        };
        let mac_dest = if wildcards.mac_dest {
            None
        } else {
            let mut arr: [u8; 6] = [0; 6];
            for i in 0..6 {
                arr[i] = bytes.read_u8()?;
            }
            Some(mac_to_bytes(arr))
        };
        let vlan_vid = if wildcards.vlan_vid {
            None
        } else {
            let vid = bytes.read_u16::<BigEndian>()?;
            if vid == 0xfff {
                None
            } else {
                Some(bytes.read_u16::<BigEndian>()?)
            }
        };
        let vlan_pcp = if wildcards.vlan_pcp {
            None
        } else {
            Some(bytes.read_u8()?)
        };
        bytes.consume(1);
        let ethernet_type = if wildcards.ethernet_type {
            None
        } else {
            Some(bytes.read_u16::<BigEndian>()?)
        };
        let tos = if wildcards.tos {
            None
        } else {
            Some(bytes.read_u8()?)
        };
        let protocol = if wildcards.protocol {
            None
        } else {
            Some(bytes.read_u8()?)
        };
        bytes.consume(2);
        let ip_src = if wildcards.ip_src >= 32 {
            None
        } else if wildcards.ip_src == 0 {
            Some(Mask {
                ip: bytes.read_u32::<BigEndian>()?,
                mask: None,
            })
        } else {
            Some(Mask {
                ip: bytes.read_u32::<BigEndian>()?,
                mask: Some(wildcards.ip_src),
            })
        };
        let ip_dest = if wildcards.ip_dest >= 32 {
            None
        } else if wildcards.ip_dest == 0 {
            Some(Mask {
                ip: bytes.read_u32::<BigEndian>()?,
                mask: None,
            })
        } else {
            Some(Mask {
                ip: bytes.read_u32::<BigEndian>()?,
                mask: Some(wildcards.ip_dest),
            })
        };
        let transport_src = if wildcards.transport_src {
            None
        } else {
            Some(bytes.read_u16::<BigEndian>()?)
        };
        let transport_dest = if wildcards.transport_dest {
            None
        } else {
            Some(bytes.read_u16::<BigEndian>()?)
        };
        Ok(MatchFields {
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
        })
    }
}
