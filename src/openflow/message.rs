pub enum Msg {
    Hello,
    PacketIn,
    NotFound,
}

impl Msg {
    pub fn parse(message_code: u8) -> Self {
        match message_code {
            0 => Msg::Hello,
            8 => Msg::PacketIn,
            _ => Msg::NotFound,
        }
    }
}
