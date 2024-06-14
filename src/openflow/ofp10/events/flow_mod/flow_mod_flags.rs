use byteorder::{BigEndian, WriteBytesExt};

pub struct FlowModFlags {
    pub send_flow_rem: bool,
    pub check_overlap: bool,
    pub emerg: bool,
}

impl FlowModFlags {
    pub fn new(send_flow_rem: bool, check_overlap: bool, emerg: bool) -> Self {
        Self {
            send_flow_rem,
            check_overlap,
            emerg,
        }
    }
    pub fn all_false() -> Self {
        Self {
            check_overlap: false,
            emerg: false,
            send_flow_rem: false,
        }
    }
    pub fn parse(byte: u16) -> Self {
        let send_flow_rem = byte >> 0 & 1 != 0;
        let check_overlap = byte >> 1 & 1 != 0;
        let emerg = byte >> 2 & 1 != 0;
        Self {
            send_flow_rem,
            check_overlap,
            emerg,
        }
    }
    pub fn marshal(&self, bytes: &mut Vec<u8>) {
        let mut value = 0u16;
        if self.send_flow_rem {
            value |= 1 << 0;
        }
        if self.check_overlap {
            value |= 1 << 1;
        }
        if self.emerg {
            value |= 1 << 2;
        }
        let _ = bytes.write_u16::<BigEndian>(value);
    }
}
