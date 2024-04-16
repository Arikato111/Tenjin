use std::io::{BufRead, Cursor};

use byteorder::{BigEndian, ReadBytesExt};

pub struct UDP {
    pub src_port: u16,
    pub des_port: u16,
    pub checksum: u16,
    pub payload: Vec<u8>,
}

impl UDP {
    pub fn sizeof() -> usize {
        8
    }
    pub fn parser(bytes: &mut Cursor<Vec<u8>>) -> Option<UDP> {
        if bytes.get_ref().len() < UDP::sizeof() {
            return None;
        }
        let src_port = bytes.read_u16::<BigEndian>().unwrap();
        let des_port = bytes.read_u16::<BigEndian>().unwrap();
        let checksum = bytes.read_u16::<BigEndian>().unwrap();
        let payload = bytes.fill_buf().unwrap().to_vec();
        Some(UDP {
            src_port,
            des_port,
            checksum,
            payload,
        })
    }
}
