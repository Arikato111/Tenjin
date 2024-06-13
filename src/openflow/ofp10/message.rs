#[repr(u8)]
pub enum Msg {
    Hello = 0,
    Error = 1,
    FeaturesReq = 5,
    PacketIn = 10,
    PacketOut = 13,
    FlowMod = 14,
    NotFound = 0xff,
}

impl Msg {
    pub fn to_int(&self) -> u8 {
        self.clone() as u8
    }
    pub fn parse(msg_code: u8) -> Self {
        type ConvTyp = u8;
        match msg_code {
            m if m == (Self::Hello as ConvTyp) => Self::Hello,
            m if m == (Self::Error as ConvTyp) => Self::Error,
            m if m == (Self::FeaturesReq as ConvTyp) => Self::FeaturesReq,
            m if m == (Self::PacketIn as ConvTyp) => Self::PacketIn,
            m if m == (Self::PacketOut as ConvTyp) => Self::PacketOut,
            m if m == (Self::FlowMod as ConvTyp) => Self::FlowMod,
            _ => Self::NotFound,
        }
    }
}

impl Clone for Msg {
    fn clone(&self) -> Self {
        match self {
            Self::Hello => Self::Hello,
            Self::Error => Self::Error,
            Self::FeaturesReq => Self::FeaturesReq,
            Self::PacketIn => Self::PacketIn,
            Self::PacketOut => Self::PacketOut,
            Self::FlowMod => Self::FlowMod,
            Self::NotFound => Self::NotFound,
        }
    }
}

