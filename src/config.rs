use std::{
    env,
    fs::read_to_string,
    ops::Add,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
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
        let home_dir_notes = env::var("HOME").unwrap().add("/notes").into();
        Self {
            path: home_dir_notes,
        }
    }
}

impl Config {
    pub fn load() -> Config {
        read_to_string("~/.scatternotes.json")
            .map(|content| serde_json::from_str(&content).expect("Could not parse you config."))
            .unwrap_or_default()
    }
}
