use std::{io::{BufRead, Cursor}, mem::size_of};

use crate::openflow::ofp10::{traiter::MessageMarshal, Msg};
use byteorder::{BigEndian, ReadBytesExt};

use super::error_type::ErrorType;

pub struct ErrorEvent {
    pub error_type: ErrorType,
    pub payload: Vec<u8>,
}

impl ErrorEvent {
    pub fn new(error_type: ErrorType, payload: Vec<u8>) -> Self {
        ErrorEvent {
            error_type,
            payload,
        }
    }
    pub fn parse(buf: &Vec<u8>) -> ErrorEvent {
        let mut bytes = Cursor::new(buf);
        let error_type = bytes.read_u16::<BigEndian>().unwrap();
        let error_code = bytes.read_u16::<BigEndian>().unwrap();
        let code = ErrorType::new(error_type, error_code);
        let payload = bytes.fill_buf().unwrap().to_vec();
        ErrorEvent::new(code, payload)
    }
}

impl MessageMarshal for ErrorEvent {
    fn marshal(&self, _: &mut Vec<u8>) {}

    fn msg_code(&self) -> crate::openflow::ofp10::Msg {
        Msg::Error
    }

    fn msg_usize<OFP: crate::openflow::ofp10::traiter::OfpMsgEvent>(&self, ofp: &OFP) -> usize {
        ofp.msg_usize(Msg::Error)
    }

    fn size_of(&self) -> usize {
        size_of::<(u16,u16)>() + self.payload.len()
    }
}
