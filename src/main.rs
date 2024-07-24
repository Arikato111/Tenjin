use tenjin::example;
use tenjin::openflow::ofp13::ControllerFrame13;
extern crate byteorder;

fn main() -> Result<(), std::io::Error> {
    let controller = example::Controller13::new();
    controller.listener("127.0.0.1:6633");
    Ok(())
}
