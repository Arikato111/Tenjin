use std::{
    io::{BufRead, Cursor, Error},
    mem::size_of,
};

use super::error_type::ErrorType;
use crate::openflow::ofp10::{MessageMarshal, Msg};
use byteorder::{BigEndian, ReadBytesExt};

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
    pub fn parse(buf: &Vec<u8>) -> Result<ErrorEvent, Error> {
        let mut bytes = Cursor::new(buf);
        let error_type = bytes.read_u16::<BigEndian>()?;
        let error_code = bytes.read_u16::<BigEndian>()?;
        let code = ErrorType::new(error_type, error_code);
        let payload = bytes.fill_buf()?.to_vec();
        Ok(ErrorEvent::new(code, payload))
    }
}

impl MessageMarshal for ErrorEvent {
    fn marshal(&self, _: &mut Vec<u8>) {}

    fn msg_code(&self) -> crate::openflow::ofp10::Msg {
        Msg::Error
    }

    fn msg_usize(&self) -> usize {
        Msg::Error as usize
    }

    fn size_of(&self) -> usize {
        size_of::<(u16, u16)>() + self.payload.len()
    }
}
