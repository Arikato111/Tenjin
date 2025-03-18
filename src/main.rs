use tenjin_sdn::cli::cli_system;

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
    cli_system::system().await;
}
