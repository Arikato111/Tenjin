use tenjin::openflow::messages::Openflow10;
use tenjin::openflow::Controller;

extern crate byteorder;

fn main() -> Result<(), std::io::Error> {
    Controller::listener("127.0.0.1:6633", Openflow10::new());
    Ok(())
}
