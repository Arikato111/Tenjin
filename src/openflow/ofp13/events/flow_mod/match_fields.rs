//! OpenFlow v1.3 Flow Modification Match Fields
//!
//! This module defines the match fields used to match packets in flow entries.
//! It implements the OpenFlow Extensible Match (OXM) format for flexible packet matching.

use std::{
    io::{BufRead, Cursor, Error},
    mem::transmute,
    net::{Ipv4Addr, Ipv6Addr},
};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::utils::MacAddr;

/// OpenFlow match structure
///
/// # Format
///
/// |     Type      |     Length    |      OXM Fields           |
/// |---------------|---------------|---------------------------|
/// |  (Optional)   |  (16 bits)    | (Array of variable length)|
///
pub struct OfpMatch {
    /// Type of match
    typ: MatchType,
    /// Length of match in bytes
    length: u16,
    /// OXM fields for matching
    oxm_fields: Vec<u8>,
}

impl OfpMatch {
    /// Creates a new OpenFlow match structure
    ///
    /// # Returns
    /// * `OfpMatch` - The new match structure
    pub fn new() -> Self {
        Self {
            typ: MatchType::OXM,
            length: 4,
            oxm_fields: Vec::new(),
        }
    }
    /// Marshals the match structure into a byte buffer
    ///
    /// # Arguments
    /// * `bytes` - The buffer to write the match to
    ///
    /// # Returns
    /// * `Result<(), Error>` - Success or error status
    pub fn marshal(&self, bytes: &mut Vec<u8>) -> Result<(), Error> {
        bytes.write_u16::<BigEndian>(self.typ.clone().into())?;
        bytes.write_u16::<BigEndian>(self.length + (self.oxm_fields.len() as u16))?;
        bytes.append(&mut self.oxm_fields.clone());
        // padding
        bytes.write_u32::<BigEndian>(0)?;
        Ok(())
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

/// OXM header structure
///
/// ## Format
///
/// |        OXM Header          |       OXM Body             |
/// |----------------------------|----------------------------|
/// | Version (1 byte)           |  Value (variable length)   |
/// | OXM Type (1 byte)          |  Mask   (variable length)  |
/// | Length (2 bytes)           |                            |
/// | OXM ID (4 bytes) (Optional)|                            |
///
#[allow(unused)]
pub struct OxmHeader {
    /// Match class (member class or reserved class)
    class: OxmClass, // Match class: member class or reserved class
    /// Match field within the class
    field: OxmMatchFields, // 7bit Match field within the class
    /// Whether OXM includes a bitmask in payload
    hasmask: bool, // 1bit Set if OXM include a bitmask in payload
    /// Length of OXM payload
    length: u8, // Length of OXM payload
    /// Optional experimenter ID
    experimenter: Option<u32>,
}

impl OxmHeader {
    /// Creates a new OXM header
    ///
    /// # Arguments
    /// * `field` - Match field within the class
    /// * `size` - Length of OXM payload
    /// * `hasmask` - Whether to include a bitmask
    ///
    /// # Returns
    /// * `OxmHeader` - The new header instance
    pub fn new(field: OxmMatchFields, size: u8, hasmask: bool) -> Self {
        Self {
            class: OxmClass::OpenflowBasic,
            field,
            hasmask: hasmask,
            length: size,
            experimenter: None,
        }
    }
    /// Marshals the OXM header into a byte buffer
    ///
    /// # Arguments
    /// * `bytes` - The buffer to write the header to
    ///
    /// # Returns
    /// * `Result<(), Error>` - Success or error status
    pub fn marshal(&self, bytes: &mut Vec<u8>) -> Result<(), Error> {
        bytes.write_u16::<BigEndian>(self.class.clone().into())?;
        let field: u8 = self.field.clone().into();
        bytes.write_u8(field << 1 | if self.hasmask { 1 } else { 0 })?;
        bytes.write_u8(self.length)?;
        // if let Some(exp) = self.experimenter {
        // bytes.write_u32::<BigEndian>(exp);
        // }
        Ok(())
    }
}

/**
 * NXM allocates only two vendors,
 * 0x0000 for fields supported by OpenFlow 1.0
 * and 0x0001 for fields implemented as an Open vSwitch extension
 */

/// OXM class types
///
/// NXM allocates only two vendors:
/// - 0x0000 for fields supported by OpenFlow 1.0
/// - 0x0001 for fields implemented as an Open vSwitch extension
#[derive(Clone)]
#[repr(u16)]
pub enum OxmClass {
    /// Backward compatibility with NXM
    Nxm0 = 0x0000, // Backward compatibility with NXM
    /// Backward compatibility with NXM
    Nxm1 = 0x0001, // Backward compatibility with NXM
    /// Basic class for OpenFlow
    OpenflowBasic = 0x8000, // Basic class for OpenFlow
    /// Experimenter class
    Experimenter = 0xffff, // Experimenter class
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
    /// Switch input port
    InPort = 0, /* Switch input port. */
    /// Switch physical input port
    InPhyPort = 1, /* Switch physical input port. */
    /// Metadata passed between tables
    METADATA = 2, /* Metadata passed between tables. */
    /// Ethernet destination address
    EthDst = 3, /* Ethernet destination address. */
    /// Ethernet source address
    EthSrc = 4, /* Ethernet source address. */
    /// Ethernet frame type
    EthType = 5, /* Ethernet frame type. */
    /// VLAN id
    VlanVid = 6, /* VLAN id. */
    /// VLAN priority
    VlanPcp = 7, /* VLAN priority. */
    /// IP DSCP (6 bits in ToS field)
    IpDscp = 8, /* IP DSCP (6 bits in ToS field). */
    /// IP ECN (2 bits in ToS field)
    IpEcn = 9, /* IP ECN (2 bits in ToS field). */
    /// IP protocol
    IpProto = 10, /* IP protocol. */
    /// IPv4 source address
    Ipv4Src = 11, /* IPv4 source address. */
    /// IPv4 destination address
    Ipv4Dst = 12, /* IPv4 destination address. */
    /// TCP source port
    TcpSrc = 13, /* TCP source port. */
    /// TCP destination port
    TcpDst = 14, /* TCP destination port. */
    /// UDP source port
    UdpSrc = 15, /* UDP source port. */
    /// UDP destination port
    UdpDst = 16, /* UDP destination port. */
    /// SCTP source port
    SctpSrc = 17, /* SCTP source port. */
    /// SCTP destination port
    SctpDst = 18, /* SCTP destination port. */
    /// ICMP type
    Icmpv4Type = 19, /* ICMP type. */
    /// ICMP code
    Icmpv4Code = 20, /* ICMP code. */
    /// ARP opcode
    ArpOp = 21, /* ARP opcode. */
    /// ARP source IPv4 address
    ArpSpa = 22, /* ARP source IPv4 address. */
    /// ARP target IPv4 address
    ArpTpa = 23, /* ARP target IPv4 address. */
    /// ARP source hardware address
    ArpSha = 24, /* ARP source hardware address. */
    /// ARP target hardware address
    ArpTha = 25, /* ARP target hardware address. */
    /// IPv6 source address
    Ipv6Src = 26, /* IPv6 source address. */
    /// IPv6 destination address
    Ipv6Dst = 27, /* IPv6 destination address. */
    /// IPv6 Flow Label
    Ipv6Flabel = 28, /* IPv6 Flow Label */
    /// ICMPv6 type
    Icmpv6Type = 29, /* ICMPv6 type. */
    /// ICMPv6 code
    Icmpv6Code = 30, /* ICMPv6 code. */
    /// Target address for ND
    Ipv6NdTarget = 31, /* Target address for ND. */
    /// Source link-layer for ND
    Ipv6NdSll = 32, /* Source link-layer for ND. */
    /// Target link-layer for ND
    Ipv6NdTll = 33, /* Target link-layer for ND. */
    /// MPLS label
    MplsLabel = 34, /* MPLS label. */
    /// MPLS TC
    MplsTc = 35, /* MPLS TC. */
    /// MPLS BoS bit
    MplsBos = 36, /* MPLS BoS bit. */
    /// PBB I-SID
    PbbIsid = 37, /* PBB I-SID. */
    /// Logical Port Metadata
    TunnelId = 38, /* Logical Port Metadata. */
    /// IPv6 Extension Header pseudo-field
    Ipv6Exthdr = 39, /* IPv6 Extension Header pseudo-field */
    /// Unparseable field
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
/// Required match fields for flow entries
pub struct MatchFields {
    /// Ingress port (physical or logical)
    pub in_port: Option<u32>, // Ingress port. This may be a physical or switch-defined logical port.
    /// Ethernet destination address with optional bitmask
    pub eth_dst: Option<MacAddr>, // Ethernet source address. Can use arbitrary bitmask
    /// Ethernet source address with optional bitmask
    pub eth_src: Option<MacAddr>, // Ethernet destination address. Can use arbitrary bitmask
    /// Ethernet type of the OpenFlow packet payload, after VLAN tags
    pub eth_typ: Option<u16>, // Ethernet type of the OpenFlow packet payload, after VLAN tags.
    /// IPv4 or IPv6 protocol number
    pub ip_proto: Option<u8>, // IPv4 or IPv6 protocol number
    /// IPv4 source address with optional mask
    pub ipv4_src: Option<Ipv4Addr>, // IPv4 source address. Can use subnet mask or arbitrary bitmask
    /// IPv4 destination address with optional mask
    pub ipv4_dst: Option<Ipv4Addr>, // IPv4 destination address. Can use subnet mask or arbitrary bitmask
    /// IPv6 source address with optional mask
    pub ipv6_src: Option<Ipv6Addr>, // IPv6 source address. Can use subnet mask or arbitrary bitmask
    /// IPv6 destination address with optional mask
    pub ipv6_dst: Option<Ipv6Addr>, // IPv6 destination address. Can use subnet mask or arbitrary bitmask
    /// TCP source port
    pub tcp_src: Option<u16>, // TCP source port
    /// TCP destination port
    pub tcp_dst: Option<u16>, // TCP destination port
    /// UDP source port
    pub udp_src: Option<u16>, // UDP source port
    /// UDP destination port
    pub udp_dst: Option<u16>, // UDP destination port
}

impl MatchFields {
    /// Creates a new match fields structure with no fields set
    ///
    /// # Returns
    /// * `MatchFields` - The new match fields instance
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
    /// Marshals the match fields into a byte buffer
    ///
    /// # Arguments
    /// * `bytes` - The buffer to write the fields to
    ///
    /// # Returns
    /// * `Result<(), Error>` - Success or error status
    pub fn marshal(&self, bytes: &mut Vec<u8>) -> Result<(), Error> {
        let mut ofp_match = OfpMatch::new();
        let mut ofp_byte = ofp_match.oxm_fields.as_mut();

        if let Some(in_port) = &self.in_port {
            let header = OxmHeader::new(OxmMatchFields::InPort, 4, false);
            header.marshal(&mut ofp_byte)?;
            ofp_byte.write_u32::<BigEndian>(*in_port)?;
        }
        if let Some(eth_dst) = &self.eth_dst {
            let header = OxmHeader::new(OxmMatchFields::EthDst, 12, true);
            header.marshal(&mut ofp_byte)?;
            eth_dst.marshal(&mut ofp_byte);
            // mac mask
            MacAddr::from(!0).marshal(&mut ofp_byte);
        }
        if let Some(eth_src) = &self.eth_src {
            let header = OxmHeader::new(OxmMatchFields::EthSrc, 12, true);
            header.marshal(&mut ofp_byte)?;
            eth_src.marshal(&mut ofp_byte);
            // mac mask
            MacAddr::from(!0).marshal(&mut ofp_byte);
        }
        if let Some(eth_typ) = &self.eth_typ {
            OxmHeader::new(OxmMatchFields::EthType, 2, false).marshal(&mut ofp_byte)?;
            ofp_byte.write_u16::<BigEndian>(*eth_typ)?;
        }
        if let Some(ip_proto) = &self.ip_proto {
            OxmHeader::new(OxmMatchFields::IpProto, 1, false).marshal(&mut ofp_byte)?;
            ofp_byte.write_u8(*ip_proto)?;
        }
        if let Some(ipv4_src) = &self.ipv4_src {
            OxmHeader::new(OxmMatchFields::Ipv4Src, 8, true).marshal(&mut ofp_byte)?;
            bytes.write_u32::<BigEndian>(ipv4_src.clone().into())?;
            bytes.write_u32::<BigEndian>(!0)?;
        }
        if let Some(ipv4_dst) = &self.ipv4_dst {
            OxmHeader::new(OxmMatchFields::Ipv4Dst, 8, true).marshal(&mut ofp_byte)?;
            bytes.write_u32::<BigEndian>(ipv4_dst.clone().into())?;
            bytes.write_u32::<BigEndian>(!0)?;
        }
        if let Some(ipv6_src) = &self.ipv6_src {
            OxmHeader::new(OxmMatchFields::Ipv6Src, 32, true).marshal(&mut ofp_byte)?;
            ofp_byte.write_u128::<BigEndian>(ipv6_src.clone().into())?;
            ofp_byte.write_u128::<BigEndian>(!0)?;
        }
        if let Some(ipv6_dst) = &self.ipv6_dst {
            OxmHeader::new(OxmMatchFields::Ipv6Dst, 32, true).marshal(&mut ofp_byte)?;
            ofp_byte.write_u128::<BigEndian>(ipv6_dst.clone().into())?;
            ofp_byte.write_u128::<BigEndian>(!0)?;
        }
        if let Some(tcp_src) = &self.tcp_src {
            OxmHeader::new(OxmMatchFields::TcpSrc, 2, false);
            ofp_byte.write_u16::<BigEndian>(*tcp_src)?;
        }
        if let Some(tcp_dst) = &self.tcp_dst {
            OxmHeader::new(OxmMatchFields::TcpDst, 2, false);
            ofp_byte.write_u16::<BigEndian>(*tcp_dst)?;
        }
        if let Some(udp_src) = &self.udp_src {
            OxmHeader::new(OxmMatchFields::UdpSrc, 2, false);
            ofp_byte.write_u16::<BigEndian>(*udp_src)?;
        }
        if let Some(udp_dst) = &self.udp_dst {
            OxmHeader::new(OxmMatchFields::UdpDst, 2, false);
            ofp_byte.write_u16::<BigEndian>(*udp_dst)?;
        }
        ofp_match.marshal(bytes)?;
        Ok(())
    }

    pub fn parse(bytes: &mut Cursor<Vec<u8>>) -> Result<MatchFields, Error> {
        let mut matcher = MatchFields::match_all();

        let _typ: MatchType = bytes.read_u16::<BigEndian>()?.into();
        let length = bytes.read_u16::<BigEndian>()?;
        let mut pkt_len = length - 4;
        while pkt_len > 0 {
            let _oxm_class = bytes.read_u16::<BigEndian>()?;
            let oxm_field = bytes.read_u8()?;
            let hash_mask = oxm_field & 1 == 1;
            let oxm_field: OxmMatchFields = (oxm_field >> 1).into();
            let oxm_length = bytes.read_u8()?;
            match oxm_field {
                OxmMatchFields::InPort => {
                    let port = bytes.read_u32::<BigEndian>()?;
                    let _mask = if hash_mask {
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
