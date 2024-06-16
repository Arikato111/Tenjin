use std::io::Error;

use crate::openflow::ofp10::{
    events::{Action, FeaturesReqEvent, HelloEvent, PacketOutEvent, Payload},
    ofp_header::OfpHeader,
    Msg,
};

/**
 * the trait for parse value to bytes.
 * for use with Controller's send_msg.
 */
pub trait MessageMarshal {
    fn marshal(&self, bytes: &mut Vec<u8>);
    fn msg_code(&self) -> Msg;
    fn msg_usize(&self) -> usize;
    fn size_of(&self) -> usize;
}

/**
 * for works with controller to create OfpMsgEvent
 */
pub trait OfpMsgEvent {
    fn header(&self, message: u8, length: u16, xid: u32) -> OfpHeader;
    fn header_parse(&self, bytes: &Vec<u8>) -> Result<OfpHeader, Error>;
    fn version(&self) -> usize;
    fn ofp_version() -> usize;
    fn header_size(&self) -> usize;

    fn msg_usize(&self, msg: Msg) -> usize;
    fn msg_parse(&self, msg: u8) -> Msg;
    fn hello_event(&self) -> HelloEvent;
    fn fetures_req(&self) -> FeaturesReqEvent;
    fn packet_out(
        &self,
        port_id: Option<u16>,
        payload: Payload,
        actions: Vec<Action>,
    ) -> PacketOutEvent;
}
