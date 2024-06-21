use std::mem::transmute;

#[derive(Clone)]
pub enum Msg {
    Hello = 0,
    Error = 1,
    EchoRequest = 2,
    EchoReply = 3,
    Experimenter = 4,
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
    GroupMod = 15,
    PortMod = 16,
    TableMod = 17,
    MultipartRequest = 18,
    MultipartReply = 19,
    BarrierRequest = 20,
    BarrierReply = 21,
    GetConfigRequest = 22,
    GetConfigReply = 23,
    RoleRequest = 24,
    RoleReply = 25,
    GetAsyncRequest = 26,
    GetAsyncReply = 27,
    SetAsync = 28,
    MeterMod = 29,
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
