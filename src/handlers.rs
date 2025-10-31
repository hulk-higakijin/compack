use crate::error::{exit_success, exit_with_error, CliError};
use crate::shell::Shell;
use crate::Config;
use std::path::PathBuf;

/// Load the config directory, exiting with error if it fails
fn get_config_dir() -> PathBuf {
    Config::default_config_dir()
        .unwrap_or_else(|e| exit_with_error(CliError::ConfigDirNotFound(e.to_string())))
}

/// Load the config from the given directory, exiting with error if it fails
fn load_config(config_dir: &PathBuf) -> Config {
    // Check if config directory exists
    if !config_dir.exists() {
        exit_with_error("Config directory not found. Run 'compack init' first.");
    }

    // Load config
    Config::load(config_dir)
        .unwrap_or_else(|e| exit_with_error(CliError::ConfigLoadFailed(e.to_string())))
}

/// Handle the query subcommand
pub fn handle_query(command: &str) {
    let config_dir = get_config_dir();
    let config = load_config(&config_dir);

    // Get subcommands
    match config.get_subcommands(command) {
        Some(subcommands) => {
            for subcommand in subcommands {
                println!("{}", subcommand);
            }
        }
        None => {
            // No subcommands found - this is not an error, just return nothing
            exit_success();
        }
    }
}

/// Handle shell integration output
fn handle_shell_integration(shell: &str) {
    let shell = Shell::from_str(shell).unwrap_or_else(|e| exit_with_error(e));
    println!("{}", shell.integration_script());
}

/// Handle config directory initialization
fn handle_config_init() {
    let config_dir = get_config_dir();

    // Check if config already exists
    if config_dir.exists() {
        eprintln!("Config directory already exists at: {}", config_dir.display());
        eprintln!("To reinitialize, please delete the directory first.");
        exit_with_error("");
    }

    // Copy bundled command files
    Config::copy_bundled_commands(&config_dir)
        .unwrap_or_else(|e| exit_with_error(CliError::ConfigInitFailed(e.to_string())));

    // Load the config to display what was created
    let config = load_config(&config_dir);

    display_init_success(&config_dir, &config);
}

/// Display initialization success message
fn display_init_success(config_dir: &PathBuf, config: &Config) {
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

/// Handle the init subcommand
pub fn handle_init(shell: Option<&str>) {
    match shell {
        Some(shell_name) => handle_shell_integration(shell_name),
        None => handle_config_init(),
    }
}
