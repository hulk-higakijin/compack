use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "compack")]
#[command(author, version, about = "Universal command completion system", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Query subcommands for a given command
    Query {
        /// Command to query subcommands for (e.g., "opencode", "cargo", "rails")
        command: String,
    },

    /// Initialize compack
    Init {
        /// Shell to initialize for
        shell: Option<String>,
    },
}
