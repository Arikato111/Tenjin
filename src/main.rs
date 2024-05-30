use tenjin::{
    openflow::{controller_frame::ControllerFrame, ofp10::ofp_v1_0::Openflow10},
    Controller,
};

extern crate byteorder;

fn main() -> Result<(), std::io::Error> {
    // Controller::listener("127.0.0.1:6633", Openflow10::new());
    // tcp_listener_handler(Controller::new(Openflow10), "127.0.0.1:6633");
    Controller::listener("127.0.0.1:6633", Openflow10::new());
    Ok(())
}
