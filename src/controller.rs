#![allow(unused)]
#![allow(unused_variables)]
/**
 * Here is Controller you can modify and write the process or more you need.
 * In production please remove allow unused.
 */
use crate::openflow::events::PacketInEvent;
use crate::openflow::{controller_frame::ControllerFrame, traiter::OfpMsgEvent};
use std::{collections::HashMap, net::TcpStream};

pub struct Controller<OME: OfpMsgEvent> {
    ofp: OME,
    mac_to_port: HashMap<u64, u16>,
}

impl<OME: OfpMsgEvent> ControllerFrame<OME> for Controller<OME> {
    fn get_ofp(&self) -> &impl OfpMsgEvent {
        &self.ofp
    }
    fn new(ofp: OME) -> Self {
        Self {
            ofp,
            mac_to_port: HashMap::new(),
        }
    }
    /**
     * Start here for handle packetIn message.
     */
    fn packet_in_handler(&mut self, xid: u32, packetin: PacketInEvent, stream: &mut TcpStream) {}
}
