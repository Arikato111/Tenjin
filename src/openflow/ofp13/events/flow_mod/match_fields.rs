use std::{
    io::{BufRead, Cursor, Error},
    mem::transmute,
    net::{Ipv4Addr, Ipv6Addr},
};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::etherparser::MacAddr;

/**
 * +---------------+---------------+---------------------------+
 * |     Type      |     Length    |      OXM Fields           |
 * +---------------+---------------+---------------------------+
 * |  (Optional)   |  (16 bits)    | (Array of variable length)|
 * |---------------+---------------+---------------------------+
 */

pub struct OfpMatch {
    typ: MatchType,
    length: u16,
    oxm_fields: Vec<u8>,
}

impl OfpMatch {
    pub fn new() -> Self {
        Self {
            typ: MatchType::OXM,
            length: 4,
            oxm_fields: Vec::new(),
        }
    }
    pub fn marshal(&self, bytes: &mut Vec<u8>) {
        bytes.write_u16::<BigEndian>(self.typ.clone().into());
        bytes.write_u16::<BigEndian>(self.length + (self.oxm_fields.len() as u16));
        bytes.append(&mut self.oxm_fields.clone());
        bytes.write_u32::<BigEndian>(0);
    }
}

#[derive(Clone)]
#[repr(u16)]
pub enum MatchType {
    Standard = 0,
    OXM = 1, //, the OpenFlow 1.1 match type OFPMT_STANDARD is deprecated
}

impl From<u16> for MatchType {
    fn from(value: u16) -> Self {
        match value {
            0 => Self::Standard,
            _ => Self::OXM,
        }
    }
}

impl From<MatchType> for u16 {
    fn from(value: MatchType) -> Self {
        value as u16
    }
}

/**
*
* +----------------------------+----------------------------+
* |        OXM Header          |       OXM Body             |
* +----------------------------+----------------------------+
* | Version (1 byte)           |  Value (variable length)   |
* | OXM Type (1 byte)          |  Mask   (variable length)  |
* | Length (2 bytes)           |                            |
* | OXM ID (4 bytes) (Optional)|                            |
* +----------------------------+----------------------------+
*
*/

pub struct OxmHeader {
    class: OxmClass,       // Match class: member class or reserved class
    field: OxmMatchFields, // 7bit Match field within the class
    hasmask: bool,         // 1bit Set if OXM include a bitmask in payload
    length: u8,            // Length of OXM payload
    experimenter: Option<u32>,
}

impl OxmHeader {
    pub fn new(field: OxmMatchFields, size: u8, hasmask: bool) -> Self {
        Self {
            class: OxmClass::OpenflowBasic,
            field,
            hasmask: hasmask,
            length: size,
            experimenter: None,
        }
    }
    pub fn marshal(&self, bytes: &mut Vec<u8>) {
        bytes.write_u16::<BigEndian>(self.class.clone().into());
        let field: u8 = self.field.clone().into();
        bytes.write_u8(field << 1 | if self.hasmask { 1 } else { 0 });
        bytes.write_u8(self.length);
        // if let Some(exp) = self.experimenter {
        // bytes.write_u32::<BigEndian>(exp);
        // }
    }
}

/**
 * NXM allocates only two vendors,
 * 0x0000 for fields supported by OpenFlow 1.0
 * and 0x0001 for fields implemented as an Open vSwitch extension
 */

#[derive(Clone)]
#[repr(u16)]
pub enum OxmClass {
    Nxm0 = 0x0000,          // Backward compatibility with NXM
    Nxm1 = 0x0001,          // Backward compatibility with NXM
    OpenflowBasic = 0x8000, // Basic class for OpenFlow
    Experimenter = 0xffff,  // Experimenter class
}

impl From<OxmClass> for u16 {
    fn from(value: OxmClass) -> Self {
        value as u16
    }
}

#[derive(Clone)]
#[repr(u8)]
/* OXM Flow match field types for OpenFlow basic class. */
pub enum OxmMatchFields {
    InPort = 0,        /* Switch input port. */
    InPhyPort = 1,     /* Switch physical input port. */
    METADATA = 2,      /* Metadata passed between tables. */
    EthDst = 3,        /* Ethernet destination address. */
    EthSrc = 4,        /* Ethernet source address. */
    EthType = 5,       /* Ethernet frame type. */
    VlanVid = 6,       /* VLAN id. */
    VlanPcp = 7,       /* VLAN priority. */
    IpDscp = 8,        /* IP DSCP (6 bits in ToS field). */
    IpEcn = 9,         /* IP ECN (2 bits in ToS field). */
    IpProto = 10,      /* IP protocol. */
    Ipv4Src = 11,      /* IPv4 source address. */
    Ipv4Dst = 12,      /* IPv4 destination address. */
    TcpSrc = 13,       /* TCP source port. */
    TcpDst = 14,       /* TCP destination port. */
    UdpSrc = 15,       /* UDP source port. */
    UdpDst = 16,       /* UDP destination port. */
    SctpSrc = 17,      /* SCTP source port. */
    SctpDst = 18,      /* SCTP destination port. */
    Icmpv4Type = 19,   /* ICMP type. */
    Icmpv4Code = 20,   /* ICMP code. */
    ArpOp = 21,        /* ARP opcode. */
    ArpSpa = 22,       /* ARP source IPv4 address. */
    ArpTpa = 23,       /* ARP target IPv4 address. */
    ArpSha = 24,       /* ARP source hardware address. */
    ArpTha = 25,       /* ARP target hardware address. */
    Ipv6Src = 26,      /* IPv6 source address. */
    Ipv6Dst = 27,      /* IPv6 destination address. */
    Ipv6Flabel = 28,   /* IPv6 Flow Label */
    Icmpv6Type = 29,   /* ICMPv6 type. */
    Icmpv6Code = 30,   /* ICMPv6 code. */
    Ipv6NdTarget = 31, /* Target address for ND. */
    Ipv6NdSll = 32,    /* Source link-layer for ND. */
    Ipv6NdTll = 33,    /* Target link-layer for ND. */
    MplsLabel = 34,    /* MPLS label. */
    MplsTc = 35,       /* MPLS TC. */
    MplsBos = 36,      /* MPLS BoS bit. */
    PbbIsid = 37,      /* PBB I-SID. */
    TunnelId = 38,     /* Logical Port Metadata. */
    Ipv6Exthdr = 39,   /* IPv6 Extension Header pseudo-field */
    Unparse,
}

impl From<u8> for OxmMatchFields {
    fn from(value: u8) -> Self {
        if value < 44 {
            unsafe { transmute(value) }
        } else {
            Self::Unparse
        }
    }
}

impl From<OxmMatchFields> for u8 {
    fn from(value: OxmMatchFields) -> Self {
        value as u8
    }
}

// Required match fields.
pub struct MatchFields {
    pub in_port: Option<u32>, // Ingress port. This may be a physical or switch-defined logical port.
    pub eth_dst: Option<MacAddr>, // Ethernet source address. Can use arbitrary bitmask
    pub eth_src: Option<MacAddr>, // Ethernet destination address. Can use arbitrary bitmask
    pub eth_typ: Option<u16>, // Ethernet type of the OpenFlow packet payload, after VLAN tags.
    pub ip_proto: Option<u8>, // IPv4 or IPv6 protocol number
    pub ipv4_src: Option<Ipv4Addr>, // IPv4 source address. Can use subnet mask or arbitrary bitmask
    pub ipv4_dst: Option<Ipv4Addr>, // IPv4 destination address. Can use subnet mask or arbitrary bitmask
    pub ipv6_src: Option<Ipv6Addr>, // IPv6 source address. Can use subnet mask or arbitrary bitmask
    pub ipv6_dst: Option<Ipv6Addr>, // IPv6 destination address. Can use subnet mask or arbitrary bitmask
    pub tcp_src: Option<u16>,       // TCP source port
    pub tcp_dst: Option<u16>,       // TCP destination port
    pub udp_src: Option<u16>,       // UDP source port
    pub udp_dst: Option<u16>,       // UDP destination port
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
        let mut ofp_match = OfpMatch::new();
        let ofp_byte = ofp_match.oxm_fields.as_mut();

        if let Some(in_port) = &self.in_port {
            let header = OxmHeader::new(OxmMatchFields::InPort, 4, false);
            header.marshal(ofp_byte);
            ofp_byte.write_u32::<BigEndian>(*in_port);
        }
        if let Some(eth_dst) = &self.eth_dst {
            let header = OxmHeader::new(OxmMatchFields::EthDst, 12, true);
            header.marshal(ofp_byte);
            eth_dst.marshal(ofp_byte);
            // mac mask
            MacAddr::from(!0).marshal(ofp_byte);
        }
        if let Some(eth_src) = &self.eth_src {
            let header = OxmHeader::new(OxmMatchFields::EthSrc, 12, true);
            header.marshal(ofp_byte);
            eth_src.marshal(ofp_byte);
            // mac mask
            MacAddr::from(!0).marshal(ofp_byte);
        }
        if let Some(eth_typ) = &self.eth_typ {
            OxmHeader::new(OxmMatchFields::EthType, 2, false).marshal(ofp_byte);
            ofp_byte.write_u16::<BigEndian>(*eth_typ);
        }
        if let Some(ip_proto) = &self.ip_proto {
            OxmHeader::new(OxmMatchFields::IpProto, 1, false).marshal(ofp_byte);
            ofp_byte.write_u8(*ip_proto);
        }
        if let Some(ipv4_src) = &self.ipv4_src {
            OxmHeader::new(OxmMatchFields::Ipv4Src, 8, true).marshal(ofp_byte);
            bytes.write_u32::<BigEndian>(ipv4_src.clone().into());
            bytes.write_u32::<BigEndian>(!0);
        }
        if let Some(ipv4_dst) = &self.ipv4_dst {
            OxmHeader::new(OxmMatchFields::Ipv4Dst, 8, true).marshal(ofp_byte);
            bytes.write_u32::<BigEndian>(ipv4_dst.clone().into());
            bytes.write_u32::<BigEndian>(!0);
        }
        if let Some(ipv6_src) = &self.ipv6_src {
            OxmHeader::new(OxmMatchFields::Ipv6Src, 32, true).marshal(ofp_byte);
            ofp_byte.write_u128::<BigEndian>(ipv6_src.clone().into());
            ofp_byte.write_u128::<BigEndian>(!0);
        }
        if let Some(ipv6_dst) = &self.ipv6_dst {
            OxmHeader::new(OxmMatchFields::Ipv6Dst, 32, true).marshal(ofp_byte);
            ofp_byte.write_u128::<BigEndian>(ipv6_dst.clone().into());
            ofp_byte.write_u128::<BigEndian>(!0);
        }
        if let Some(tcp_src) = &self.tcp_src {
            OxmHeader::new(OxmMatchFields::TcpSrc, 2, false);
            ofp_byte.write_u16::<BigEndian>(*tcp_src);
        }
        if let Some(tcp_dst) = &self.tcp_dst {
            OxmHeader::new(OxmMatchFields::TcpDst, 2, false);
            ofp_byte.write_u16::<BigEndian>(*tcp_dst);
        }
        if let Some(udp_src) = &self.udp_src {
            OxmHeader::new(OxmMatchFields::UdpSrc, 2, false);
            ofp_byte.write_u16::<BigEndian>(*udp_src);
        }
        if let Some(udp_dst) = &self.udp_dst {
            OxmHeader::new(OxmMatchFields::UdpDst, 2, false);
            ofp_byte.write_u16::<BigEndian>(*udp_dst);
        }
        ofp_match.marshal(bytes);
    }

    pub fn parse(bytes: &mut Cursor<Vec<u8>>) -> Result<MatchFields, Error> {
        let mut matcher = MatchFields::match_all();

        let typ: MatchType = bytes.read_u16::<BigEndian>()?.into();
        let length = bytes.read_u16::<BigEndian>()?;
        let mut pkt_len = length - 4;
        while pkt_len > 0 {
            let oxm_class = bytes.read_u16::<BigEndian>()?;
            let oxm_field = bytes.read_u8()?;
            let hash_mask = oxm_field & 1 == 1;
            let oxm_field: OxmMatchFields = (oxm_field >> 1).into();
            let oxm_length = bytes.read_u8()?;
            match oxm_field {
                OxmMatchFields::InPort => {
                    let port = bytes.read_u32::<BigEndian>()?;
                    let mask = if hash_mask {
                        Some(bytes.read_u32::<BigEndian>())
                    } else {
                        None
                    };
                    matcher.in_port = Some(port);
                }
                OxmMatchFields::EthDst => {
                    let mut mac = [0u8; 6];
                    for i in 0..6 {
                        mac[i] = bytes.read_u8()?;
                    }
                    if hash_mask {
                        bytes.consume(6);
                    }
                    matcher.eth_dst = Some(MacAddr::new(mac));
                }
                OxmMatchFields::EthSrc => {
                    let mut mac = [0u8; 6];
                    for i in 0..6 {
                        mac[i] = bytes.read_u8()?;
                    }
                    if hash_mask {
                        bytes.consume(6);
                    }
                    matcher.eth_src = Some(MacAddr::new(mac));
                }
                OxmMatchFields::EthType => {
                    let eth_typ = bytes.read_u16::<BigEndian>()?;
                    if hash_mask {
                        bytes.consume(2);
                    }
                    matcher.eth_typ = Some(eth_typ);
                }
                OxmMatchFields::IpProto => {
                    let proto = bytes.read_u8()?;
                    if hash_mask {
                        bytes.consume(1);
                    }
                    matcher.ip_proto = Some(proto);
                }
                OxmMatchFields::Ipv4Src => {
                    let ip = bytes.read_u32::<BigEndian>()?;
                    if hash_mask {
                        bytes.consume(4);
                    }
                    matcher.ipv4_src = Some(Ipv4Addr::from(ip));
                }
                OxmMatchFields::Ipv4Dst => {
                    let ip = bytes.read_u32::<BigEndian>()?;
                    if hash_mask {
                        bytes.consume(4);
                    }
                    matcher.ipv4_dst = Some(Ipv4Addr::from(ip));
                }
                OxmMatchFields::Ipv6Src => {
                    let ipv6 = bytes.read_u128::<BigEndian>()?;
                    if hash_mask {
                        bytes.consume(16);
                    }
                    matcher.ipv6_src = Some(Ipv6Addr::from(ipv6));
                }
                OxmMatchFields::Ipv6Dst => {
                    let ipv6 = bytes.read_u128::<BigEndian>()?;
                    if hash_mask {
                        bytes.consume(16);
                    }
                    matcher.ipv6_dst = Some(Ipv6Addr::from(ipv6));
                }
                OxmMatchFields::TcpSrc => {
                    let tcp = bytes.read_u16::<BigEndian>()?;
                    if hash_mask {
                        bytes.consume(2);
                    }
                    matcher.tcp_src = Some(tcp);
                }
                OxmMatchFields::TcpDst => {
                    let tcp = bytes.read_u16::<BigEndian>()?;
                    if hash_mask {
                        bytes.consume(2);
                    }
                    matcher.tcp_dst = Some(tcp);
                }
                OxmMatchFields::UdpSrc => {
                    let udp = bytes.read_u16::<BigEndian>()?;
                    if hash_mask {
                        bytes.consume(2);
                    }
                    matcher.udp_src = Some(udp);
                }
                OxmMatchFields::UdpDst => {
                    let udp = bytes.read_u16::<BigEndian>()?;
                    if hash_mask {
                        bytes.consume(2);
                    }
                    matcher.udp_dst = Some(udp);
                }

                _ => {
                    bytes.consume((oxm_length - 4) as usize);
                }
            }
            // 4 is size of oxm_tlv_header
            pkt_len = pkt_len - (oxm_length as u16 + 4);
        }
        if length % 8 != 0 {
            bytes.consume(4);
        }
        Ok(matcher)
    }
}
