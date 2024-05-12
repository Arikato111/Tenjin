#![allow(unused)]
#![allow(unused_variables)]
use crate::openflow::events::flow_mod::{FlowModCommand, MatchFields};
/**
 * Here is Controller you can modify and write the process or more you need.
 * In production please remove allow unused.
 */
use crate::openflow::events::{FlowAction, FlowModEvent, PacketInEvent};
use crate::openflow::PseudoPort;
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
    fn packet_in_handler(&mut self, xid: u32, packetin: PacketInEvent, stream: &mut TcpStream) {
        let pkt = packetin.ether_parse();
        self.mac_to_port.insert(pkt.mac_src, packetin.port);

        let mac_dst = pkt.mac_des;
        let mac_src = pkt.mac_src;
        let out_port = self.mac_to_port.get(&mac_dst);
        match out_port {
            Some(p) => {
                let src_port = packetin.port;
                let mut src_dst_match = MatchFields::match_all();
                src_dst_match.mac_dest = Some(mac_dst);
                src_dst_match.mac_src = Some(mac_src);

                let mut dst_src_match = MatchFields::match_all();
                dst_src_match.mac_dest = Some(mac_src);
                dst_src_match.mac_src = Some(mac_dst);

                let actions = vec![FlowAction::Oputput(PseudoPort::PhysicalPort(*p))];
                self.add_flow(0, src_dst_match, actions, stream);

                let actions = vec![FlowAction::Oputput(PseudoPort::PhysicalPort(src_port))];
                self.add_flow(0, dst_src_match, actions, stream);

                let packet_out = self.ofp.packet_out(
                    None,
                    packetin.payload,
                    vec![FlowAction::Oputput(PseudoPort::PhysicalPort(*p))],
                );
                self.send_msg(packet_out, xid, stream);
            }
            None => {
                let packet_out = self.ofp.packet_out(
                    None,
                    packetin.payload,
                    vec![FlowAction::Oputput(PseudoPort::AllPorts)],
                );
            }
        }
    }
}

impl<OME: OfpMsgEvent> Controller<OME> {
    fn add_flow(
        &self,
        xid: u32,
        flow: MatchFields,
        actions: Vec<FlowAction>,
        stream: &mut TcpStream,
    ) {
        self.send_msg(FlowModEvent::add_flow(10, flow, actions), xid, stream)
    }
}
