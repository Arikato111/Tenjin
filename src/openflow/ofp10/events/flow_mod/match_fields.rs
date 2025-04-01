//! OpenFlow 1.0 Match Fields
//!
//! This module implements the match fields functionality for OpenFlow 1.0 flow entries.
//! Match fields define the criteria used to match packets against flow entries in the
//! switch's flow tables.
//!
//! The module provides:
//! - Match field structure definitions
//! - Wildcard handling for match fields
//! - IP address masking
//! - Serialization/deserialization of match fields

use std::io::{BufRead, Cursor, Error};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::utils::{
    value_converter::{bit_bool, set_bit},
    MacAddr,
};

/// Represents an IP address with an optional mask
///
/// Used for matching source and destination IP addresses with subnet masks
pub struct Mask<T> {
    /// The IP address value
    pub ip: T,
    /// Optional subnet mask
    pub mask: Option<T>,
}

impl Mask<u32> {
    /// Converts the mask to an integer value
    ///
    /// # Returns
    /// The mask value as a u32, or 0 if no mask is specified
    pub fn to_int(&self) -> u32 {
        match self.mask {
            Some(v) => v,
            None => 0,
        }
    }
}

/// Represents wildcard flags for match fields
///
/// Used to specify which match fields should be ignored when matching packets
struct Wildcards {
    /// Ignore input port
    pub in_port: bool,
    /// Ignore destination MAC address
    pub mac_dest: bool,
    /// Ignore source MAC address
    pub mac_src: bool,
    /// Ignore Ethernet type
    pub ethernet_type: bool,

    /// Ignore VLAN ID
    pub vlan_vid: bool,
    /// Ignore VLAN priority
    pub vlan_pcp: bool,

    /// IP source address mask bits
    pub ip_src: u32,
    /// IP destination address mask bits
    pub ip_dest: u32,
    /// Ignore protocol
    pub protocol: bool,
    /// Ignore Type of Service
    pub tos: bool,
    /// Ignore transport source port
    pub transport_src: bool,
    /// Ignore transport destination port
    pub transport_dest: bool,
}

impl Wildcards {
    /// Creates wildcards from match fields
    ///
    /// # Arguments
    /// * `match_fields` - The match fields to create wildcards from
    ///
    /// # Returns
    /// A new Wildcards instance
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

    /// Parses wildcards from a byte value
    ///
    /// # Arguments
    /// * `byte` - The byte value containing wildcard flags
    ///
    /// # Returns
    /// A new Wildcards instance
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

    /// Serializes wildcards to a byte buffer
    ///
    /// # Arguments
    /// * `bytes` - Mutable reference to the byte buffer to write to
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

    /// Gets network mask bits from a byte value
    ///
    /// # Arguments
    /// * `f` - The byte value to extract mask bits from
    /// * `offset` - The bit offset to start from
    ///
    /// # Returns
    /// The mask bits as a u32
    pub fn get_nw_mask(f: u32, offset: usize) -> u32 {
        (f >> offset) & 0x3f
    }

    /// Sets network mask bits in a byte value
    ///
    /// # Arguments
    /// * `byte` - The byte value to modify
    /// * `offset` - The bit offset to set mask bits at
    /// * `set` - The mask bits to set
    ///
    /// # Returns
    /// The modified byte value
    pub fn set_nw_mask(byte: u32, offset: usize, set: u32) -> u32 {
        let value = (0x3f & set) << offset;
        byte | value
    }

    /// Converts a mask to its bit representation
    ///
    /// # Arguments
    /// * `mask` - The mask to convert
    ///
    /// # Returns
    /// The mask bits as a u32
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

/// Represents the match fields for an OpenFlow flow entry
///
/// Contains all possible match criteria that can be used to match packets
/// against flow entries in the switch's flow tables.
pub struct MatchFields {
    /// Input port to match
    pub in_port: Option<u16>,
    /// Destination MAC address to match
    pub mac_dest: Option<MacAddr>,
    /// Source MAC address to match
    pub mac_src: Option<MacAddr>,
    /// Ethernet type to match
    pub ethernet_type: Option<u16>,

    /// VLAN ID to match
    pub vlan_vid: Option<u16>,
    /// VLAN priority to match
    pub vlan_pcp: Option<u8>,

    /// Source IP address and mask to match
    pub ip_src: Option<Mask<u32>>,
    /// Destination IP address and mask to match
    pub ip_dest: Option<Mask<u32>>,
    /// Protocol to match
    pub protocol: Option<u8>,
    /// Type of Service to match
    pub tos: Option<u8>,
    /// Transport source port to match
    pub transport_src: Option<u16>,
    /// Transport destination port to match
    pub transport_dest: Option<u16>,
}

impl MatchFields {
    /// Creates a match fields instance that matches all packets
    ///
    /// # Returns
    /// A new MatchFields instance with all fields set to None
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

    /// Serializes match fields to a byte buffer
    ///
    /// # Arguments
    /// * `bytes` - Mutable reference to the byte buffer to write to
    pub fn marshal(&self, bytes: &mut Vec<u8>) {
        let wildcard = Wildcards::from_match_fields(self);
        wildcard.marshal(bytes);
        let _ = bytes.write_u16::<BigEndian>(match self.in_port {
            Some(p) => p,
            None => 0,
        });
        match &self.mac_src {
            Some(mac) => mac.marshal(bytes),
            None => MacAddr::from(0).marshal(bytes),
        }
        match &self.mac_dest {
            Some(mac) => mac.marshal(bytes),
            None => MacAddr::from(0).marshal(bytes),
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

    /// Parses match fields from a byte buffer
    ///
    /// # Arguments
    /// * `bytes` - Cursor containing the byte buffer to parse
    ///
    /// # Returns
    /// Result containing either the parsed MatchFields or an error
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
            Some(MacAddr::new(arr))
        };
        let mac_dest = if wildcards.mac_dest {
            None
        } else {
            let mut arr: [u8; 6] = [0; 6];
            for i in 0..6 {
                arr[i] = bytes.read_u8()?;
            }
            Some(MacAddr::new(arr))
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
