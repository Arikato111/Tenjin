#![allow(unused)]
#![allow(unused_variables)]
use etherparse::EtherType;
use std::collections::HashMap;
use tokio::net::TcpStream;

use crate::{
    etherparser::{
        lib::{GetMacAddr, Log, ToU64},
        MacAddr,
    },
    openflow::ofp10::{
        self,
        events::{flow_mod::MatchFields, Action},
        ControllerFrame10, FlowModEvent, OfpMsgEvent, PacketInEvent,
    },
};
/**
 * Here is Controller you can modify and write the process or more you need.
 * In production please remove allow unused.
 */

#[derive(Clone)]
pub struct Controller10 {
    mac_to_port: HashMap<u64, u16>,
}

impl ControllerFrame10 for Controller10 {
    fn new() -> Self {
        Self {
            mac_to_port: HashMap::new(),
        }
    }
    /**
     * Start here for handle packetIn message.
     */
    async fn packet_in_handler(
        &mut self,
        xid: u32,
        packetin: PacketInEvent,
        stream: &mut TcpStream,
    ) {
        let pkt = match packetin.ether_parse() {
            Ok(pkt) => pkt,
            Err(_) => return,
        };
        let (mac_dst, mac_src, ether_type) = match pkt.link.macs() {
            Ok(macs) => (
                MacAddr::from(macs.destination),
                MacAddr::from(macs.source),
                macs.ether_type,
            ),
            Err(_) => return,
        };

        if let Some(net) = pkt.net {
            println!(
                "packet in {} {} {}",
                mac_src.to_string(),
                mac_dst.to_string(),
                packetin.in_port
            );
        }

        self.mac_to_port.insert(mac_src.into(), packetin.in_port);

        // LLDP = 0x88cc
        if EtherType::from(0x88cc) == ether_type {
            return;
        }

        let out_port = match self.mac_to_port.get(&mac_dst.into()) {
            Some(p) => ofp10::PseudoPort::PhysicalPort(*p),
            None => ofp10::PseudoPort::Flood,
        };

        let actions = vec![Action::Oputput(out_port.clone())];

        if let ofp10::PseudoPort::PhysicalPort(_) = out_port {
            let mut match_fields = ofp10::MatchFields::match_all();
            match_fields.in_port = Some(packetin.in_port);
            match_fields.mac_dest = Some(mac_dst);
            match_fields.mac_src = Some(mac_src);
            if let Some(buf_id) = packetin.buf_id {
                self.add_flow(xid, 1, match_fields, &actions, Some(buf_id), stream)
                    .await;
                return;
            } else {
                self.add_flow(xid, 1, match_fields, &actions, None, stream)
                    .await;
            }
        }
        let packet_out = self
            .ofp()
            .packet_out(Some(packetin.in_port), packetin.payload, actions);
        self.send_msg(packet_out, xid, stream).await;
    }
}

impl Controller10 {
    async fn add_flow(
        &self,
        xid: u32,
        priority: u16,
        flow: MatchFields,
        actions: &Vec<Action>,
        buffer_id: Option<u32>,
        stream: &mut TcpStream,
    ) {
        self.send_msg(
            FlowModEvent::add_flow(10, flow, actions.clone(), buffer_id),
            xid,
            stream,
        )
        .await;
    }
}
