#![cfg(feature = "full")]
#[cfg(test)]
mod tests {
    use tenjin_sdn::example::Controller10;
    use tenjin_sdn::openflow::ofp10::ControllerFrame10;
    use tenjin_sdn::openflow::ofp10::{Msg, OfpMsgEvent, OpenflowHeader};

    #[test]
    fn test_header_v1_0_parser() {
        let ofp_header_bytes: Vec<u8> = vec![1, 0, 0, 8, 0, 0, 0, 1];

        let controller = Controller10::new();
        let ofp = controller.ofp();
        let header = match ofp.header_parse(&ofp_header_bytes) {
            Ok(v) => v,
            Err(_) => panic!("cannot parse ofp header"),
        };

        assert_eq!(header.version(), 1);
        assert_eq!(header.message(), 0);
        assert_eq!(header.length(), 8);
        assert_eq!(header.xid(), 1);
    }
    #[test]
    fn test_header_v1_0_marshal() {
        let ofp_header_bytes: Vec<u8> = vec![1, 0, 0, 8, 0, 0, 0, 0];

        let controller = Controller10::new();
        let ofp = controller.ofp();
        let ofp_header = ofp.header(ofp.msg_usize(Msg::Hello) as u8, 0, 0);
        let mut bytes: Vec<u8> = Vec::new();
        ofp_header.marshal(&mut bytes);

        assert_eq!(ofp_header_bytes, bytes);
    }
}
