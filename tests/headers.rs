#[cfg(test)]
mod tests {
    use tenjin::{
        openflow::{
            controller_frame::ControllerFrame, ofp_manager::OfpMsgEvent, ofp_v1_0::Openflow10,
        },
        Controller,
    };

    #[test]
    fn test_header_v1_0_parser() {
        let ofp_header_bytes: Vec<u8> = vec![1, 0, 0, 8, 0, 0, 0, 1];

        let controller = Controller::new(Openflow10::new());
        let ofp = controller.get_ofp();

        let header = ofp.header_parse(&ofp_header_bytes);

        assert_eq!(header.version(), 1);
        assert_eq!(header.message(), 0);
        assert_eq!(header.length(), 8);
        assert_eq!(header.xid(), 1);
    }
}
