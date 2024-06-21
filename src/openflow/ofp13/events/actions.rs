use std::{
    io::{BufRead, Cursor, Error},
    mem::size_of,
};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::{
    etherparser::tools::bits::{bytes_to_mac, mac_to_bytes},
    openflow::ofp13::PseudoPort,
};

pub enum ActionType {
    Output = 0,
    SetVlanId = 1,
    SetVlanPCP = 2,
    StripVlan = 3,
    SetSrcMac = 4,
    SetDstMac = 5,
    SetIPv4Src = 6,
    SetIPv4Des = 7,
    SetTos = 8,
    SetTpSrc = 9,
    SetTpDst = 10,
    Enqueue = 11,
}

#[derive(Clone)]
pub enum Action {
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

impl Action {
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
                let mac = bytes_to_mac(*mac);
                for m in mac {
                    let _ = bytes.write_u8(m);
                }
                for _ in 0..6 {
                    let _ = bytes.write_u8(0);
                }
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

    pub fn parse(bytes: &mut Cursor<Vec<u8>>) -> Result<Action, Error> {
        let action_code = bytes.read_u16::<BigEndian>()?;
        let _ = bytes.read_u16::<BigEndian>()?;
        match action_code {
            t if t == (ActionType::Output as u16) => {
                let port_code = bytes.read_u32::<BigEndian>()?;
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
                Ok(Action::SetDlSrc(mac_to_bytes(addr)))
            }
            t if t == (ActionType::SetDstMac as u16) => {
                let mut addr = [0u8; 6];
                for i in 0..6 {
                    addr[i] = bytes.read_u8()?;
                }
                bytes.consume(6);
                Ok(Action::SetDlDest(mac_to_bytes(addr)))
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
                let pt = bytes.read_u32::<BigEndian>()?;
                bytes.consume(6);
                let qid = bytes.read_u32::<BigEndian>()?;
                Ok(Action::Enqueue(PseudoPort::new(pt, Some(0)), qid))
            }
            _ => Ok(Action::Unparsable),
        }
    }
}

pub trait SizeCheck {
    fn size_of_sequence(&self) -> usize;
    fn move_controller_last(&self) -> Vec<Action>;
}

impl SizeCheck for Vec<Action> {
    fn size_of_sequence(&self) -> usize {
        self.iter().fold(0, |acc, x| x.length() + acc)
    }

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
