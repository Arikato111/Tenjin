use crate::openflow::{
    events::{FeaturesReq, HelloEvent},
    OfpHeader,
};

use super::OfpMsg;

/**
 * the trait for parse value to bytes.
 * for use with Controller's send_msg.
 */
pub trait MessageMarshal {
    fn marshal(&self, bytes: &mut Vec<u8>);
    fn msg_code(&self) -> OfpMsg;
    fn msg_usize<OFP: OfpMsgEvent>(&self, ofp: &OFP) -> usize;
    fn size_of(&self) -> usize;
}

/**
 * for works with controller to create OfpMsgEvent
 */
pub trait OfpMsgEvent {
    fn header(&self, message: u8, length: u16, xid: u32) -> OfpHeader;
    fn version(&self) -> usize;
    fn ofp_version() -> usize;
    fn header_size(&self) -> usize;
    fn header_parse(&self, bytes:&Vec<u8>) -> OfpHeader;

    fn msg_usize(&self, msg: OfpMsg) -> usize;
    fn msg_parse(&self, msg: u16) -> OfpMsg;
    fn hello_event(&self) -> HelloEvent;
    fn fetures_req(&self) -> FeaturesReq;
}
