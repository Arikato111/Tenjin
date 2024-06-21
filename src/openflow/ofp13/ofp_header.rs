use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::{
    io::{Cursor, Error},
    mem::size_of,
};

use crate::openflow::ofp13::OpenflowHeader;

pub struct OfpHeader {
    pub version: u8,
    pub message: u8,
    pub length: u16,
    pub xid: u32,
}

impl OpenflowHeader for OfpHeader {
    fn new(message: u8, length: usize, xid: usize) -> Self {
        Self {
            version: 1,
            message,
            length: (size_of::<OfpHeader>() + length) as u16,
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
        size_of::<Self>()
    }
    fn pkt_size(&self) -> usize {
        return self.length as usize - size_of::<Self>();
    }

    fn parse(buf: &Vec<u8>) -> Result<Self, Error> {
        let mut buf_cursor = Cursor::new(buf);
        let version = buf_cursor.read_u8()?;
        let message = buf_cursor.read_u8()?;
        let length = buf_cursor.read_u16::<BigEndian>()?;
        let xid = buf_cursor.read_u32::<BigEndian>()?;
        Ok(Self {
            version,
            message,
            length,
            xid,
        })
    }
    fn marshal(&self, bytes: &mut Vec<u8>) {
        let _ = bytes.write_u8(self.version);
        let _ = bytes.write_u8(self.message);
        let _ = bytes.write_u16::<BigEndian>(self.length);
        let _ = bytes.write_u32::<BigEndian>(self.xid);
    }
}
