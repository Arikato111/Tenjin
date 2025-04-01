//! CLI System for Tenjin SDN Controller
//!
//! This module provides the command-line interface for the Tenjin SDN controller,
//! allowing users to run different controller versions and manage the application.

use crate::{
    example::{Controller10, Controller13},
    openflow::{ofp10::ControllerFrame10, ofp13::ControllerFrame13},
};
use clap::{command, CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Shell};
use std::{io, net::SocketAddr};
use tokio::task::JoinHandle;

/// Type alias for error handling across the CLI system
type Error = Box<dyn std::error::Error + Send + Sync>;

/// Main CLI command structure
#[derive(Parser)]
#[command(name = "tenjin", author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// Available CLI commands
#[derive(Subcommand)]
enum Commands {
    /// Run the controller
    Run {
        #[command(subcommand)]
        controller: Option<Controllers>,
        /// Ports to listen on (default: 6633,6653)
        #[arg(default_values_t = [6633,6653], short = 'p', long, value_delimiter = ',')]
        port: Vec<u16>,
        /// IP address to listen on
        #[arg(
            default_value = "127.0.0.1",
            short = 'l',
            long = "listen",
            value_name = "ADDRESS",
            help = "ip address"
        )]
        listen: String,
    },
    /// Generate shell completion scripts
    Completions { shell: Shell },
}

/// Available controller versions
#[derive(Subcommand, Clone)]
pub enum Controllers {
    /// Openflow 1.3 with Controller13
    Ctrl13,
    /// Openflow 1.0 with Controller10
    Ctrl10,
}

/// Runs a controller instance on the specified address
///
/// # Arguments
/// * `addr` - The socket address to listen on
/// * `controller` - The controller version to run (defaults to Ctrl13 if None)
async fn run_controller(addr: SocketAddr, controller: Option<Controllers>) -> Result<(), Error> {
    let controller = controller.unwrap_or(Controllers::Ctrl13);
    match controller {
        Controllers::Ctrl13 => Ok(Controller13::new().listener(&addr.to_string()).await),
        Controllers::Ctrl10 => Ok(Controller10::new().listener(&addr.to_string()).await),
    }
}

/// Generates shell completion scripts for the CLI
///
/// # Arguments
/// * `shell` - The shell type to generate completions for
async fn handle_completions(shell: Shell) -> Result<(), Error> {
    let mut cli_gen = Cli::command();
    generate(shell, &mut cli_gen, "tenjin", &mut io::stdout());
    Ok(())
}

/// Main entry point for the CLI system
///
/// This function parses command line arguments and executes the appropriate command.
/// For the run command, it spawns controller instances for each specified port.
pub async fn system() -> Result<(), Error> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run {
            controller,
            port,
            listen,
        } => {
            // Pre-allocate vector for better performance
            let mut handles: Vec<JoinHandle<Result<(), Error>>> = Vec::with_capacity(port.len());

            // Spawn controller instances for each port
            for p in port.iter() {
                let addr = format!("{}:{}", listen, p)
                    .parse::<SocketAddr>()
                    .map_err(|e| format!("Invalid address: {}", e))?;

                let controller = controller.clone();
                let handle = tokio::spawn(async move { run_controller(addr, controller).await });
                handles.push(handle);
            }

            // Wait for all controller instances to complete
            for handle in handles {
                handle.await??;
            }
        }
        Commands::Completions { shell } => {
            handle_completions(shell).await?;
        }
    }

    Ok(())
}
