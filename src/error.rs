use std::fmt;
use std::process;

/// Custom error type for compack CLI operations
#[derive(Debug)]
pub enum CliError {
    ConfigDirNotFound(String),
    ConfigLoadFailed(String),
    ConfigInitFailed(String),
    UnsupportedShell(String),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CliError::ConfigDirNotFound(msg) => write!(f, "Config directory error: {}", msg),
            CliError::ConfigLoadFailed(msg) => write!(f, "Failed to load config: {}", msg),
            CliError::ConfigInitFailed(msg) => write!(f, "Failed to initialize config: {}", msg),
            CliError::UnsupportedShell(msg) => write!(f, "Unsupported shell: {}", msg),
        }
    }
}

impl std::error::Error for CliError {}

/// Exit the program with an error message
pub fn exit_with_error(error: impl fmt::Display) -> ! {
    eprintln!("Error: {}", error);
    process::exit(1);
}

/// Exit the program with a success message
pub fn exit_success() -> ! {
    process::exit(0);
}
