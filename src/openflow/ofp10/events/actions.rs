//! OpenFlow 1.0 Actions
//!
//! This module implements the action types and structures used in OpenFlow 1.0
//! for packet manipulation and forwarding. Actions define what operations should
//! be performed on packets as they flow through the switch.
//!
//! The module provides:
//! - Action type definitions
//! - Action structure implementations
//! - Serialization/deserialization of actions
//! - Action sequence handling
//! - Size checking and controller action reordering

use std::{
    io::{BufRead, Cursor, Error},
    mem::size_of,
};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::{openflow::ofp10::PseudoPort, utils::MacAddr};

/// Represents the standard OpenFlow 1.0 action types
///
/// Each variant corresponds to a specific action that can be performed on packets
/// as defined in the OpenFlow 1.0 specification.
#[derive(Debug)]
pub enum ActionType {
    /// Forward packet to a specific port
    Output = 0,
    /// Set VLAN ID
    SetVlanId = 1,
    /// Set VLAN priority
    SetVlanPCP = 2,
    /// Remove VLAN header
    StripVlan = 3,
    /// Set source MAC address
    SetSrcMac = 4,
    /// Set destination MAC address
    SetDstMac = 5,
    /// Set IPv4 source address
    SetIPv4Src = 6,
    /// Set IPv4 destination address
    SetIPv4Des = 7,
    /// Set IP Type of Service
    SetTos = 8,
    /// Set transport source port
    SetTpSrc = 9,
    /// Set transport destination port
    SetTpDst = 10,
    /// Forward packet to a specific queue
    Enqueue = 11,
}

/// Represents an OpenFlow 1.0 action with its associated parameters
///
/// Each variant contains the necessary data for performing the specific action
/// on packets flowing through the switch.
#[derive(Clone, Debug)]
pub enum Action {
    /// Forward packet to a specific port
    Oputput(PseudoPort),
    /// Set or remove VLAN ID
    SetDlVlan(Option<u16>),
    /// Set VLAN priority
    SetDlVlanPcp(u8),
    /// Set source MAC address
    SetDlSrc(MacAddr),
    /// Set destination MAC address
    SetDlDest(MacAddr),
    /// Set IPv4 source address
    SetIpSrc(u32),
    /// Set IPv4 destination address
    SetIpDes(u32),
    /// Set IP Type of Service
    SetTos(u8),
    /// Set transport source port
    SetTpSrc(u16),
    /// Set transport destination port
    SetTpDest(u16),
    /// Forward packet to a specific queue
    Enqueue(PseudoPort, u32),
    /// Action that could not be parsed
    Unparsable,
}

impl Action {
    /// Converts an action to its corresponding action type code
    ///
    /// # Returns
    /// The ActionType enum variant corresponding to this action
    pub fn to_action_code(&self) -> ActionType {
        match self {
            Action::Oputput(_) => ActionType::Output,
            Action::SetDlVlan(_) => ActionType::SetVlanId,
            Action::SetDlVlanPcp(_) => ActionType::SetVlanPCP,
            Action::SetDlSrc(_) => ActionType::SetSrcMac,
            Action::SetDlDest(_) => ActionType::SetDstMac,
            Action::SetIpSrc(_) => ActionType::SetIPv4Src,
            Action::SetIpDes(_) => ActionType::SetIPv4Des,
            Action::SetTos(_) => ActionType::SetTos,
            Action::SetTpSrc(_) => ActionType::SetTpSrc,
            Action::SetTpDest(_) => ActionType::SetTpDst,
            Action::Enqueue(_, _) => ActionType::Enqueue,
            Action::Unparsable => panic!("Unparse Action to ActionType"),
        }
    }

    /// Returns the total length of the action in bytes
    ///
    /// # Returns
    /// The size of the action including header and payload
    pub fn length(&self) -> usize {
        let header = size_of::<(u16, u16)>();
        let body = match self {
            Action::Oputput(_) => size_of::<(u16, u16)>(),
            Action::SetDlVlan(_) => size_of::<(u16, u16)>(),
            Action::SetDlVlanPcp(_) => size_of::<(u8, [u8; 3])>(),
            Action::SetDlSrc(_) => size_of::<([u8; 6], [u8; 6])>(),
            Action::SetDlDest(_) => size_of::<([u8; 6], [u8; 6])>(),
            Action::SetIpSrc(_) => size_of::<u32>(),
            Action::SetIpDes(_) => size_of::<u32>(),
            Action::SetTos(_) => size_of::<(u8, [u8; 3])>(),
            Action::SetTpSrc(_) => size_of::<(u16, u16)>(),
            Action::SetTpDest(_) => size_of::<(u16, u16)>(),
            Action::Enqueue(_, _) => size_of::<(u16, [u8; 6], u32)>(),
            Action::Unparsable => 0,
        };
        header + body
    }

    /// Serializes the action into a byte buffer
    ///
    /// # Arguments
    /// * `bytes` - Mutable reference to the byte buffer to write to
    pub fn marshal(&self, bytes: &mut Vec<u8>) {
        let _ = bytes.write_u16::<BigEndian>(self.to_action_code() as u16);
        let _ = bytes.write_u16::<BigEndian>(self.length() as u16);
        match self {
            Action::Oputput(pseudo) => {
                pseudo.marshal(bytes);
                let _ = bytes.write_u16::<BigEndian>(match pseudo {
                    PseudoPort::Controller(w) => *w as u16,
                    _ => 0,
                });
            }
            Action::SetDlVlan(None) => {
                let _ = bytes.write_u32::<BigEndian>(0xffff);
            }
            Action::SetDlVlan(Some(vid)) => {
                let _ = bytes.write_u16::<BigEndian>(*vid);
                let _ = bytes.write_u16::<BigEndian>(0);
            }
            Action::SetDlVlanPcp(pcp) => {
                let _ = bytes.write_u8(*pcp);
                for _ in 0..3 {
                    let _ = bytes.write_u8(0);
                }
            }
            Action::SetDlSrc(mac) | Action::SetDlDest(mac) => {
                mac.marshal(bytes);
                MacAddr::from(0).marshal(bytes);
            }
            Action::SetIpSrc(address) | Action::SetIpDes(address) => {
                let _ = bytes.write_u32::<BigEndian>(*address);
            }
            Action::SetTos(n) => {
                let _ = bytes.write_u8(*n);
            }
            Action::SetTpSrc(pt) | Action::SetTpDest(pt) => {
                let _ = bytes.write_u16::<BigEndian>(*pt);
                let _ = bytes.write_u16::<BigEndian>(0);
            }
            Action::Enqueue(pp, qid) => {
                pp.marshal(bytes);
                for _ in 0..6 {
                    let _ = bytes.write_u8(0);
                }
                let _ = bytes.write_u32::<BigEndian>(*qid);
            }
            Action::Unparsable => (),
        }
    }

    /// Parses a sequence of actions from a byte buffer
    ///
    /// # Arguments
    /// * `bytes` - Cursor containing the byte buffer to parse
    ///
    /// # Returns
    /// Vector of parsed actions
    pub fn parse_sequence(bytes: &mut Cursor<Vec<u8>>) -> Vec<Action> {
        if bytes.get_ref().is_empty() {
            vec![]
        } else {
            if let Ok(action) = Action::parse(bytes) {
                let mut v = vec![action];
                v.append(&mut Action::parse_sequence(bytes));
                v
            } else {
                vec![]
            }
        }
    }

    /// Parses a single action from a byte buffer
    ///
    /// # Arguments
    /// * `bytes` - Cursor containing the byte buffer to parse
    ///
    /// # Returns
    /// Result containing either the parsed action or an error
    pub fn parse(bytes: &mut Cursor<Vec<u8>>) -> Result<Action, Error> {
        let action_code = bytes.read_u16::<BigEndian>()?;
        let _ = bytes.read_u16::<BigEndian>()?;
        match action_code {
            t if t == (ActionType::Output as u16) => {
                let port_code = bytes.read_u16::<BigEndian>()?;
                let len = bytes.read_u16::<BigEndian>()?;
                Ok(Action::Oputput(PseudoPort::new(
                    port_code,
                    Some(len as u64),
                )))
            }
            t if t == (ActionType::SetVlanId as u16) => {
                let vid = bytes.read_u16::<BigEndian>()?;
                bytes.consume(2);
                if vid == 0xffff {
                    Ok(Action::SetDlVlan(None))
                } else {
                    Ok(Action::SetDlVlan(Some(vid)))
                }
            }
            t if t == (ActionType::SetVlanPCP as u16) => {
                let pcp = bytes.read_u8()?;
                bytes.consume(3);
                Ok(Action::SetDlVlanPcp(pcp))
            }
            t if t == (ActionType::StripVlan as u16) => {
                bytes.consume(4);
                Ok(Action::SetDlVlan(None))
            }
            t if t == (ActionType::SetSrcMac as u16) => {
                let mut addr = [0u8; 6];
                for i in 0..6 {
                    addr[i] = bytes.read_u8()?;
                }
                bytes.consume(6);
                Ok(Action::SetDlSrc(MacAddr::new(addr)))
            }
            t if t == (ActionType::SetDstMac as u16) => {
                let mut addr = [0u8; 6];
                for i in 0..6 {
                    addr[i] = bytes.read_u8()?;
                }
                bytes.consume(6);
                Ok(Action::SetDlDest(MacAddr::new(addr)))
            }
            t if t == (ActionType::SetIPv4Src as u16) => {
                Ok(Action::SetIpSrc(bytes.read_u32::<BigEndian>()?))
            }
            t if t == (ActionType::SetIPv4Des as u16) => {
                Ok(Action::SetIpDes(bytes.read_u32::<BigEndian>()?))
            }
            t if t == (ActionType::SetTos as u16) => {
                let tos = bytes.read_u8()?;
                bytes.consume(3);
                Ok(Action::SetTos(tos))
            }
            t if t == (ActionType::SetTpSrc as u16) => {
                let pt = bytes.read_u16::<BigEndian>()?;
                bytes.consume(2);
                Ok(Action::SetTpSrc(pt))
            }
            t if t == (ActionType::SetTpDst as u16) => {
                let pt = bytes.read_u16::<BigEndian>()?;
                bytes.consume(2);
                Ok(Action::SetTpDest(pt))
            }
            t if t == (ActionType::Enqueue as u16) => {
                let pt = bytes.read_u16::<BigEndian>()?;
                bytes.consume(6);
                let qid = bytes.read_u32::<BigEndian>()?;
                Ok(Action::Enqueue(PseudoPort::new(pt, Some(0)), qid))
            }
            _ => Ok(Action::Unparsable),
        }
    }
}

/// Trait for checking action sequence sizes and reordering
pub trait SizeCheck {
    /// Returns the total size of an action sequence
    fn size_of_sequence(&self) -> usize;
    /// Moves controller actions to the end of the sequence
    fn move_controller_last(&self) -> Vec<Action>;
}

impl SizeCheck for Vec<Action> {
    /// Calculates the total size of all actions in the sequence
    fn size_of_sequence(&self) -> usize {
        self.iter().fold(0, |acc, x| x.length() + acc)
    }

    /// Reorders actions to ensure controller actions are last
    fn move_controller_last(&self) -> Vec<Action> {
        let mut not_ctrl: Vec<Action> = Vec::new();
        let mut is_ctrl: Vec<Action> = Vec::new();
        for act in self {
            match act {
                Action::Oputput(PseudoPort::Controller(_)) => {
                    is_ctrl.push(act.clone());
                }
                _ => not_ctrl.push(act.clone()),
            }
        }
        not_ctrl.append(&mut is_ctrl);
        not_ctrl
    }
}
