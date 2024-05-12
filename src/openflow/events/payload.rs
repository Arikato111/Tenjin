pub enum Payload {
    Buffered(u32, Vec<u8>),
    NoBuffered(Vec<u8>),
}
