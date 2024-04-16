pub enum Msg {
    Hello,
    PacketIn(Vec<u8>),
    NotFound,
}

impl Msg {
    pub fn parse(message_code: u8, payload: &Vec<u8>) -> Self {
        match message_code {
            0 => Msg::Hello,
            8 => Msg::PacketIn(payload.clone()),
            _ => Msg::NotFound,
        }
    }
}
