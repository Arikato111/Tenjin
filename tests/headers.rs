#[cfg(test)]
mod tests {
    use tenjin::{
        openflow::{
            controller_frame::ControllerFrame,
            ofp_manager::{OfpMsg, OfpMsgEvent},
            ofp_v1_0::Openflow10,
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
    #[test]
    fn test_header_v1_0_marshal() {
        let ofp_header_bytes: Vec<u8> = vec![1, 0, 0, 8, 0, 0, 0, 0];

        let controller = Controller::new(Openflow10::new());
        let ofp = controller.get_ofp();
        let ofp_header = ofp.header(ofp.msg_usize(OfpMsg::Hello) as u8, 0, 0);
        let mut bytes: Vec<u8> = Vec::new();
        ofp_header.marshal(&mut bytes);

        assert_eq!(ofp_header_bytes, bytes);
    }
}
