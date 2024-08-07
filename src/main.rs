use tenjin::cli;

extern crate byteorder;

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
fn main() {
    cli::system();
}
