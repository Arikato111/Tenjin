use std::io::Write;

pub enum Payload {
    Buffered(u32, Vec<u8>),
    NoBuffered(Vec<u8>),
}

impl Payload {
    pub fn length(&self) -> usize {
        match self {
            Payload::Buffered(_, p) | Payload::NoBuffered(p) => p.len(),
        }
    }
    pub fn marshal(&self, bytes: &mut Vec<u8>) {
        match self {
            Payload::Buffered(_, buf) | Payload::NoBuffered(buf) => {
                let _ = bytes.write_all(buf);
            }
        }
    }
}
