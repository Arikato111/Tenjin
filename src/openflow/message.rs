pub enum OfpMsg {
    Hello = 0,
    FeaturesReq = 5,
    PacketIn = 8,
    FlowMod = 14,
    NotFound = -1,
}

impl OfpMsg {
    pub fn parse(message_code: u8) -> Self {
        match message_code {
            0 => OfpMsg::Hello,
            8 => OfpMsg::PacketIn,
            _ => OfpMsg::NotFound,
        }
    }
}
