use tenjin::{openflow::ofp10::ControllerFrame10, Controller};

extern crate byteorder;

fn main() -> Result<(), std::io::Error> {
    let controller = Controller::new();
    controller.listener("127.0.0.1:6633");
    Ok(())
}
