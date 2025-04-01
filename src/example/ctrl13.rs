//! OpenFlow 1.3 Controller Implementation
//! 
//! This module implements an OpenFlow 1.3 controller that handles packet forwarding
//! and flow management in a software-defined network.
#![allow(unused)]
#![allow(unused_variables)]

use crate::{
    openflow::ofp13::{
        self,
        events::{flow_mod::MatchFields, Action},
        ControllerFrame13, FlowModEvent, OfpMsgEvent, PacketInEvent,
    },
    utils::{net::GetMacAddr, MacAddr},
};
use etherparse::{EtherType, Ethernet2Header};
use std::collections::HashMap;
use tokio::net::TcpStream;

/// OpenFlow 1.3 Controller implementation
/// 
/// This controller maintains a mapping of MAC addresses to ports and handles
/// packet forwarding based on this information.
#[derive(Clone)]
pub struct Controller13 {
    /// Mapping of MAC addresses to physical ports
    mac_to_port: HashMap<u64, u32>,
}

impl ControllerFrame13 for Controller13 {
    /// Creates a new instance of Controller13
    fn new() -> Self {
        Self {
            mac_to_port: HashMap::new(),
        }
    }

    /// Handles switch features reply messages
    /// 
    /// Sets up initial flow rules to send all packets to the controller
    async fn switch_features_handler(
        &self,
        xid: u32,
        features_reply: ofp13::FeaturesReplyEvent,
        stream: &mut TcpStream,
    ) {
        let matchs = MatchFields::match_all();
        let actions = vec![Action::Oputput(ofp13::PseudoPort::Controller(!0))];
        let _ = self.add_flow(xid, 0, matchs, &actions, 0, None, stream).await;
    }

    /// Handles incoming packets
    /// 
    /// This function implements the main packet forwarding logic:
    /// 1. Parses the incoming packet
    /// 2. Updates the MAC-to-port mapping
    /// 3. Determines the output port
    /// 4. Sets up flow rules for future packets
    /// 5. Forwards the packet if necessary
    async fn packet_in_handler(
        &mut self,
        xid: u32,
        packetin: PacketInEvent,
        stream: &mut TcpStream,
    ) {
        let pkt = match packetin.ether_parse() {
            Ok(pkt) => pkt,
            Err(e) => {
                eprintln!("Failed to parse ethernet packet: {}", e);
                return;
            }
        };

        // Extract MAC addresses and ethernet type
        let macs = match pkt.link.macs() {
            Ok(macs) => macs,
            Err(e) => {
                eprintln!("Failed to get MAC addresses: {}", e);
                return;
            }
        };

        let in_port = match packetin.matchs.in_port {
            Some(p) => p,
            None => {
                eprintln!("No input port specified");
                return;
            }
        };

        let mac_dst = MacAddr::from(macs.destination);
        let mac_src = MacAddr::from(macs.source);
        let ether_type = macs.ether_type;

        // Log packet information if network layer is present
        if let Some(net) = pkt.net {
            println!(
                "Packet in: src={}, dst={}, port={}",
                mac_src.to_string(),
                mac_dst.to_string(),
                in_port
            );
        }

        // Update MAC-to-port mapping
        self.mac_to_port.insert(mac_src.into(), in_port);

        // Skip LLDP packets (0x88cc)
        if EtherType::from(0x88cc) == ether_type {
            return;
        }

        // Determine output port based on destination MAC
        let out_port = match self.mac_to_port.get(&mac_dst.into()) {
            Some(p) => ofp13::PseudoPort::PhysicalPort(*p),
            None => ofp13::PseudoPort::Flood,
        };

        let actions = vec![Action::Oputput(out_port.clone())];

        // If we know the destination port, set up a flow rule
        if let ofp13::PseudoPort::PhysicalPort(_) = out_port {
            let mut match_fields = MatchFields::match_all();
            match_fields.in_port = Some(in_port);
            match_fields.eth_dst = Some(mac_dst);
            match_fields.eth_src = Some(mac_src);

            // Use buffer ID if available to avoid packet duplication
            if let Some(buf_id) = packetin.buf_id {
                let _ = self.add_flow(
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
            }

            // Add flow rule without buffer ID
            let _ = self.add_flow(
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

        // Forward the packet
        let packet_out = self
            .ofp()
            .packet_out(Some(in_port), packetin.payload, actions);
        let _ = self.send_msg(packet_out, xid, stream).await;
    }
}

impl Controller13 {
    /// Adds a flow rule to the switch
    /// 
    /// # Arguments
    /// * `xid` - Transaction ID
    /// * `priority` - Flow rule priority
    /// * `flow` - Match fields for the flow
    /// * `actions` - Actions to perform on matching packets
    /// * `table_id` - Table to add the flow to
    /// * `buffer_id` - Optional buffer ID for packet buffering
    /// * `stream` - TCP stream to the switch
    async fn add_flow(
        &self,
        xid: u32,
        priority: u16,
        flow: MatchFields,
        actions: &[Action],
        table_id: u8,
        buffer_id: Option<u32>,
        stream: &mut TcpStream,
    ) {
        let _ = self.send_msg(
            FlowModEvent::add_flow(priority, flow, actions.to_vec(), table_id, buffer_id),
            xid,
            stream,
        )
        .await;
    }
}
