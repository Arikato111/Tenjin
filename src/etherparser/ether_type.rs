pub enum EtherType {
    IP = 0x0800,
    ARP = 0x0806,
    TEB = 0x6558,
    VLAN = 0x8100,
    IPV6 = 0x86dd,
    SLOW = 0x8809,
    MPLS = 0x8847,
    SVLAN = 0x88a8,
    LLDP = 0x88cc,
    PBB = 0x88e7,
    IEEE802_3 = 0x05dc,
    CFM = 0x8902,
    NSH = 0x894f,
    Unparsable = 0xffff,
}

impl EtherType {
    pub fn parse(ether: u16) -> EtherType {
        match ether {
            tp if tp == EtherType::IP as u16 => EtherType::IP,
            tp if tp == EtherType::ARP as u16 => EtherType::ARP,
            tp if tp == EtherType::TEB as u16 => EtherType::TEB,
            tp if tp == EtherType::VLAN as u16 => EtherType::VLAN,
            tp if tp == EtherType::IPV6 as u16 => EtherType::IPV6,
            tp if tp == EtherType::SLOW as u16 => EtherType::SLOW,
            tp if tp == EtherType::MPLS as u16 => EtherType::MPLS,
            tp if tp == EtherType::SVLAN as u16 => EtherType::SVLAN,
            tp if tp == EtherType::LLDP as u16 => EtherType::LLDP,
            tp if tp == EtherType::PBB as u16 => EtherType::PBB,
            tp if tp == EtherType::IEEE802_3 as u16 => EtherType::IEEE802_3,
            tp if tp == EtherType::CFM as u16 => EtherType::CFM,
            tp if tp == EtherType::NSH as u16 => EtherType::NSH,
            _ => EtherType::Unparsable,
        }
    }
}
