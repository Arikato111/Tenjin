use crate::openflow::{
    ofp10::Msg,
    traiter::{MessageMarshal, OfpMsgEvent},
};

pub struct FeaturesReqEvent {}

impl FeaturesReqEvent {
    pub fn new() -> Self {
        FeaturesReqEvent {}
    }
}

impl MessageMarshal for FeaturesReqEvent {
    fn marshal(&self, _: &mut Vec<u8>) {}

    fn msg_code(&self) -> Msg {
        Msg::FeaturesReq
    }

    fn size_of(&self) -> usize {
        0
    }

    fn msg_usize<OFP: OfpMsgEvent>(&self, ofp: &OFP) -> usize {
        ofp.msg_usize(Msg::FeaturesReq)
    }
}
