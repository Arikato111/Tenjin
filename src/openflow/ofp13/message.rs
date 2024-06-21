use std::mem::transmute;

#[derive(Clone)]
pub enum Msg {
    Hello = 0,
    Error = 1,
    EchoRequest = 2,
    EchoReply = 3,
    Vendor = 4,
    FeaturesRequest = 5,
    FeaturesReply = 6,
    ConfigRequest = 7,
    ConfigReply = 8,
    SetConfig = 9,
    PacketIn = 10,
    FlowRemove = 11,
    PortStatus = 12,
    PacketOut = 13,
    FlowMod = 14,
    PortMod = 15,
    StatsRequest = 16,
    StateReply = 17,
    BarrierRequest = 18,
    BarrierReply = 19,
    QueueGetConfigRequest = 20,
    QueueGetConfigReply = 21,
    NotFound = 0xff,
}

impl Msg {
    pub fn to_int(&self) -> u8 {
        self.clone() as u8
    }
    pub fn from(msg_code: u8) -> Self {
        if msg_code > 21 {
            return Self::NotFound;
        }
        unsafe { transmute::<u8, Msg>(msg_code) }
    }
}
