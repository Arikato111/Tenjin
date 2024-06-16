use byteorder::{BigEndian, ReadBytesExt};
use std::io::{BufRead, Cursor, Error, ErrorKind};

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
    pub fn parse(bytes: &mut Cursor<Vec<u8>>) -> Result<ICMP, Error> {
        if bytes.get_ref().len() < 4 {
            return Err(Error::new(ErrorKind::Other, "icmp len wrong"));
        }
        let typ = bytes.read_u8()?;
        let code = bytes.read_u8()?;
        let checksum = bytes.read_u16::<BigEndian>()?;
        let payload = bytes.fill_buf()?.to_vec();
        Ok(ICMP {
            typ,
            code,
            checksum,
            payload,
        })
    }
}
