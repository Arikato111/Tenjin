#![allow(unused)]
#![allow(unused_variables)]
use crate::{
    etherparser::{ether_type::EtherType, MacAddr},
    openflow::ofp13::{
        self,
        events::{flow_mod::MatchFields, Action},
        ControllerFrame13, FlowModEvent, OfpMsgEvent, PacketInEvent,
    },
};
use std::collections::HashMap;
use tokio::net::TcpStream;
/**
 * Here is Controller you can modify and write the process or more you need.
 * In production please remove allow unused.
 */

#[derive(Clone)]
pub struct Controller13 {
    mac_to_port: HashMap<u64, u32>,
}

impl ControllerFrame13 for Controller13 {
    fn new() -> Self {
        Self {
            mac_to_port: HashMap::new(),
        }
    }
    /**
     * Start here for handle packetIn message.
     */
    async fn switch_features_handler(
        &self,
        xid: u32,
        features_reply: ofp13::FeaturesReplyEvent,
        stream: &mut TcpStream,
    ) {
        let matchs = MatchFields::match_all();
        let actions = vec![Action::Oputput(ofp13::PseudoPort::Controller(!0))];
        self.add_flow(xid, 0, matchs, &actions, 0, None, stream)
            .await;
    }
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
        let in_port = match packetin.matchs.in_port {
            Some(p) => p,
            None => return,
        };
        println!(
            "packet in {} {} {}",
            pkt.mac_src_string(),
            pkt.mac_dst_string(),
            in_port
        );

        self.mac_to_port.insert(pkt.mac_src.into(), in_port);

        let mac_dst = pkt.mac_dst;
        let mac_src = pkt.mac_src;

        if let EtherType::LLDP = pkt.ether_type {
            return;
        }

        let out_port = match self.mac_to_port.get(&mac_dst.into()) {
            Some(p) => ofp13::PseudoPort::PhysicalPort(*p),
            None => ofp13::PseudoPort::Flood,
        };

        let actions = vec![Action::Oputput(out_port.clone())];

        if let ofp13::PseudoPort::PhysicalPort(_) = out_port {
            let mut match_fields = MatchFields::match_all();
            match_fields.in_port = Some(in_port);
            match_fields.eth_dst = Some(mac_dst);
            match_fields.eth_src = Some(mac_src);
            if let Some(buf_id) = packetin.buf_id {
                self.add_flow(
                    xid,
                    1,
                    match_fields,
                    &actions,
                    packetin.table_id,
                    Some(buf_id),
                    stream,
                )
                .await;
                return;
            } else {
                self.add_flow(
                    xid,
                    1,
                    match_fields,
                    &actions,
                    packetin.table_id,
                    None,
                    stream,
                )
                .await;
            }
        }
        let packet_out = self
            .ofp()
            .packet_out(Some(in_port), packetin.payload, actions);
        self.send_msg(packet_out, xid, stream).await;
    }
}

impl Controller13 {
    async fn add_flow(
        &self,
        xid: u32,
        priority: u16,
        flow: MatchFields,
        actions: &Vec<Action>,
        table_id: u8,
        buffer_id: Option<u32>,
        stream: &mut TcpStream,
    ) {
        self.send_msg(
            FlowModEvent::add_flow(priority, flow, actions.clone(), table_id, buffer_id),
            xid,
            stream,
        )
        .await
    }
}
