use super::events::{FeaturesReq, HelloEvent};

pub trait OfpMsgEvent {
    fn hello_event(&self) -> HelloEvent;
    fn fetures_req(&self) -> FeaturesReq;
    fn version(&self) -> usize;
}

pub enum OfpMsg {
    Hello = 0,
    FeaturesReq = 5,
    PacketIn = 8,
    FlowMod = 14,
    NotFound = -1,
}

impl OfpMsgEvent for OfpMsg {
    fn hello_event(&self) -> HelloEvent {
        HelloEvent::new()
    }

    fn fetures_req(&self) -> FeaturesReq {
        FeaturesReq::new()
    }

    fn version(&self) -> usize {
        1
    }
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
