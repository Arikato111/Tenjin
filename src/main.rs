use tenjin::{openflow::ofp10::ControllerFrame10, Controller};

extern crate byteorder;

fn main() -> Result<(), std::io::Error> {
    Controller::listener("127.0.0.1:6653");
    Ok(())
}
