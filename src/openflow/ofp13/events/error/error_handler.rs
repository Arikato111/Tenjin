//! OpenFlow v1.3 Error Event Handler
//! 
//! This module implements the error event handling for OpenFlow v1.3 protocol.
//! It provides functionality for parsing and handling error messages received
//! from the OpenFlow switch.

use std::{
    io::{BufRead, Cursor, Error},
    mem::size_of,
};

use super::error_type::ErrorType;
use crate::openflow::ofp13::{MessageMarshal, Msg};
use byteorder::{BigEndian, ReadBytesExt};

/// Represents an OpenFlow error event message
/// 
/// Contains information about the error type and any additional payload data
/// associated with the error.
pub struct ErrorEvent {
    /// The type and code of the error
    pub error_type: ErrorType,
    /// Additional error data payload
    pub payload: Vec<u8>,
}

impl ErrorEvent {
    /// Creates a new error event with the specified error type and payload
    pub fn new(error_type: ErrorType, payload: Vec<u8>) -> Self {
        ErrorEvent {
            error_type,
            payload,
        }
    }

    /// Parses an error event from a byte buffer
    /// 
    /// # Arguments
    /// * `buf` - The byte buffer containing the error message
    /// 
    /// # Returns
    /// * `Result<ErrorEvent, Error>` - The parsed error event or an error if parsing fails
    pub fn parse(buf: &Vec<u8>) -> Result<ErrorEvent, Error> {
        let mut bytes = Cursor::new(buf);
        let error_type = bytes.read_u16::<BigEndian>()?;
        let error_code = bytes.read_u16::<BigEndian>()?;
        let code = ErrorType::new(error_type, error_code);
        let payload = bytes.fill_buf()?.to_vec();
        Ok(ErrorEvent::new(code, payload))
    }
}

/// Implementation of MessageMarshal trait for ErrorEvent
/// 
/// Provides methods for converting ErrorEvent to/from wire format
impl MessageMarshal for ErrorEvent {
    /// Marshals the error event into a byte buffer
    /// Note: Currently not implemented as error events are typically only received
    fn marshal(&self, _: &mut Vec<u8>) {}

    /// Returns the message code for error events
    fn msg_code(&self) -> Msg {
        Msg::Error
    }

    /// Returns the message code as a usize
    fn msg_usize(&self) -> usize {
        Msg::Error as usize
    }

    /// Returns the total size of the error message in bytes
    fn size_of(&self) -> usize {
        size_of::<(u16, u16)>() + self.payload.len()
    }
}
