use super::OpenflowHeader;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::{io::Cursor, mem::size_of};

pub struct OfpHeader10 {
    pub version: u8,
    pub message: u8,
    pub length: u16,
    pub xid: u32,
}

impl OpenflowHeader for OfpHeader10 {
    fn new(message: u8, length: usize, xid: usize) -> Self {
        Self {
            version: 1,
            message,
            length: (size_of::<OfpHeader10>() + length) as u16,
            xid: xid as u32,
        }
    }
    fn version(&self) -> usize {
        1
    }
    fn message(&self) -> u8 {
        self.message
    }
    fn length(&self) -> usize {
        self.length as usize
    }
    fn xid(&self) -> u32 {
        self.xid
    }
    fn header_size(&self) -> usize {
        size_of::<OfpHeader10>()
    }
    fn pkt_size(&self) -> usize {
        return self.length as usize - size_of::<OfpHeader10>();
    }

    fn parse(buf: &Vec<u8>) -> Self {
        let mut buf_cursor = Cursor::new(buf);
        let version = buf_cursor.read_u8().unwrap();
        let message = buf_cursor.read_u8().unwrap();
        let length = buf_cursor.read_u16::<BigEndian>().unwrap();
        let xid = buf_cursor.read_u32::<BigEndian>().unwrap();
        Self {
            version,
            message,
            length,
            xid,
        }
    }
    fn marshal(&self, bytes: &mut Vec<u8>) {
        bytes.write_u8(self.version).unwrap();
        bytes.write_u8(self.message).unwrap();
        bytes.write_u16::<BigEndian>(self.length).unwrap();
        bytes.write_u32::<BigEndian>(self.xid).unwrap();
    }
}
