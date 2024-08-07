use clap::{command, Parser, Subcommand};

use crate::{
    example::{Controller10, Controller13},
    openflow::{ofp10::ControllerFrame10, ofp13::ControllerFrame13},
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run the controller
    Run {
        #[command(subcommand)]
        controller: Option<Controllers>,
        #[arg(default_value_t = 6633, short = 'p', long)]
        port: u16,
        #[arg(
            default_value = "127.0.0.1",
            short = 'a',
            long = "address",
            help = "ip address"
        )]
        address: String,
    },
}

#[derive(Subcommand)]
enum Controllers {
    /// Openflow 1.3 with Controller13
    Ctrl13,
    /// Openflow 1.0 with Controller10
    Ctrl10,
}

pub fn system() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Run {
            controller,
            port,
            address,
        } => {
            let addr = format!("{}:{}", address.as_str(), port.to_string());
            match controller {
                Some(controller) => match controller {
                    Controllers::Ctrl13 => Controller13::new().listener(&addr),
                    Controllers::Ctrl10 => Controller10::new().listener(&addr),
                },
                // Set Default Controller at here
                None => Controller13::new().listener(&addr),
            }
        }
    }
}
