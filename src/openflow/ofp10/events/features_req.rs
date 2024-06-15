use crate::openflow::ofp10::{MessageMarshal, Msg};

pub struct FeaturesReqEvent {}

impl FeaturesReqEvent {
    pub fn new() -> Self {
        FeaturesReqEvent {}
    }
}

impl MessageMarshal for FeaturesReqEvent {
    fn marshal(&self, _: &mut Vec<u8>) {}

    fn msg_code(&self) -> Msg {
        Msg::FeaturesRequest
    }

    fn size_of(&self) -> usize {
        0
    }

    fn msg_usize(&self) -> usize {
        Msg::FeaturesRequest as usize
    }
}
