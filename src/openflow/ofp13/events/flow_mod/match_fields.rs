use std::{
    error::Error,
    io::{BufRead, Cursor},
    mem::transmute,
    net::{Ipv4Addr, Ipv6Addr},
};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::etherparser::MacAddr;

/**
 * +---------- ----+- -------------+---------------------------+
 * |     Type      |     Length    |      OXM Fields           |
 * +----- ---------+--- -----------+---------------------------+
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
    }
}

#[derive(Clone)]
#[repr(u16)]
pub enum MatchType {
    Standard = 0,
    OXM = 1, //, the OpenFlow 1.1 match type OFPMT_STANDARD is deprecated
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
    pub fn new(field: OxmMatchFields, size: u8) -> Self {
        Self {
            class: OxmClass::OpenflowBasic,
            field,
            hasmask: false,
            length: 4 + size,
            experimenter: None,
        }
    }
    pub fn marshal(&self, bytes: &mut Vec<u8>) {
        bytes.write_u16::<BigEndian>(self.class.clone().into());
        let field: u8 = self.field.clone().into();
        bytes.write_u8(field << 1 | if self.hasmask { 1 } else { 0 });
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

impl From<u8> for OxmMatchFields {
    fn from(value: u8) -> Self {
        if value < 44 {
            unsafe { transmute(value) }
        } else {
            Self::ActsetOutput
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
        let mut ofp_match = OfpMatch::new();
        let ofp_byte = ofp_match.oxm_fields.as_mut();

        if let Some(in_port) = &self.in_port {
            let header = OxmHeader::new(OxmMatchFields::InPort, 4);
            header.marshal(ofp_byte);
            ofp_byte.write_u32::<BigEndian>(*in_port);
        }
        if let Some(eth_dst) = &self.eth_dst {
            let header = OxmHeader::new(OxmMatchFields::MacDest, 6);
            header.marshal(ofp_byte);
            eth_dst.marshal(ofp_byte);
        }
        if let Some(eth_src) = &self.eth_src {
            let header = OxmHeader::new(OxmMatchFields::MacSrc, 6);
            header.marshal(ofp_byte);
            eth_src.marshal(ofp_byte);
        }
        if let Some(eth_typ) = &self.eth_typ {
            OxmHeader::new(OxmMatchFields::EthernetType, 2).marshal(ofp_byte);
            ofp_byte.write_u16::<BigEndian>(*eth_typ);
        }
        if let Some(ip_proto) = &self.ip_proto {
            OxmHeader::new(OxmMatchFields::Protocol, 1);
            ofp_byte.write_u8(*ip_proto);
        }
        if let Some(ipv4_src) = &self.ipv4_src {
            OxmHeader::new(OxmMatchFields::IpSrc, 4).marshal(ofp_byte);
            bytes.write_u32::<BigEndian>(ipv4_src.clone().into());
        }
        if let Some(ipv4_dst) = &self.ipv4_dst {
            OxmHeader::new(OxmMatchFields::IpDst, 4).marshal(ofp_byte);
            bytes.write_u32::<BigEndian>(ipv4_dst.clone().into());
        }
        if let Some(ipv6_src) = &self.ipv6_src {
            OxmHeader::new(OxmMatchFields::Ipv6Src, 16);
            ofp_byte.write_u128::<BigEndian>(ipv6_src.clone().into());
        }
        if let Some(ipv6_dst) = &self.ipv6_dst {
            OxmHeader::new(OxmMatchFields::Ipv6Dst, 16);
            ofp_byte.write_u128::<BigEndian>(ipv6_dst.clone().into());
        }
        if let Some(tcp_src) = &self.tcp_src {
            OxmHeader::new(OxmMatchFields::TcpSrc, 2);
            ofp_byte.write_u16::<BigEndian>(*tcp_src);
        }
        if let Some(tcp_dst) = &self.tcp_dst {
            OxmHeader::new(OxmMatchFields::TcpDst, 2);
            ofp_byte.write_u16::<BigEndian>(*tcp_dst);
        }
        if let Some(udp_src) = &self.udp_src {
            OxmHeader::new(OxmMatchFields::UdpSrc, 2);
            ofp_byte.write_u16::<BigEndian>(*udp_src);
        }
        if let Some(udp_dst) = &self.udp_dst {
            OxmHeader::new(OxmMatchFields::UdpDst, 2);
            ofp_byte.write_u16::<BigEndian>(*udp_dst);
        }
        ofp_match.marshal(bytes);
    }

    pub fn parse(bytes: &mut Cursor<Vec<u8>>) -> Result<MatchFields, Box<dyn Error>> {
        let mut matcher = MatchFields::match_all();

        let typ = bytes.read_u16::<BigEndian>()?;
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
                OxmMatchFields::MacDest => {
                    let mut mac = [0u8; 6];
                    for i in 0..6 {
                        mac[i] = bytes.read_u8()?;
                    }
                    if hash_mask {
                        bytes.consume(6);
                    }
                    matcher.eth_dst = Some(MacAddr::new(mac));
                }
                OxmMatchFields::MacSrc => {
                    let mut mac = [0u8; 6];
                    for i in 0..6 {
                        mac[i] = bytes.read_u8()?;
                    }
                    if hash_mask {
                        bytes.consume(6);
                    }
                    matcher.eth_src = Some(MacAddr::new(mac));
                }
                OxmMatchFields::EthernetType => {
                    let eth_typ = bytes.read_u16::<BigEndian>()?;
                    if hash_mask {
                        bytes.consume(2);
                    }
                    matcher.eth_typ = Some(eth_typ);
                }
                OxmMatchFields::Protocol => {
                    let proto = bytes.read_u8()?;
                    if hash_mask {
                        bytes.consume(1);
                    }
                    matcher.ip_proto = Some(proto);
                }
                OxmMatchFields::IpSrc => {
                    let ip = bytes.read_u32::<BigEndian>()?;
                    if hash_mask {
                        bytes.consume(4);
                    }
                    matcher.ipv4_src = Some(Ipv4Addr::from(ip));
                }
                OxmMatchFields::IpDst => {
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
                    bytes.consume((oxm_length - 4).into());
                }
            }
            pkt_len = pkt_len - (oxm_length as u16);
        }

        Ok(matcher)
    }
}
