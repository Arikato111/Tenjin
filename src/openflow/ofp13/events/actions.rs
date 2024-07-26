use crate::{etherparser::MacAddr, openflow::ofp13::PseudoPort};
use byteorder::{BigEndian, WriteBytesExt};
use std::net::{Ipv4Addr, Ipv6Addr};

use super::flow_mod::{
    instructions::InstructActions,
    match_fields::{OxmHeader, OxmMatchFields},
};

#[derive(Clone)]
#[repr(u16)]
enum ActionType {
    Output = 0,      // Output to switch port.
    CopyTtlOut = 11, // Copy TTL "outwards" -- from next-to-outermost to outermost
    CopyTtlIn = 12,  // Copy TTL "inwards" -- from outermost to next-to-outermost
    SetMplsTtl = 15, // MPLS TTL

    DecMplsTtl = 16, // Decrement MPLS TTL
    PushVlan = 17,   // Push a new VLAN tag
    PopVlan = 18,    // Pop the outer VLAN tag
    PushMpls = 19,   // Push a new MPLS tag
    PopMpls = 20,    // Pop the outer MPLS tag
    SetQueue = 21,   // Set queue id when outputting to a port
    Group = 22,      // Apply group.
    SetNwTtl = 23,   // IP TTL.
    DecNwTtl = 24,   // Decrement IP TTL.
    SetField = 25,   // Set a header field using OXM TLV format.
    PushPbb = 26,    // Push a new PBB service tag (I-TAG)
    PopPbb = 27,     // Pop the outer PBB service tag (I-TAG)
    Experimenter = 0xffff,
}

impl ActionType {
    pub fn marshal(&self, bytes: &mut Vec<u8>) {
        bytes.write_u16::<BigEndian>(self.clone().into());
    }
}

impl From<ActionType> for u16 {
    fn from(value: ActionType) -> Self {
        value as u16
    }
}

#[derive(Clone)]
#[repr(u16)]
enum ControllerMaxLen {
    Max = 0xffe5,
    NoBuffer = 0xffff,
}

impl From<ControllerMaxLen> for u16 {
    fn from(value: ControllerMaxLen) -> Self {
        value as u16
    }
}

struct ActionOutput {
    typ: ActionType,  // u16
    port: PseudoPort, // u32
    max_len: ControllerMaxLen,
}

impl ActionOutput {
    pub const LEN: usize = 16;
    pub fn marshal(&self, bytes: &mut Vec<u8>) {
        self.typ.marshal(bytes);
        self.port.marshal(bytes);
        bytes.write_u16::<BigEndian>(self.max_len.clone().into());
        // write padding 48 bytes [32 + 16]
        bytes.write_u32::<BigEndian>(0);
        bytes.write_u16::<BigEndian>(0);
    }
    pub fn new(port: PseudoPort, max_len: ControllerMaxLen) -> Self {
        Self {
            typ: ActionType::Output,
            port,
            max_len,
        }
    }
}

#[derive(Clone)]
pub enum SetField {
    InPort(PseudoPort), // Ingress port. This may be a physical or switch-defined logical port.
    EthDst(MacAddr),    // Ethernet source address. Can use arbitrary bitmask
    EthSrc(MacAddr),    // Ethernet destination address. Can use arbitrary bitmask
    EthTyp(u16),        // Ethernet type of the OpenFlow packet payload, after VLAN tags.
    IpProto(u8),        // IPv4 or IPv6 protocol number
    Ipv4Src(Ipv4Addr),  // IPv4 source address. Can use subnet mask or arbitrary bitmask
    Ipv4Dst(Ipv4Addr),  // IPv4 destination address. Can use subnet mask or arbitrary bitmask
    Ipv6Src(Ipv6Addr),  // IPv6 source address. Can use subnet mask or arbitrary bitmask
    Ipv6Dst(Ipv6Addr),  // IPv6 destination address. Can use subnet mask or arbitrary bitmask
    TcpSrc(u16),        // TCP source port
    TcpDst(u16),        // TCP destination port
    UdpSrc(u16),        // UDP source port
    UdpDst(u16),        // UDP destination port
}

impl SetField {
    pub fn marshal(&self, bytes: &mut Vec<u8>) {
        match &self {
            SetField::InPort(port) => {
                OxmHeader::new(OxmMatchFields::ActsetOutput, 4).marshal(bytes);
                port.marshal(bytes);
            }
            SetField::EthDst(mac) => {
                OxmHeader::new(OxmMatchFields::MacDest, 6).marshal(bytes);
                mac.marshal(bytes);
            }
            SetField::EthSrc(mac) => {
                OxmHeader::new(OxmMatchFields::MacSrc, 6);
                mac.marshal(bytes);
            }
            SetField::EthTyp(eth) => {
                OxmHeader::new(OxmMatchFields::EthernetType, 2).marshal(bytes);
                bytes.write_u16::<BigEndian>(*eth);
            }
            SetField::IpProto(proto) => {
                OxmHeader::new(OxmMatchFields::Protocol, 1).marshal(bytes);
                bytes.write_u8(*proto);
            }
            SetField::Ipv4Src(ipv4) => {
                OxmHeader::new(OxmMatchFields::IpSrc, 4).marshal(bytes);
                bytes.write_u32::<BigEndian>(ipv4.clone().into());
            }
            SetField::Ipv4Dst(ipv4) => {
                OxmHeader::new(OxmMatchFields::IpDst, 4).marshal(bytes);
                bytes.write_u32::<BigEndian>(ipv4.clone().into());
            }
            SetField::Ipv6Src(ipv6) => {
                OxmHeader::new(OxmMatchFields::Ipv6Src, 16).marshal(bytes);
                bytes.write_u128::<BigEndian>(ipv6.clone().into());
            }
            SetField::Ipv6Dst(ipv6) => {
                OxmHeader::new(OxmMatchFields::Ipv6Dst, 16).marshal(bytes);
                bytes.write_u128::<BigEndian>(ipv6.clone().into());
            }
            SetField::TcpSrc(tcp) => {
                OxmHeader::new(OxmMatchFields::TcpSrc, 2).marshal(bytes);
                bytes.write_u16::<BigEndian>(*tcp);
            }
            SetField::TcpDst(tcp) => {
                OxmHeader::new(OxmMatchFields::TcpDst, 2).marshal(bytes);
                bytes.write_u16::<BigEndian>(*tcp);
            }
            SetField::UdpSrc(udp) => {
                OxmHeader::new(OxmMatchFields::UdpSrc, 2).marshal(bytes);
                bytes.write_u16::<BigEndian>(*udp);
            }
            SetField::UdpDst(udp) => {
                OxmHeader::new(OxmMatchFields::UdpDst, 2).marshal(bytes);
                bytes.write_u16::<BigEndian>(*udp);
            }
        }
    }
}

pub type Buffer = u16;
#[derive(Clone)]
#[repr(u8)]
pub enum Action {
    Oputput(PseudoPort),
    CopyTtlOut,     // Copy TTL "outwards" -- from next-to-outermost to outermost
    CopyTtlIn,      // Copy TTL "inwards" -- from outermost to next-to-outermost
    SetMplsTtl(u8), // MPLS TTL

    DecMplsTtl, // Decrement MPLS TTL
    PushVlan(u16),
    PushMpls(u16),
    PushPbb(u16),

    PopVlan(u16),
    PopMpls(u16),
    PopPbb(u16),

    SetQueue(u32),      // Set queue id when outputting to a port
    Group(u32),         // Apply group.
    SetNwTtl(u8),       // IP TTL.
    DecNwTtl = 24,      // Decrement IP TTL.
    SetField(SetField), // Set a header field using OXM TLV format.
    Experimenter(u32),
}

impl Action {
    pub fn action_type(&self) -> ActionType {
        match &self {
            Action::Oputput(_) => ActionType::Output,
            Action::CopyTtlOut => ActionType::CopyTtlOut,
            Action::CopyTtlIn => ActionType::CopyTtlIn,
            Action::SetMplsTtl(_) => ActionType::SetMplsTtl,
            Action::DecMplsTtl => ActionType::DecMplsTtl,
            Action::PushVlan(_) => ActionType::PushVlan,
            Action::PushMpls(_) => ActionType::PushMpls,
            Action::PushPbb(_) => ActionType::PushPbb,
            Action::PopVlan(_) => ActionType::PopVlan,
            Action::PopMpls(_) => ActionType::PopMpls,
            Action::PopPbb(_) => ActionType::PopPbb,
            Action::SetQueue(_) => ActionType::SetQueue,
            Action::Group(_) => ActionType::Group,
            Action::SetNwTtl(_) => ActionType::SetNwTtl,
            Action::DecNwTtl => ActionType::DecNwTtl,
            Action::SetField(_) => ActionType::SetField,
            Action::Experimenter(_) => ActionType::Experimenter,
        }
    }
    pub fn marshal(&self, bytes: &mut Vec<u8>) {
        match &self {
            Action::Oputput(port) => {
                self.action_type().marshal(bytes);
                bytes.write_u16::<BigEndian>(16); // len
                port.marshal(bytes);
                bytes.write_u16::<BigEndian>(ControllerMaxLen::NoBuffer.into());
                // padding 48bit
                bytes.write_u32::<BigEndian>(0);
                bytes.write_u16::<BigEndian>(0);
            }
            Action::SetMplsTtl(mpls_ttl) => {
                self.action_type().marshal(bytes);
                bytes.write_u16::<BigEndian>(8);
                bytes.write_u8(*mpls_ttl);
                // padding 24bit
                bytes.write_u16::<BigEndian>(0);
                bytes.write_u8(0);
            }
            Action::PushVlan(ethertype)
            | Action::PushMpls(ethertype)
            | Action::PushPbb(ethertype) => {
                self.action_type().marshal(bytes);
                bytes.write_u16::<BigEndian>(8);
                bytes.write_u16::<BigEndian>(*ethertype);
                // padding 16 bit
                bytes.write_u16::<BigEndian>(0);
            }
            Action::PopVlan(ethertype) | Action::PopMpls(ethertype) | Action::PopPbb(ethertype) => {
                self.action_type().marshal(bytes);
                bytes.write_u16::<BigEndian>(8);
                bytes.write_u16::<BigEndian>(*ethertype);
                bytes.write_u16::<BigEndian>(0);
            }
            Action::SetQueue(queue_id) => {
                self.action_type().marshal(bytes);
                bytes.write_u16::<BigEndian>(8);
                bytes.write_u32::<BigEndian>(*queue_id);
            }
            Action::Group(group_id) => {
                self.action_type().marshal(bytes);
                bytes.write_u16::<BigEndian>(8);
                bytes.write_u32::<BigEndian>(*group_id);
            }
            Action::SetNwTtl(nw_ttl) => {
                self.action_type().marshal(bytes);
                bytes.write_u16::<BigEndian>(8);
                bytes.write_u8(*nw_ttl);
                // padding 24bit
                bytes.write_u16::<BigEndian>(0);
                bytes.write_u8(0);
            }
            Action::SetField(omx_field) => {
                let mut field_bytes: Vec<u8> = Vec::new();
                omx_field.marshal(&mut field_bytes);

                self.action_type().marshal(bytes);
                bytes.write_u16::<BigEndian>(4 + field_bytes.len() as u16);
                bytes.append(&mut field_bytes);
            }
            Action::Experimenter(exper_id) => {
                self.action_type().marshal(bytes);
                bytes.write_u16::<BigEndian>(8);
                bytes.write_u32::<BigEndian>(*exper_id);
            }
            Action::DecMplsTtl | Action::DecNwTtl | Action::CopyTtlOut | Action::CopyTtlIn => {
                self.action_type().marshal(bytes);
                bytes.write_u16::<BigEndian>(8);
                // padding 32 bit
                bytes.write_u32::<BigEndian>(0);
            }
        }
    }

    // TODO
    // pub fn parse_sequence(bytes: &mut Cursor<Vec<u8>>) -> Vec<Action> {
    //     if bytes.get_ref().is_empty() {
    //         vec![]
    //     } else {
    //         if let Ok(action) = Action::parse(bytes) {
    //             let mut v = vec![action];
    //             v.append(&mut Action::parse_sequence(bytes));
    //             v
    //         } else {
    //             vec![]
    //         }
    //     }
    // }

    // TODO
    // pub fn parse(bytes: &mut Cursor<Vec<u8>>) -> Result<Action, Error> {
    // }
}

pub trait ToInstruction {
    fn to_instruct(&self) -> InstructActions;
}

impl ToInstruction for Vec<Action> {
    fn to_instruct(&self) -> InstructActions {
        let mut instruct = InstructActions::new(InstructActions::APPLY);
        instruct.actions.append(&mut self.clone());
        instruct
    }
}
