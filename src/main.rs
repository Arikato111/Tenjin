use tenjin::{openflow::controller_frame::ControllerFrame, Controller};

extern crate byteorder;

fn main() -> Result<(), std::io::Error> {
    Controller::listener("127.0.0.1:6633");
    Ok(())
}
