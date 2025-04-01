//! Tenjin SDN Controller
//! 
//! This is the main entry point for the Tenjin Software-Defined Networking (SDN) controller.
//! The application provides both a CLI interface and direct controller initialization options.

use tenjin_sdn::cli::cli_system;

/**
 * Alternative entry point for running the controller without CLI
 * 
 * This example shows how to directly initialize and run the OpenFlow 1.3 controller
 * on a specific address and port. This is useful for embedded or automated scenarios
 * where CLI interaction is not needed.
 * 
 * Usage:
 * ```rust
 * fn main() {
 *     let controller = example::Controller13::new();
 *     controller.listener("127.0.0.1:6633");
 * }
 * ```
 */
#[tokio::main]
async fn main() {
    // Initialize and run the CLI system
    // This will parse command line arguments and execute the appropriate controller
    if let Err(e) = cli_system::system().await {
        eprintln!("Error running CLI system: {}", e);
        std::process::exit(1);
    }
}
