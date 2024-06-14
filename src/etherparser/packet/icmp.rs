use std::io::{BufRead, Cursor};

use byteorder::{BigEndian, ReadBytesExt};

#[derive(Clone)]
pub struct ICMP {
    pub typ: u8,
    pub code: u8,
    pub checksum: u16,
    pub payload: Vec<u8>,
}

impl ICMP {
    pub fn size_of() -> usize {
        4
    }
    pub fn parse(bytes: &mut Cursor<Vec<u8>>) -> Option<ICMP> {
        if bytes.get_ref().len() < 4 {
            return None;
        }
        let typ = bytes.read_u8().unwrap();
        let code = bytes.read_u8().unwrap();
        let checksum = bytes.read_u16::<BigEndian>().unwrap();
        let payload = bytes.fill_buf().unwrap().to_vec();
        Some(ICMP {
            typ,
            code,
            checksum,
            payload,
        })
    }
}
