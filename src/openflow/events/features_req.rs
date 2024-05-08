use crate::openflow::ofp_manager::{MessageMarshal, OfpMsg, OfpMsgEvent};

pub struct FeaturesReq {}

impl FeaturesReq {
    pub fn new() -> Self {
        FeaturesReq {}
    }
}

impl MessageMarshal for FeaturesReq {
    fn marshal(&self, _: &mut Vec<u8>) {}

    fn msg_code(&self) -> OfpMsg {
        OfpMsg::FeaturesReq
    }

    fn size_of(&self) -> usize {
        0
    }

    fn msg_usize<OFP: OfpMsgEvent>(&self, ofp: &OFP) -> usize {
        ofp.msg_usize(OfpMsg::FeaturesReq)
    }
}
