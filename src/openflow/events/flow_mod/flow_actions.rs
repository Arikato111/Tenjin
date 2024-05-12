use std::{
    io::{BufRead, Cursor},
    mem::size_of,
};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::{
    etherparser::tools::bits::{bytes_to_mac, mac_to_bytes},
    openflow::PseudoPort,
};

use super::flow_actions_type::FlowActionType;

pub enum FlowAction {
    Oputput(PseudoPort),
    SetDlVlan(Option<u16>),
    SetDlVlanPcp(u8),
    SetDlSrc(u64),
    SetDlDest(u64),
    SetIpSrc(u32),
    SetIpDes(u32),
    SetTos(u8),
    SetTpSrc(u16),
    SetTpDest(u16),
    Enqueue(PseudoPort, u32),
    Unparsable,
}

impl FlowAction {
    pub fn to_action_code(&self) -> FlowActionType {
        match self {
            FlowAction::Oputput(_) => FlowActionType::Output,
            FlowAction::SetDlVlan(_) => FlowActionType::SetVlanId,
            FlowAction::SetDlVlanPcp(_) => FlowActionType::SetVlanPCP,
            FlowAction::SetDlSrc(_) => FlowActionType::SetSrcMac,
            FlowAction::SetDlDest(_) => FlowActionType::SetDstMac,
            FlowAction::SetIpSrc(_) => FlowActionType::SetIPv4Src,
            FlowAction::SetIpDes(_) => FlowActionType::SetIPv4Des,
            FlowAction::SetTos(_) => FlowActionType::SetTos,
            FlowAction::SetTpSrc(_) => FlowActionType::SetTpSrc,
            FlowAction::SetTpDest(_) => FlowActionType::SetTpDst,
            FlowAction::Enqueue(_, _) => FlowActionType::Enqueue,
            FlowAction::Unparsable => panic!("Unparse FlowAction to FlowActionType"),
        }
    }
    pub fn length(&self) -> usize {
        let header = size_of::<(u16, u16)>();
        let body = match self {
            FlowAction::Oputput(_) => size_of::<(u16, u16)>(),
            FlowAction::SetDlVlan(_) => size_of::<(u16, u16)>(),
            FlowAction::SetDlVlanPcp(_) => size_of::<(u8, [u8; 3])>(),
            FlowAction::SetDlSrc(_) => size_of::<([u8; 6], [u8; 6])>(),
            FlowAction::SetDlDest(_) => size_of::<([u8; 6], [u8; 6])>(),
            FlowAction::SetIpSrc(_) => size_of::<u32>(),
            FlowAction::SetIpDes(_) => size_of::<u32>(),
            FlowAction::SetTos(_) => size_of::<(u8, [u8; 3])>(),
            FlowAction::SetTpSrc(_) => size_of::<(u16, u16)>(),
            FlowAction::SetTpDest(_) => size_of::<(u16, u16)>(),
            FlowAction::Enqueue(_, _) => size_of::<(u16, [u8; 6], u32)>(),
            FlowAction::Unparsable => 0,
        };
        header + body
    }

    pub fn marshal(&self, bytes: &mut Vec<u8>) {
        let _ = bytes.write_u16::<BigEndian>(self.to_action_code() as u16);
        let _ = bytes.write_u16::<BigEndian>(self.length() as u16);
        match self {
            FlowAction::Oputput(pseudo) => {
                pseudo.marshal(bytes);
                let _ = bytes.write_u16::<BigEndian>(match pseudo {
                    PseudoPort::Controller(w) => *w as u16,
                    _ => 0,
                });
            }
            FlowAction::SetDlVlan(None) => {
                let _ = bytes.write_u32::<BigEndian>(0xffff);
            }
            FlowAction::SetDlVlan(Some(vid)) => {
                let _ = bytes.write_u16::<BigEndian>(*vid);
                let _ = bytes.write_u16::<BigEndian>(0);
            }
            FlowAction::SetDlVlanPcp(pcp) => {
                let _ = bytes.write_u8(*pcp);
                for _ in 0..3 {
                    let _ = bytes.write_u8(0);
                }
            }
            FlowAction::SetDlSrc(mac) | FlowAction::SetDlDest(mac) => {
                let mac = bytes_to_mac(*mac);
                for m in mac {
                    let _ = bytes.write_u8(m);
                }
                for _ in 0..6 {
                    let _ = bytes.write_u8(0);
                }
            }
            FlowAction::SetIpSrc(address) | FlowAction::SetIpDes(address) => {
                let _ = bytes.write_u32::<BigEndian>(*address);
            }
            FlowAction::SetTos(n) => {
                let _ = bytes.write_u8(*n);
            }
            FlowAction::SetTpSrc(pt) | FlowAction::SetTpDest(pt) => {
                let _ = bytes.write_u16::<BigEndian>(*pt);
                let _ = bytes.write_u16::<BigEndian>(0);
            }
            FlowAction::Enqueue(pp, qid) => {
                pp.marshal(bytes);
                for _ in 0..6 {
                    let _ = bytes.write_u8(0);
                }
                let _ = bytes.write_u32::<BigEndian>(*qid);
            }
            FlowAction::Unparsable => todo!(),
        }
    }
    pub fn parse_sequence(bytes: &mut Cursor<Vec<u8>>) -> Vec<FlowAction> {
        if bytes.get_ref().is_empty() {
            vec![]
        } else {
            let action = FlowAction::parse(bytes);
            let mut v = vec![action];
            v.append(&mut FlowAction::parse_sequence(bytes));
            v
        }
    }

    pub fn parse(bytes: &mut Cursor<Vec<u8>>) -> FlowAction {
        let action_code = bytes.read_u16::<BigEndian>().unwrap();
        let _ = bytes.read_u16::<BigEndian>().unwrap();
        match action_code {
            t if t == (FlowActionType::Output as u16) => {
                let port_code = bytes.read_u16::<BigEndian>().unwrap();
                let len = bytes.read_u16::<BigEndian>().unwrap();
                FlowAction::Oputput(PseudoPort::new(port_code, Some(len as u64)))
            }
            t if t == (FlowActionType::SetVlanId as u16) => {
                let vid = bytes.read_u16::<BigEndian>().unwrap();
                bytes.consume(2);
                if vid == 0xffff {
                    FlowAction::SetDlVlan(None)
                } else {
                    FlowAction::SetDlVlan(Some(vid))
                }
            }
            t if t == (FlowActionType::SetVlanPCP as u16) => {
                let pcp = bytes.read_u8().unwrap();
                bytes.consume(3);
                FlowAction::SetDlVlanPcp(pcp)
            }
            t if t == (FlowActionType::StripVlan as u16) => {
                bytes.consume(4);
                FlowAction::SetDlVlan(None)
            }
            t if t == (FlowActionType::SetSrcMac as u16) => {
                let mut addr = [0u8; 6];
                for i in 0..6 {
                    addr[i] = bytes.read_u8().unwrap();
                }
                bytes.consume(6);
                FlowAction::SetDlSrc(mac_to_bytes(addr))
            }
            t if t == (FlowActionType::SetDstMac as u16) => {
                let mut addr = [0u8; 6];
                for i in 0..6 {
                    addr[i] = bytes.read_u8().unwrap();
                }
                bytes.consume(6);
                FlowAction::SetDlDest(mac_to_bytes(addr))
            }
            t if t == (FlowActionType::SetIPv4Src as u16) => {
                FlowAction::SetIpSrc(bytes.read_u32::<BigEndian>().unwrap())
            }
            t if t == (FlowActionType::SetIPv4Des as u16) => {
                FlowAction::SetIpDes(bytes.read_u32::<BigEndian>().unwrap())
            }
            t if t == (FlowActionType::SetTos as u16) => {
                let tos = bytes.read_u8().unwrap();
                bytes.consume(3);
                FlowAction::SetTos(tos)
            }
            t if t == (FlowActionType::SetTpSrc as u16) => {
                let pt = bytes.read_u16::<BigEndian>().unwrap();
                bytes.consume(2);
                FlowAction::SetTpSrc(pt)
            }
            t if t == (FlowActionType::SetTpDst as u16) => {
                let pt = bytes.read_u16::<BigEndian>().unwrap();
                bytes.consume(2);
                FlowAction::SetTpDest(pt)
            }
            t if t == (FlowActionType::Enqueue as u16) => {
                let pt = bytes.read_u16::<BigEndian>().unwrap();
                bytes.consume(6);
                let qid = bytes.read_u32::<BigEndian>().unwrap();
                FlowAction::Enqueue(PseudoPort::new(pt, Some(0)), qid)
            }
            _ => FlowAction::Unparsable,
        }
    }
}

pub trait SizeCheck {
    fn size_of_sequence(&self) -> usize;
    fn move_controller_last(&self) -> Vec<FlowAction>;
}

impl SizeCheck for Vec<FlowAction> {
    fn size_of_sequence(&self) -> usize {
        self.iter().fold(0, |acc, x| x.length() + acc)
    }

    fn move_controller_last(&self) -> Vec<FlowAction> {
        let mut not_ctrl: Vec<FlowAction> = Vec::new();
        let mut is_ctrl: Vec<FlowAction> = Vec::new();
        for act in self {
            match act {
                FlowAction::Oputput(PseudoPort::Controller(_)) => {
                    is_ctrl.push(act.clone());
                }
                _ => not_ctrl.push(act.clone()),
            }
        }
        not_ctrl.append(&mut is_ctrl);
        not_ctrl
    }
}

impl Clone for FlowAction {
    fn clone(&self) -> Self {
        match self {
            Self::Oputput(v) => Self::Oputput(v.clone()),
            Self::SetDlVlan(v) => Self::SetDlVlan(v.clone()),
            Self::SetDlVlanPcp(v) => Self::SetDlVlanPcp(v.clone()),
            Self::SetDlSrc(v) => Self::SetDlSrc(v.clone()),
            Self::SetDlDest(v) => Self::SetDlDest(v.clone()),
            Self::SetIpSrc(v) => Self::SetIpSrc(v.clone()),
            Self::SetIpDes(v) => Self::SetIpDes(v.clone()),
            Self::SetTos(v) => Self::SetTos(v.clone()),
            Self::SetTpSrc(v) => Self::SetTpSrc(v.clone()),
            Self::SetTpDest(v) => Self::SetTpDest(v.clone()),
            Self::Enqueue(v, arg1) => Self::Enqueue(v.clone(), arg1.clone()),
            Self::Unparsable => Self::Unparsable,
        }
    }
}
