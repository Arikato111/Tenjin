use super::OpenflowHeader;

pub struct OfpHeader<OFH: OpenflowHeader> {
    ofp_header: OFH,
}

impl<OFH: OpenflowHeader> OfpHeader<OFH> {
    pub fn new(ofp_header: OFH) -> Self {
        Self { ofp_header }
    }
    pub fn version(&self) -> usize {
        self.ofp_header.version()
    }
    pub fn message(&self) -> u8 {
        self.ofp_header.message()
    }
    pub fn length(&self) -> usize {
        self.ofp_header.length()
    }
    pub fn xid(&self) -> u32 {
        self.ofp_header.xid()
    }
    pub fn header_size(&self) -> usize {
        self.ofp_header.header_size()
    }
    pub fn pkt_size(&self) -> usize {
        self.ofp_header.pkt_size()
    }
    pub fn marshal(&self, bytes: &mut Vec<u8>) {
        self.ofp_header.marshal(bytes);
    }
}
