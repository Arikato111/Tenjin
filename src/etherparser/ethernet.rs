use crate::etherparser::ether_type::EtherType;
use std::io::{BufRead, Cursor, Error};

use byteorder::{BigEndian, ReadBytesExt};

use super::{
    packet::{ARP, IP},
    tools::bits::mac_to_bytes,
};

pub struct EthernetFrame {
    pub ether_type: EtherType,
    pub mac_dst: u64,
    pub mac_src: u64,
    pub vlan_pcp: u8,
    pub vlan_dei: bool,
    pub vlan_vid: Option<u16>,
    pub network: Network,
}

impl EthernetFrame {
    pub fn mac_dst_string(&self) -> String {
        Self::mac_str(self.mac_dst)
    }
    pub fn mac_src_string(&self) -> String {
        Self::mac_str(self.mac_src)
    }
    pub fn mac_str(mac: u64) -> String {
        let mut mac_string = String::new();
        let mut mac = mac;
        for _ in 0..6 {
            mac_string = format!("{:02x}:{}", mac as u8, mac_string);
            mac = mac >> 8;
        }
        mac_string.pop();
        mac_string
    }
    pub fn parse(payload: &Vec<u8>) -> Result<EthernetFrame, Error> {
        let mut bytes = Cursor::new(payload.to_vec());
        let mut mac_dst = [0u8; 6];
        let mut mac_src = [0u8; 6];
        for i in 0..6 {
            mac_dst[i] = bytes.read_u8()?;
        }
        for i in 0..6 {
            mac_src[i] = bytes.read_u8()?;
        }
        // check 802.1q tag tpid
        let typ = bytes.read_u16::<BigEndian>()?;
        let (vlan_pcp, vlan_dei, vlan_vid, typ) = match typ {
            tp if tp == EtherType::VLAN as u16 => {
                let tci = bytes.read_u16::<BigEndian>()?;
                let pcp = tci >> 13;
                let dei = (tci & 0x1000) > 0;
                let vid = tci & 0xfff;
                (pcp as u8, dei, Some(vid), typ)
            }
            _ => (0x0, false, None, typ),
        };
        let network = match typ {
            tp if tp == EtherType::IP as u16 => {
                let ip = IP::parse(&mut bytes);
                match ip {
                    Ok(ip) => Network::IP(ip),
                    Err(_) => Network::Unparsable(typ, bytes.fill_buf()?.to_vec()),
                }
            }
            tp if tp == (EtherType::ARP as u16) => {
                let arp = ARP::parse(&mut bytes);
                match arp {
                    Ok(arp) => Network::ARP(arp),
                    Err(_) => Network::Unparsable(typ, bytes.fill_buf()?.to_vec()),
                }
            }
            _ => Network::Unparsable(typ, bytes.fill_buf()?.to_vec()),
        };
        Ok(EthernetFrame {
            ether_type: EtherType::parse(typ),
            mac_dst: mac_to_bytes(mac_dst),
            mac_src: mac_to_bytes(mac_src),
            vlan_pcp,
            vlan_dei,
            vlan_vid,
            network,
        })
    }
}

pub enum Network {
    IP(IP),
    ARP(ARP),
    Unparsable(u16, Vec<u8>),
}

impl Network {
    pub fn get_ip(&self) -> Option<IP> {
        match self {
            Network::IP(ip) => Some(ip.clone()),
            _ => None,
        }
    }
    pub fn get_arp(&self) -> Option<ARP> {
        if let Network::ARP(arp) = self {
            Some(arp.clone())
        } else {
            None
        }
    }
}
