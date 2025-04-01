use super::{
    events::{Action, FeaturesReqEvent, Payload},
    ofp_header::OfpHeader,
    HelloEvent, Msg, OfpMsgEvent, OpenflowHeader, PacketOutEvent,
};

// Openflow13 implements the OpenFlow 1.3 protocol specification
// This struct provides methods for handling OpenFlow 1.3 protocol messages and events
pub struct Openflow13 {}

impl Openflow13 {
    /// Creates a new Openflow13 instance
    pub fn new() -> Self {
        Openflow13 {}
    }
}

impl Default for Openflow13 {
    fn default() -> Self {
        Self::new()
    }
}

/// Implements the OfpMsgEvent trait for Openflow13
/// This implementation provides methods for parsing and handling OpenFlow protocol messages
impl OfpMsgEvent for Openflow13 {
    /// Parses the OpenFlow header from raw bytes
    /// Returns a Result containing either the parsed OfpHeader or an IO error
    fn header_parse(&self, bytes: &Vec<u8>) -> Result<OfpHeader, std::io::Error> {
        OfpHeader::parse(bytes)
    }

    /// Returns the size of the OpenFlow header in bytes
    fn header_size(&self) -> usize {
        8
    }

    /// Creates a new Hello event for OpenFlow protocol handshake
    fn hello_event(&self) -> HelloEvent {
        HelloEvent::new()
    }

    /// Creates a new Features Request event to query switch capabilities
    fn fetures_req(&self) -> FeaturesReqEvent {
        FeaturesReqEvent::new()
    }

    /// Creates a new Packet Out event for sending packets through the switch
    /// Parameters:
    /// - port_id: Optional port number to send the packet out
    /// - payload: The packet data to send
    /// - actions: List of actions to apply to the packet
    fn packet_out(
        &self,
        port_id: Option<u32>,
        payload: Payload,
        actions: Vec<Action>,
    ) -> PacketOutEvent {
        PacketOutEvent::new(port_id, payload, actions)
    }

    /// Returns the OpenFlow protocol version (0x04 for version 1.3)
    fn ofp_version() -> usize {
        0x04
    }

    /// Returns the current OpenFlow protocol version
    fn version(&self) -> usize {
        Self::ofp_version()
    }

    /// Creates a new OpenFlow header with the specified parameters
    /// Parameters:
    /// - message: The message type
    /// - length: The total message length
    /// - xid: Transaction ID
    fn header(&self, message: u8, length: u16, xid: u32) -> OfpHeader {
        OfpHeader::new(message, length as usize, xid as usize)
    }

    /// Converts a message type byte to a Msg enum
    fn msg_parse(&self, msg: u8) -> Msg {
        Msg::from(msg)
    }

    /// Converts a Msg enum to its corresponding integer value
    fn msg_usize(&self, msg: Msg) -> usize {
        msg.to_int() as usize
    }
}
