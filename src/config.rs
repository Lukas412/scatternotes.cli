use std::{fs::read_to_string, path::{Path, PathBuf}, str::FromStr};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    path: PathBuf,
}

impl Config {
    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            path: PathBuf::from_str("~/notes").unwrap()
        }
    }
}

impl Config {
    pub fn load() -> Config {
        read_to_string( "~/.scatternotes.json")
            .map(|content| serde_json::from_str(&content).expect("Could not parse you config."))
            .unwrap_or_default()
    }
}

