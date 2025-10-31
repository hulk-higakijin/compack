use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub commands: HashMap<String, CommandConfig>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CommandConfig {
    pub subcommands: Vec<String>,
}

impl Config {
    /// Load configuration from a directory containing TOML files
    pub fn load(dir_path: &PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let mut commands = HashMap::new();

        // Check if directory exists
        if !dir_path.exists() {
            return Err(format!("Config directory not found: {}", dir_path.display()).into());
        }

        // Read all .toml files in the directory
        for entry in fs::read_dir(dir_path)? {
            let entry = entry?;
            let path = entry.path();

            // Only process .toml files
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("toml") {
                let content = fs::read_to_string(&path)?;
                let command_config: CommandConfig = toml::from_str(&content)?;

                // Use filename (without extension) as command name
                if let Some(command_name) = path.file_stem().and_then(|s| s.to_str()) {
                    commands.insert(command_name.to_string(), command_config);
                }
            }
        }

        Ok(Config { commands })
    }

    /// Get default config directory path (project_root/commands/)
    pub fn default_config_dir() -> Result<PathBuf, io::Error> {
        // Use environment variable if set, otherwise use current directory
        if let Ok(path) = std::env::var("COMPACK_CONFIG_DIR") {
            return Ok(PathBuf::from(path));
        }

        // Use current directory + commands
        let current_dir = std::env::current_dir()?;
        Ok(current_dir.join("commands"))
    }

    /// Create default configuration with example commands
    pub fn default() -> Self {
        let mut commands = HashMap::new();

        commands.insert(
            "opencode".to_string(),
            CommandConfig {
                subcommands: vec![
                    "acp".to_string(),
                    "attach".to_string(),
                    "run".to_string(),
                    "auth".to_string(),
                    "agent".to_string(),
                    "upgrade".to_string(),
                    "serve".to_string(),
                    "models".to_string(),
                    "export".to_string(),
                    "github".to_string(),
                ],
            },
        );

        commands.insert(
            "cargo".to_string(),
            CommandConfig {
                subcommands: vec![
                    "build".to_string(),
                    "run".to_string(),
                    "test".to_string(),
                    "check".to_string(),
                    "clean".to_string(),
                    "doc".to_string(),
                ],
            },
        );

        commands.insert(
            "rails".to_string(),
            CommandConfig {
                subcommands: vec![
                    "new".to_string(),
                    "server".to_string(),
                    "console".to_string(),
                    "generate".to_string(),
                    "db:migrate".to_string(),
                    "routes".to_string(),
                ],
            },
        );

        Config { commands }
    }

    /// Save configuration to a directory (each command as a separate file)
    pub fn save(&self, dir_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        // Create directory if it doesn't exist
        fs::create_dir_all(dir_path)?;

        // Save each command as a separate file
        for (command_name, command_config) in &self.commands {
            let file_path = dir_path.join(format!("{}.toml", command_name));
            let content = toml::to_string_pretty(command_config)?;
            fs::write(file_path, content)?;
        }

        Ok(())
    }

    /// Get subcommands for a specific command
    pub fn get_subcommands(&self, command: &str) -> Option<&Vec<String>> {
        self.commands.get(command).map(|c| &c.subcommands)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert!(config.commands.contains_key("opencode"));
        assert!(config.commands.contains_key("cargo"));
        assert!(config.commands.contains_key("rails"));
    }

    #[test]
    fn test_get_subcommands() {
        let config = Config::default();
        let subcommands = config.get_subcommands("opencode");
        assert!(subcommands.is_some());
        assert!(subcommands.unwrap().contains(&"acp".to_string()));
    }
}
