use tenjin::{example, openflow::ofp10::ControllerFrame10};
extern crate byteorder;

fn main() -> Result<(), std::io::Error> {
    let controller = example::Controller10::new();
    controller.listener("127.0.0.1:6633");
    Ok(())
}
