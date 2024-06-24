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
    class: OxmClass, // Match class: member class or reserved class
    field: u8,       // 7bit Match field within the class
    hasmask: u8,     // 1bit Set if OXM include a bitmask in payload
    length: u8,      // Length of OXM payload
}

/**
 * NXM allocates only two vendors,
 * 0x0000 for fields supported by OpenFlow 1.0
 * and 0x0001 for fields implemented as an Open vSwitch extension
 */

#[repr(u16)]
pub enum OxmClass {
    Nxm0 = 0x0000,
    Nxm1 = 0x0001,
    OpenflowBasic = 0x8000,
    Experimenter = 0xffff,
}

/* OXM Flow match field types for OpenFlow basic class. */
pub struct MatchFields {
    pub in_port: Option<u32>,
    in_phy_port: Option<u32>,
    metadata: Option<i64>,
    pub mac_dest: Option<MacAddr>,
    pub mac_src: Option<MacAddr>,
    pub ethernet_type: Option<u16>,
    pub vlan_vid: Option<u16>, // vlan type
    pub vlan_pcp: Option<u8>,
    // ToS from IPv4 packet
    pub ip_dscp: Option<u8>, // IP DSCP (6 bits in ToS field).
    pub ip_ecn: Option<u8>,  // IP ECN (2 bits in ToS field).
    pub protocol: Option<u8>,
    pub ip_src: Option<Ipv4Addr>,
    pub ip_dst: Option<Ipv4Addr>,

    pub tcp_src: Option<u16>,
    pub tcp_dst: Option<u16>,
    pub udp_src: Option<u16>,
    pub udp_dst: Option<u16>,
    pub sctp_src: Option<u16>,
    pub sctp_dst: Option<u16>,

    pub icmpv4_type: Option<u8>,
    pub icmpv4_code: Option<u8>,
    pub arp_op: Option<u16>,
    pub arp_spa: Option<Ipv4Addr>,    // ARP source IPv4 address
    pub arp_tpa: Option<Ipv4Addr>,    // ARP target IPv4 address
    pub arp_sha: Option<MacAddr>,     // ARP source Mac
    pub arp_tha: Option<MacAddr>,     // ARP target Mac
    pub ipv6_src: Option<Ipv6Addr>,   // IPv6 address
    pub ipv6_dst: Option<Ipv6Addr>,   // IPv6 address
    pub ipv6_flabel: Option<u32>,     // IPv6 Flow Lable
    pub icmpv6_type: Option<u8>,      // ICMPv6 type
    pub icmpv6_code: Option<u8>,      // ICMPv6 code
    pub ipv6_nd_target: Option<u128>, // Target address for ND
    pub ipv6_nd_sll: Option<MacAddr>, // MAC , source link-layer for ND
    pub ipv6_nd_tll: Option<MacAddr>, // Mac , Target link-layer for ND
    pub mpls_label: Option<u32>,      // MPLS label
    pub mpls_tc: Option<u8>,          // MPLS TC
    pub mpls_bos: Option<u8>,         // MPLS Bos bit
    pub pbb_isid: Option<u32>,        // 24bit PBB I-SID
    pub tunnel_id: Option<u64>,       // Logical Port Metadata
    pub ipv6_exthdr: Option<u64>,     // IPv6 Extension Header pseudo-field
    pub pbb_uca: Option<u8>,          // PBB UCA Header
    pub tcp_flags: Option<u16>,       // TCP Flags
    pub actset_output: Option<u32>,   // Output port from action set metadata
}

impl MatchFields {
    pub fn match_all() -> Self {
        Self {
            in_port: None,
            in_phy_port: None,
            metadata: None,
            mac_dest: None,
            mac_src: None,
            ethernet_type: None,
            vlan_vid: None,
            vlan_pcp: None,
            ip_dscp: None,
            ip_ecn: None,
            protocol: None,
            ip_src: None,
            ip_dst: None,
            tcp_src: None,
            tcp_dst: None,
            udp_src: None,
            udp_dst: None,
            sctp_src: None,
            sctp_dst: None,
            icmpv4_type: None,
            icmpv4_code: None,
            arp_op: None,
            arp_spa: None,
            arp_tpa: None,
            arp_sha: None,
            arp_tha: None,
            ipv6_src: None,
            ipv6_dst: None,
            ipv6_flabel: None,
            icmpv6_type: None,
            icmpv6_code: None,
            ipv6_nd_target: None,
            ipv6_nd_sll: None,
            ipv6_nd_tll: None,
            mpls_label: None,
            mpls_tc: None,
            mpls_bos: None,
            pbb_isid: None,
            tunnel_id: None,
            ipv6_exthdr: None,
            pbb_uca: None,
            tcp_flags: None,
            actset_output: None,
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
