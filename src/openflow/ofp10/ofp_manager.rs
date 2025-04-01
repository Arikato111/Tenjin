// OpenFlow Protocol v1.0 manager implementation
// This struct handles the core functionality for OpenFlow v1.0 protocol operations
use super::{
    events::{Action, FeaturesReqEvent, Payload},
    ofp_header::OfpHeader,
    HelloEvent, Msg, OfpMsgEvent, OpenflowHeader, PacketOutEvent,
};

pub struct Openflow10 {}

impl Openflow10 {
    /// Creates a new OpenFlow v1.0 manager instance
    pub fn new() -> Self {
        Openflow10 {}
    }
}

/// Implementation of OpenFlow Protocol v1.0 message events
/// This trait provides methods for handling various OpenFlow protocol operations
impl OfpMsgEvent for Openflow10 {
    /// Parses the OpenFlow header from raw bytes
    /// Returns a Result containing either the parsed OfpHeader or an IO error
    fn header_parse(&self, bytes: &Vec<u8>) -> Result<OfpHeader, std::io::Error>  {
        OfpHeader::parse(bytes)
    }

    /// Returns the size of the OpenFlow header in bytes
    fn header_size(&self) -> usize {
        8
    }

    /// Creates a new Hello event for protocol version negotiation
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
    /// - payload: The packet data to be sent
    /// - actions: List of actions to be applied to the packet
    fn packet_out(
        &self,
        port_id: Option<u16>,
        payload: Payload,
        actions: Vec<Action>,
    ) -> PacketOutEvent {
        PacketOutEvent::new(port_id, payload, actions)
    }

    /// Returns the OpenFlow protocol version (1.0)
    fn ofp_version() -> usize {
        1
    }

    /// Returns the current protocol version
    fn version(&self) -> usize {
        1
    }

    /// Creates a new OpenFlow header with the specified parameters
    /// Parameters:
    /// - message: The message type
    /// - length: The total message length
    /// - xid: Transaction ID
    fn header(&self, message: u8, length: u16, xid: u32) -> OfpHeader {
        OfpHeader::new(message, length as usize, xid as usize)
    }

    /// Converts a raw message byte into a Msg enum
    fn msg_parse(&self, msg: u8) -> Msg {
        Msg::from(msg)
    }

    /// Converts a Msg enum into its corresponding integer value
    fn msg_usize(&self, msg: Msg) -> usize {
        msg.to_int() as usize
    }
}
