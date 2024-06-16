use std::io::{BufRead, Cursor, Error, ErrorKind};

use byteorder::{BigEndian, ReadBytesExt};

#[derive(Clone)]
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
    pub fn parser(bytes: &mut Cursor<Vec<u8>>) -> Result<UDP, Error> {
        if bytes.get_ref().len() < UDP::sizeof() {
            return Err(Error::new(ErrorKind::Other, "UDP error size"));
        }
        let src_port = bytes.read_u16::<BigEndian>()?;
        let des_port = bytes.read_u16::<BigEndian>()?;
        let checksum = bytes.read_u16::<BigEndian>()?;
        let payload = bytes.fill_buf()?.to_vec();
        Ok(UDP {
            src_port,
            des_port,
            checksum,
            payload,
        })
    }
}
