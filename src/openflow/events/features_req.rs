use crate::openflow::trait_marshal::MessageMarshal;

pub struct FeaturesReq {}

impl FeaturesReq {
    pub fn new() -> Self {
        FeaturesReq {}
    }
}

impl MessageMarshal for FeaturesReq {
    fn marshal(&self, _: &mut Vec<u8>) {}

    fn msg_code(&self) -> crate::openflow::OfpMsg {
        crate::openflow::OfpMsg::FeaturesReq
    }

    fn size_of(&self) -> usize {
        0
    }
}
