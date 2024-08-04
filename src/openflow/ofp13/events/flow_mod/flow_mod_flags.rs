use byteorder::{BigEndian, WriteBytesExt};

pub struct FlowModFlags {
    pub send_flow_rem: bool,
    pub check_overlap: bool,
    pub reset_counts: bool,
    pub no_pkt_counts: bool,
    pub no_byt_counts: bool,
}

impl FlowModFlags {
    pub fn new(
        send_flow_rem: bool,
        check_overlap: bool,
        reset_counts: bool,
        no_pkt_counts: bool,
        no_byt_counts: bool,
    ) -> Self {
        Self {
            send_flow_rem,
            check_overlap,
            reset_counts,
            no_pkt_counts,
            no_byt_counts,
        }
    }
    pub fn all_false() -> Self {
        Self {
            send_flow_rem: false,
            check_overlap: false,
            reset_counts: false,
            no_pkt_counts: false,
            no_byt_counts: false,
        }
    }
    pub fn parse(byte: u16) -> Self {
        let send_flow_rem = byte >> 0 & 1 != 0;
        let check_overlap = byte >> 1 & 1 != 0;
        let reset_counts = byte >> 2 & 1 != 0;
        let no_pkt_counts = byte >> 3 & 1 != 0;
        let no_byt_counts = byte >> 4 & 1 == 1;

        Self {
            send_flow_rem,
            check_overlap,
            reset_counts,
            no_pkt_counts,
            no_byt_counts,
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
        if self.reset_counts {
            value |= 1 << 2;
        }
        if self.no_pkt_counts {
            value |= 1 << 3;
        }
        if self.no_byt_counts {
            value |= 1 << 4;
        }
        let _ = bytes.write_u16::<BigEndian>(value);
    }
}
