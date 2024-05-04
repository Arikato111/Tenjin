use crate::openflow::{trait_marshal::MessageMarshal, OfpMsg};

pub struct HelloEvent {}

impl HelloEvent {
   pub fn new() -> Self {
        HelloEvent {}
    }
}

impl MessageMarshal for HelloEvent {
    fn marshal(&self, _: &mut Vec<u8>) {}

    fn msg_code(&self) -> crate::openflow::OfpMsg {
        OfpMsg::Hello
    }

    fn size_of(&self) -> usize {
        0
    }
}
