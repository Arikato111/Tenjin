use std::io::Write;

use crate::openflow::ofp10::{self, MessageMarshal, Msg};

pub struct EchoRequestEvent {
    pub payload: Vec<u8>,
}

impl EchoRequestEvent {
    pub fn new(payload: Vec<u8>) -> Self {
        Self { payload }
    }
}

impl MessageMarshal for EchoRequestEvent {
    fn marshal(&self, bytes: &mut Vec<u8>) {
        let _ = bytes.write_all(&self.payload);
    }

    fn msg_code(&self) -> ofp10::Msg {
        Msg::EchoRequest
    }

    fn msg_usize<OFP: ofp10::OfpMsgEvent>(&self, _: &OFP) -> usize {
        Msg::EchoRequest as usize
    }

    fn size_of(&self) -> usize {
        self.payload.len()
    }
}
