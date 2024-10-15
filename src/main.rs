use tenjin::{example::Controller13, openflow::ofp13::ControllerFrame13};

/**
 * If you prefer to run only Controller without cli.
 * use this instead.
 ```
fn main() {
    let controller = example::Controller13::new();
    controller.listener("127.0.0.1:6633");
}
 ```
 */
#[tokio::main]
async fn main() {
    let controller = Controller13::new();
    controller.listener("127.0.0.1:6653").await;
}
