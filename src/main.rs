use tenjin::openflow::messages::Openflow10;
use tenjin::openflow::Controller;

extern crate byteorder;

fn main() -> Result<(), std::io::Error> {
    let mut controller = Controller::new(Openflow10::new());
    controller.listener("127.0.0.1:6633");
    Ok(())
}
