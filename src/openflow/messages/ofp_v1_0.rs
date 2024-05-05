use crate::openflow::{
    events::{FeaturesReq, HelloEvent},
    OfpHeader,
};

use super::traiter::OfpMsgEvent;
use super::OfpMsg;

pub struct Openflow10 {}

impl Openflow10 {
    pub fn new() -> Self {
        Openflow10 {}
    }
}

impl OfpMsgEvent for Openflow10 {
    fn hello_event(&self) -> HelloEvent {
        HelloEvent::new()
    }

    fn fetures_req(&self) -> FeaturesReq {
        FeaturesReq::new()
    }

    fn version(&self) -> usize {
        1
    }

    fn header(&self, message: u8, length: u16, xid: u32) -> OfpHeader {
        OfpHeader {
            version: 1,
            message,
            length,
            xid,
        }
    }

    fn msg_parse(&self, msg: u16) -> OfpMsg {
        match msg {
            0 => OfpMsg::Hello,
            5 => OfpMsg::FeaturesReq,
            8 => OfpMsg::PacketIn,
            14 => OfpMsg::FlowMod,
            _ => OfpMsg::NotFound,
        }
    }

    fn msg_usize(&self, msg: OfpMsg) -> usize {
        match msg {
            OfpMsg::Hello => 0,
            OfpMsg::FeaturesReq => 5,
            OfpMsg::PacketIn => 8,
            OfpMsg::FlowMod => 14,
            _ => 1024,
        }
    }
}
