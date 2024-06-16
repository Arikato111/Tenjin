use super::{tcp::TCP, udp::UDP, ICMP};
use byteorder::{BigEndian, ReadBytesExt};
use std::io::{BufRead, Cursor, Error, ErrorKind, Read};

#[derive(Clone)]
pub struct Flags {
    pub dont_flagment: bool,
    pub more_fragments: bool,
}

pub enum IpProtocol {
    ICMP = 0x01,
    TCP = 0x06,
    UDP = 0x11,
}

#[derive(Clone)]
pub struct IP {
    pub version: u8,
    pub ihl: u8,
    pub tos: u8,
    pub length: u16,
    pub ident: u16,
    pub flags: Flags,
    pub fragment: u16,
    pub ttl: u8,
    pub protocol: u8,
    pub checksum: u16,
    pub src: u32,
    pub dst: u32,
    pub options: Vec<u8>,
    pub ptcol: EtherData,
}

impl IP {
    pub fn parse(bytes: &mut Cursor<Vec<u8>>) -> Result<IP, Error> {
        if bytes.get_ref().len() < 20 {
            return Err(Error::new(ErrorKind::Other, "IP error len"));
        }
        let version_ihl = bytes.read_u8()?;
        let version = version_ihl >> 4;
        if version != 4 {
            return Err(Error::new(ErrorKind::Other, "IPv4 wrong version"));
        }
        let ihl = version_ihl & 0x0f;
        let tos = bytes.read_u8()?;
        let length = bytes.read_u16::<BigEndian>()?;
        let ident = bytes.read_u16::<BigEndian>()?;
        let fragment = bytes.read_u16::<BigEndian>()?;
        let flags = Flags {
            dont_flagment: (fragment & 0x8000) > 0,
            more_fragments: (fragment & 0x4000) > 0,
        };
        let ttl = bytes.read_u8()?;
        let protocol = bytes.read_u8()?;
        let checksum = bytes.read_u16::<BigEndian>()?;
        let src = bytes.read_u32::<BigEndian>()?;
        let dst = bytes.read_u32::<BigEndian>()?;
        let option_len = (ihl * 4) as usize - 20;
        let mut options = vec![0u8; option_len];
        bytes.read_exact(&mut options)?;

        let ptcol = match protocol {
            ptc if ptc == (IpProtocol::ICMP as u8) => {
                let icmp = ICMP::parse(bytes);
                match icmp {
                    Ok(v) => EtherData::ICMP(v),
                    Err(_) => EtherData::Unparse(protocol, bytes.fill_buf()?.to_vec()),
                }
            }
            ptc if ptc == (IpProtocol::TCP as u8) => {
                let tcp = TCP::parser(bytes);
                match tcp {
                    Ok(tcp) => EtherData::TCP(tcp),
                    Err(_) => EtherData::Unparse(protocol, bytes.fill_buf()?.to_vec()),
                }
            }
            ptc if ptc == (IpProtocol::UDP as u8) => {
                let udp = UDP::parser(bytes);
                match udp {
                    Ok(udp) => EtherData::UDP(udp),
                    Err(_) => EtherData::Unparse(protocol, bytes.fill_buf()?.to_vec()),
                }
            }
            _ => EtherData::Unparse(protocol, bytes.fill_buf()?.to_vec()),
        };
        Ok(IP {
            version,
            ihl,
            length,
            protocol,
            tos,
            ident,
            flags,
            fragment,
            ttl,
            checksum,
            src,
            dst,
            options,
            ptcol,
        })
    }
}

#[derive(Clone)]
pub enum EtherData {
    ICMP(ICMP),
    TCP(TCP),
    UDP(UDP),
    Unparse(u8, Vec<u8>),
}
