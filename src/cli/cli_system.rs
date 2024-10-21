use crate::{
    example::{Controller10, Controller13},
    openflow::{ofp10::ControllerFrame10, ofp13::ControllerFrame13},
};
use clap::{command, CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Shell};
use std::io;

#[derive(Parser)]
#[command(name = "tenjin",author, version, about, long_about = None)]
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
    /// Generate auto complete for shell
    Completions { shell: Shell },
}

#[derive(Subcommand, Clone)]
pub enum Controllers {
    /// Openflow 1.3 with Controller13
    Ctrl13,
    /// Openflow 1.0 with Controller10
    Ctrl10,
}

pub async fn system() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Run {
            controller,
            port,
            listen,
        } => {
            let mut thread_list = Vec::new();
            for p in port.iter() {
                let addr = format!("{}:{}", listen, p);
                let controller = controller.clone();
                let t = tokio::spawn(async move {
                    match controller {
                        Some(controller) => match controller {
                            Controllers::Ctrl13 => Controller13::new().listener(&addr).await,
                            Controllers::Ctrl10 => Controller10::new().listener(&addr).await,
                        },
                        // Set Default Controller at here
                        None => Controller13::new().listener(&addr).await,
                    }
                });
                thread_list.push(t);
            }
            for th in thread_list {
                let _ = th.await;
            }
        }
        Commands::Completions { shell } => {
            let mut cli_gen = Cli::command();
            generate(shell, &mut cli_gen, "tenjin", &mut io::stdout());
        }
    }
}
