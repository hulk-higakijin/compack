use clap::Parser;
use compack::cli::{Cli, Commands};
use compack::error::exit_with_error;
use compack::handlers::{handle_init, handle_query};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Query { command }) => {
            handle_query(&command);
        }
        Some(Commands::Init { shell }) => {
            handle_init(shell.as_deref());
        }
        None => {
            exit_with_error("No command specified. Use --help for usage information.");
        }
    }
}
