use crate::{
    example::{Controller10, Controller13},
    openflow::{ofp10::ControllerFrame10, ofp13::ControllerFrame13},
};
use clap::{command, Parser, Subcommand};
use std::thread;

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
        #[arg(default_values_t = [6633,6653], short = 'p', long, value_delimiter = ',')]
        port: Vec<u16>,
        #[arg(
            default_value = "127.0.0.1",
            short = 'l',
            long = "listen",
            value_name = "ADDRESS",
            help = "ip address"
        )]
        listen: String,
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
            listen,
        } => {
            // creat runner function to run inside thread spawn
            let runner = match controller {
                Some(controller) => match controller {
                    Controllers::Ctrl13 => |addr: &str| Controller13::new().listener(addr),
                    Controllers::Ctrl10 => |addr: &str| Controller10::new().listener(addr),
                },
                // Set Default Controller at here
                None => |addr: &str| Controller13::new().listener(addr),
            };
            // spawn and run threads
            let mut thread_list = Vec::new();
            for p in port.iter() {
                let addr = format!("{}:{}", listen, p);
                let t = thread::spawn(move || {
                    runner(&addr);
                });
                thread_list.push(t);
            }
            for th in thread_list {
                let _ = th.join();
            }
        }
    }
}
