use super::{
    events::{Action, FeaturesReqEvent, Payload},
    ofp_header::{OfpHeader, OfpHeader10, OpenflowHeader},
    traiter::OfpMsgEvent,
    HelloEvent, Msg, PacketOutEvent,
};

pub struct Openflow10 {}

impl Openflow10 {
    pub fn new() -> Self {
        Openflow10 {}
    }
}

impl OfpMsgEvent for Openflow10 {
    fn header_parse(&self, bytes: &Vec<u8>) -> OfpHeader<impl OpenflowHeader> {
        OfpHeader::new(OfpHeader10::parse(bytes))
    }
    fn header_size(&self) -> usize {
        8
    }
    fn hello_event(&self) -> HelloEvent {
        HelloEvent::new()
    }

    fn fetures_req(&self) -> FeaturesReqEvent {
        FeaturesReqEvent::new()
    }
    fn packet_out(
        &self,
        port_id: Option<u16>,
        payload: Payload,
        actions: Vec<Action>,
    ) -> PacketOutEvent {
        PacketOutEvent::new(port_id, payload, actions)
    }
    fn ofp_version() -> usize {
        1
    }
    fn version(&self) -> usize {
        1
    }

    fn header(&self, message: u8, length: u16, xid: u32) -> OfpHeader<impl OpenflowHeader> {
        OfpHeader::new(OfpHeader10::new(message, length as usize, xid as usize))
    }

    fn msg_parse(&self, msg: u16) -> Msg {
        match msg {
            0 => Msg::Hello,
            1 => Msg::Error,
            5 => Msg::FeaturesReq,
            10 => Msg::PacketIn,
            13 => Msg::PacketOut,
            14 => Msg::FlowMod,
            _ => Msg::NotFound,
        }
    }

    fn msg_usize(&self, msg: Msg) -> usize {
        match msg {
            Msg::Hello => 0,
            Msg::Error => 1,
            Msg::FeaturesReq => 5,
            Msg::PacketIn => 10,
            Msg::PacketOut => 13,
            Msg::FlowMod => 14,
            _ => 1024,
        }
    }
}
