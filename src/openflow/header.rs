use std::{io::Cursor, mem::size_of};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

pub struct OfpHeader {
    pub version: u8,
    pub message: u8,
    pub length: u16,
    pub xid: u32,
}

impl OfpHeader {
    pub fn size(&self) -> usize {
        return self.length as usize - size_of::<OfpHeader>();
    }
    pub fn new(version: u8, message: u8, length: u16, xid: u32) -> Self {
        Self {
            version,
            message,
            length,
            xid,
        }
    }
    pub fn parse(buf: &Vec<u8>) -> Self {
        let mut buf_cursor = Cursor::new(buf);
        // split data from header
        let version = buf_cursor.read_u8().unwrap();
        let message = buf_cursor.read_u8().unwrap();
        // size is size of packet
        let length = buf_cursor.read_u16::<BigEndian>().unwrap();
        let xid = buf_cursor.read_u32::<BigEndian>().unwrap();
        Self {
            version,
            message,
            length,
            xid,
        }
    }
    pub fn marshal(&self, bytes: &mut Vec<u8>) {
        bytes.write_u8(self.version).unwrap();
        bytes.write_u8(self.message).unwrap();
        bytes.write_u16::<BigEndian>(self.length).unwrap();
        bytes.write_u32::<BigEndian>(self.xid).unwrap();
    }
}
