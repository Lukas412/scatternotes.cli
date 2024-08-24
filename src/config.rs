use std::{
    env,
    fs::{self, read_to_string},
    ops::Add,
    path::{Path, PathBuf},
};

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    path: PathBuf,
    code_path: PathBuf,
    carlender_path: PathBuf,
    meta_path: PathBuf,
}

impl Config {
    pub fn load() -> Config {
        read_to_string("~/.scatternotes.json")
            .map(|content| serde_json::from_str(&content).expect("Could not parse you config."))
            .unwrap_or_default()
    }

    pub fn new(
        path: PathBuf,
        code_path: PathBuf,
        carlender_path: PathBuf,
        meta_path: PathBuf,
    ) -> eyre::Result<Self> {
        Ok(Self {
            path,
            code_path,
            carlender_path,
            meta_path,
        })
    }

    pub fn path(&self) -> &Path {
        ensure_directory_exists(&self.path).unwrap();
        &self.path
    }

    pub fn note(&self, name: &str) -> PathBuf {
        ensure_directory_exists(&self.path).unwrap();
        self.path.join(name)
    }

    pub fn code(&self, name: &str) -> PathBuf {
        ensure_directory_exists(&self.code_path).unwrap();
        self.code_path.join(name)
    }

    pub fn meta(&self, name: &str) -> PathBuf {
        ensure_directory_exists(&self.meta_path).unwrap();
        self.meta_path.join(name)
    }

    pub fn carlender(&self, date: NaiveDate) -> PathBuf {
        ensure_directory_exists(&self.carlender_path).unwrap();
        self.carlender_path
            .join(format!("{}", date.format("%Y-%m-%d_carlender.md")))
    }
}

impl Default for Config {
    fn default() -> Self {
        let path: PathBuf = env::var("HOME").unwrap().add("/notes").into();
        let code_path = path.join("code");
        let carlender_path = path.join("carlender");
        let meta_path = path.join("meta");
        Self::new(path, code_path, carlender_path, meta_path).unwrap()
    }
}

fn ensure_directory_exists(path: impl AsRef<Path>) -> eyre::Result<()> {
    let path = path.as_ref();
    if !path.exists() || !path.is_dir() {
        fs::create_dir_all(&path)?;
    }
    Ok(())
}
