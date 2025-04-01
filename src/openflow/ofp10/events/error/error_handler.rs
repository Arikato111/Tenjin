//! OpenFlow 1.0 error event handling implementation.
//! This module provides functionality for parsing and handling OpenFlow error events
//! received from switches.

use std::{
    io::{BufRead, Cursor, Error},
    mem::size_of,
};

use super::error_type::ErrorType;
use crate::openflow::ofp10::{MessageMarshal, Msg};
use byteorder::{BigEndian, ReadBytesExt};

/// Represents an OpenFlow error event received from a switch.
/// Contains the error type and any additional payload data associated with the error.
pub struct ErrorEvent {
    /// The specific type of error that occurred
    pub error_type: ErrorType,
    /// Additional data associated with the error, if any
    pub payload: Vec<u8>,
}

impl ErrorEvent {
    /// Creates a new ErrorEvent with the specified error type and payload.
    /// 
    /// # Arguments
    /// * `error_type` - The type of error that occurred
    /// * `payload` - Additional data associated with the error
    pub fn new(error_type: ErrorType, payload: Vec<u8>) -> Self {
        ErrorEvent {
            error_type,
            payload,
        }
    }

    /// Parses a raw buffer into an ErrorEvent.
    /// 
    /// # Arguments
    /// * `buf` - The raw buffer containing the error event data
    /// 
    /// # Returns
    /// A Result containing either the parsed ErrorEvent or an IO Error
    pub fn parse(buf: &Vec<u8>) -> Result<ErrorEvent, Error> {
        let mut bytes = Cursor::new(buf);
        let error_type = bytes.read_u16::<BigEndian>()?;
        let error_code = bytes.read_u16::<BigEndian>()?;
        let code = ErrorType::new(error_type, error_code);
        let payload = bytes.fill_buf()?.to_vec();
        Ok(ErrorEvent::new(code, payload))
    }
}

/// Implementation of MessageMarshal trait for ErrorEvent.
/// Provides functionality for serializing and handling OpenFlow error messages.
impl MessageMarshal for ErrorEvent {
    /// Marshals the error event into a byte buffer.
    /// Currently not implemented as error events are only received, not sent.
    fn marshal(&self, _: &mut Vec<u8>) {}

    /// Returns the OpenFlow message type for error events.
    fn msg_code(&self) -> crate::openflow::ofp10::Msg {
        Msg::Error
    }

    /// Returns the message type as a usize.
    fn msg_usize(&self) -> usize {
        Msg::Error as usize
    }

    /// Returns the total size of the error event in bytes.
    fn size_of(&self) -> usize {
        size_of::<(u16, u16)>() + self.payload.len()
    }
}
