use crate::openflow::ofp_manager::{MessageMarshal, OfpMsg};
use crate::openflow::traiter::OfpMsgEvent;

pub struct HelloEvent {}

impl HelloEvent {
    pub fn new() -> Self {
        HelloEvent {}
    }
}

impl MessageMarshal for HelloEvent {
    fn marshal(&self, _: &mut Vec<u8>) {}

    fn msg_code(&self) -> OfpMsg {
        OfpMsg::Hello
    }

    fn size_of(&self) -> usize {
        0
    }

    fn msg_usize<OFP: OfpMsgEvent>(&self, ofp: &OFP) -> usize {
        ofp.msg_usize(OfpMsg::Hello)
    }
}
