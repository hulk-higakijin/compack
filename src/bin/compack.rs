use clap::{Parser, Subcommand};
use compack::Config;
use std::process;

#[derive(Parser)]
#[command(name = "compack")]
#[command(author, version, about = "Universal command completion system", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
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

const ZSH_INTEGRATION: &str = r#"# compack zsh integration
_compack_completion_or_space() {
    # First insert the space
    zle self-insert

    # Get the command (first word before space)
    local cmd="${LBUFFER%% *}"

    # Only trigger if we have exactly one word and just typed a space
    if [[ "$LBUFFER" == "$cmd " && "$cmd" != "" ]]; then
        # Check if this command has subcommands defined
        local candidates=$(compack query "$cmd" 2>/dev/null)

        if [[ -n "$candidates" ]]; then
            # Add a special option to run without subcommand
            local all_options="[Run without subcommand]"$'\n'"$candidates"
            
            # Command is defined, open fzf
            local selected=$(echo "$all_options" | fzf --height 40% --reverse --prompt="$cmd > " --bind=tab:down,shift-tab:up)

            if [[ -n "$selected" ]]; then
                if [[ "$selected" == "[Run without subcommand]" ]]; then
                    # User wants to run the command without subcommand
                    LBUFFER="${cmd}"
                    # Execute the command immediately
                    zle accept-line
                else
                    # User selected a subcommand
                    LBUFFER="${cmd} ${selected}"
                    # Execute the command immediately
                    zle accept-line
                fi
            else
                # If nothing selected (ESC pressed), LBUFFER stays as "${cmd} " with the space
                # Redraw the prompt to show the current buffer
                zle reset-prompt
            fi
        fi
        # If no candidates (command not defined), just keep the space as normal input
    fi
}

zle -N _compack_completion_or_space
bindkey ' ' _compack_completion_or_space
"#;

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
            eprintln!("No command specified. Use --help for usage information.");
            process::exit(1);
        }
    }
}

fn handle_query(command: &str) {
    // Get config directory path
    let config_dir = match Config::default_config_dir() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Error: Failed to get config directory: {}", e);
            process::exit(1);
        }
    };

    // Check if config directory exists
    if !config_dir.exists() {
        eprintln!("Error: Config directory not found. Run 'compack init' first.");
        process::exit(1);
    }

    // Load config
    let config = match Config::load(&config_dir) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error: Failed to load config: {}", e);
            process::exit(1);
        }
    };

    // Get subcommands
    match config.get_subcommands(command) {
        Some(subcommands) => {
            for subcommand in subcommands {
                println!("{}", subcommand);
            }
        }
        None => {
            // No subcommands found - this is not an error, just return nothing
            process::exit(0);
        }
    }
}

fn handle_init(shell: Option<&str>) {
    match shell {
        Some("zsh") => {
            // Output zsh integration script
            println!("{}", ZSH_INTEGRATION);
        }
        Some(other) => {
            eprintln!("Error: Shell '{}' is not supported yet. Currently supported: zsh", other);
            process::exit(1);
        }
        None => {
            // Initialize config directory
            let config_dir = match Config::default_config_dir() {
                Ok(path) => path,
                Err(e) => {
                    eprintln!("Error: Failed to get config directory: {}", e);
                    process::exit(1);
                }
            };

            // Check if config already exists
            if config_dir.exists() {
                eprintln!("Config directory already exists at: {}", config_dir.display());
                eprintln!("To reinitialize, please delete the directory first.");
                process::exit(1);
            }

            // Copy bundled command files
            if let Err(e) = Config::copy_bundled_commands(&config_dir) {
                eprintln!("Error: Failed to copy bundled commands: {}", e);
                process::exit(1);
            }

            // Load the config to display what was created
            let config = match Config::load(&config_dir) {
                Ok(config) => config,
                Err(e) => {
                    eprintln!("Error: Failed to load config: {}", e);
                    process::exit(1);
                }
            };

            println!("Created config directory: {}", config_dir.display());
            println!();
            println!("Created default command files:");
            for command_name in config.commands.keys() {
                println!("  - {}.toml", command_name);
            }
            println!();
            println!("To enable zsh integration, add the following to your .zshrc:");
            println!();
            println!("  eval \"$(compack init zsh)\"");
        }
    }
}
