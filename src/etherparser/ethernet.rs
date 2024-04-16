use crate::etherparser::ether_type::EtherType;
use std::io::{BufRead, Cursor};

use byteorder::{BigEndian, ReadBytesExt};

use super::packet::IP;

struct EthernetFrame {
    mac_des: u64,
    mac_src: u64,
    vlan: Option<u16>,
    vlan_pcp: u8,
    valn_dei: bool,
    vlan_vid: u16,
}

impl EthernetFrame {
    fn parse(payload: &Vec<u8>) {
        let mut bytes = Cursor::new(*payload);
        let mut mac_des = vec![0u8; 6];
        let mut mac_src = vec![0u8; 6];
        for i in 0..6 {
            mac_des[i] = bytes.read_u8().unwrap();
        }
        for i in 0..6 {
            mac_src[i] = bytes.read_u8().unwrap();
        }
        // check 802.1q tag tpid
        let typ = bytes.read_u16::<BigEndian>().unwrap();
        let (pcp, dei, vid, typ) = match typ {
            tp if tp == EtherType::VLAN as u16 => {
                let tci = bytes.read_u16::<BigEndian>().unwrap();
                let pcp = tci >> 13;
                let dei = (tci & 0x1000) > 0;
                let vid = tci & 0xfff;
                (pcp as u8, dei, Some(vid), typ)
            }
            _ => (0x0, false, None, typ),
        };
        let ip_header = match typ {
            tp if tp == EtherType::IP as u16 => {
                let ip = IP::parse(&mut bytes);
                if ip.is_some() {
                    Network::IP(ip.unwrap())
                } else {
                    Network::Unparsable(typ, bytes.fill_buf().unwrap().to_vec())
                }
            }
            // tp if tp == (EtherType::ARP as u16) => {

            // }
            _ => Network::Unparsable(typ, bytes.fill_buf().unwrap().to_vec()),
        };
    }
}

pub enum Network {
    IP(IP),
    Unparsable(u16, Vec<u8>),
}
