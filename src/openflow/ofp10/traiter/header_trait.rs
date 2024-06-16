use std::io::Error;

pub trait OpenflowHeader {
    fn version(&self) -> usize;
    fn message(&self) -> u8;
    fn length(&self) -> usize;
    fn xid(&self) -> u32;
    fn pkt_size(&self) -> usize;

    fn new(message: u8, length: usize, xid: usize) -> Self;
    fn header_size(&self) -> usize;
    fn parse(buf: &Vec<u8>) -> Result<Self, Error>
    where
        Self: Sized;
    fn marshal(&self, bytes: &mut Vec<u8>);
}
