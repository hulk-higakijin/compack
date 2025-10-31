pub mod cli;
pub mod error;
pub mod handlers;
pub mod shell;

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

    /// Get the bundled commands directory (from the project root at compile time)
    pub fn bundled_commands_dir() -> PathBuf {
        // This assumes the binary is being run from the project root during development
        // In production, this would need to be handled differently (e.g., embedded in binary)
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("commands")
    }

    /// Copy bundled command files to the target directory
    pub fn copy_bundled_commands(target_dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let bundled_dir = Self::bundled_commands_dir();
        
        // Create target directory if it doesn't exist
        fs::create_dir_all(target_dir)?;

        // Copy all .toml files from bundled directory
        for entry in fs::read_dir(&bundled_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() 
                && path.extension().and_then(|s| s.to_str()) == Some("toml")
                && let Some(file_name) = path.file_name()
            {
                let target_path = target_dir.join(file_name);
                fs::copy(&path, &target_path)?;
            }
        }

        Ok(())
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
    fn test_load_bundled_commands() {
        let bundled_dir = Config::bundled_commands_dir();
        let config = Config::load(&bundled_dir).expect("Failed to load bundled commands");
        
        assert!(config.commands.contains_key("opencode"));
        assert!(config.commands.contains_key("cargo"));
        assert!(config.commands.contains_key("rails"));
    }

    #[test]
    fn test_get_subcommands() {
        let bundled_dir = Config::bundled_commands_dir();
        let config = Config::load(&bundled_dir).expect("Failed to load bundled commands");
        
        let subcommands = config.get_subcommands("opencode");
        assert!(subcommands.is_some());
        assert!(subcommands.unwrap().contains(&"acp".to_string()));
    }

    #[test]
    fn test_copy_bundled_commands() {
        use std::env;
        
        // Create a temporary directory
        let temp_dir = env::temp_dir().join("compack_test");
        
        // Clean up if it exists
        if temp_dir.exists() {
            fs::remove_dir_all(&temp_dir).ok();
        }
        
        // Copy bundled commands
        Config::copy_bundled_commands(&temp_dir).expect("Failed to copy bundled commands");
        
        // Verify files were copied
        assert!(temp_dir.join("opencode.toml").exists());
        assert!(temp_dir.join("cargo.toml").exists());
        assert!(temp_dir.join("rails.toml").exists());
        
        // Clean up
        fs::remove_dir_all(&temp_dir).ok();
    }
}
