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
    /// Load configuration from a file
    pub fn load(path: &PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    /// Get default config file path (~/.config/compack/commands.toml)
    pub fn default_config_path() -> Result<PathBuf, io::Error> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Config directory not found"))?;
        Ok(config_dir.join("compack").join("commands.toml"))
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

    /// Save configuration to a file
    pub fn save(&self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        // Create parent directories if they don't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = toml::to_string_pretty(self)?;
        fs::write(path, content)?;
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
