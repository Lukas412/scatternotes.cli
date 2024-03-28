use std::{env, fs::read_to_string, ops::Add, path::{Path, PathBuf}};

use serde::Deserialize;

use crate::tag::TagConfig;

#[derive(Debug, Deserialize)]
pub struct Config {
    path: PathBuf,
    tags: Option<TagConfig>,
    tag_config_path: Option<PathBuf>
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
            tags: None,
            tag_config_path: None
        }
    }
}

impl Config {
    pub fn load() -> Config {
        read_to_string( "~/.scatternotes.json")
            .map(|content| serde_json::from_str(&content).expect("Could not parse you config."))
            .unwrap_or_default()
    }

    pub fn tag_config_loading(&self) -> TagConfigLoading {
        if let Some(config) = &self.tags {
            return TagConfigLoading::Loaded(config.clone())
        }
        let path = self.tag_config_path.clone().unwrap_or_else(|| {
            self.path.join(".tags.json")
        });
        TagConfigLoading::Path(path)
    }
}

pub enum TagConfigLoading {
    Loaded(TagConfig),
    Path(PathBuf)
}

