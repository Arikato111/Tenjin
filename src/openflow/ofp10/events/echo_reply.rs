use std::io::Write;

use crate::openflow::ofp10::{self, MessageMarshal};

pub struct EchoReplyEvent {
    pub payload: Vec<u8>,
}

impl EchoReplyEvent {
    pub fn new(payload: Vec<u8>) -> Self {
        Self { payload }
    }
}

impl MessageMarshal for EchoReplyEvent {
    fn marshal(&self, bytes: &mut Vec<u8>) {
        let _ = bytes.write_all(&self.payload);
    }

    fn msg_code(&self) -> ofp10::Msg {
        ofp10::Msg::EchoReply
    }

    fn msg_usize(&self) -> usize {
        ofp10::Msg::EchoReply as usize
    }

    fn size_of(&self) -> usize {
        self.payload.len()
    }
}
