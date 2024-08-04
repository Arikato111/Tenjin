use crate::openflow::ofp13::{MessageMarshal, Msg};

pub struct HelloEvent {}

impl HelloEvent {
    pub fn new() -> Self {
        HelloEvent {}
    }
}

impl MessageMarshal for HelloEvent {
    fn marshal(&self, _: &mut Vec<u8>) {}

    fn msg_code(&self) -> Msg {
        Msg::Hello
    }

    fn size_of(&self) -> usize {
        0
    }

    fn msg_usize(&self) -> usize {
        Msg::Hello as usize
    }
}
