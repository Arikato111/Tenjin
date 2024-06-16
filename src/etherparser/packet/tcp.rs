use crate::etherparser::tools::bits::bit_bool;
use byteorder::{BigEndian, ReadBytesExt};
use std::io::{BufRead, Cursor, Error, ErrorKind};

#[derive(Clone)]
pub struct TCP {
    pub src_port: u16,
    pub dst_port: u16,
    pub seq: u32,
    pub ack: u32,
    pub offset: u8,
    pub flags: TcpFlags,
    pub window: u16,
    pub checksum: u16,
    pub urgent: u16,
    pub payload: Vec<u8>,
}

impl TCP {
    pub fn size_of() -> usize {
        20
    }
    pub fn parser(bytes: &mut Cursor<Vec<u8>>) -> Result<TCP, Error> {
        if bytes.get_ref().len() < TCP::size_of() {
            return Err(Error::new(ErrorKind::Other, "Tcp wrong size"));
        }
        let src_port = bytes.read_u16::<BigEndian>()?;
        let dst_port = bytes.read_u16::<BigEndian>()?;
        let seq = bytes.read_u32::<BigEndian>()?;
        let ack = bytes.read_u32::<BigEndian>()?;
        let dataoff_reserv_flags = bytes.read_u16::<BigEndian>()?;
        let flags = TcpFlags::parser(dataoff_reserv_flags);
        let offset = (dataoff_reserv_flags >> 12) as u8 & 0x0f;
        let window = bytes.read_u16::<BigEndian>()?;
        let checksum = bytes.read_u16::<BigEndian>()?;
        let urgent = bytes.read_u16::<BigEndian>()?;
        let payload = bytes.fill_buf()?.to_vec();
        Ok(TCP {
            src_port,
            dst_port,
            seq,
            ack,
            offset,
            flags,
            window,
            checksum,
            urgent,
            payload,
        })
    }
}

#[derive(Clone)]
pub struct TcpFlags {
    pub ns: bool,
    pub cwr: bool,
    pub ece: bool,
    pub urg: bool,
    pub ack: bool,
    pub psh: bool,
    pub rst: bool,
    pub syn: bool,
    pub fin: bool,
}

impl TcpFlags {
    pub fn parser(bytes: u16) -> TcpFlags {
        TcpFlags {
            ns: bit_bool(0, bytes),
            cwr: bit_bool(1, bytes),
            ece: bit_bool(2, bytes),
            urg: bit_bool(3, bytes),
            ack: bit_bool(4, bytes),
            psh: bit_bool(5, bytes),
            rst: bit_bool(6, bytes),
            syn: bit_bool(7, bytes),
            fin: bit_bool(8, bytes),
        }
    }
}
