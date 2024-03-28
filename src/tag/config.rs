use std::fs::read_to_string;

use serde::Deserialize;

use crate::config::{Config, TagConfigLoading};

#[derive(Debug, Clone, Deserialize)]
pub struct TagConfig {
    context: Vec<String>,
    persons: Vec<String>,
}

impl TagConfig {
    pub fn load(config: &Config) -> Self {
        match config.tag_config_loading() {
            TagConfigLoading::Loaded(config) => config,
            TagConfigLoading::Path(path) => read_to_string(path)
                .ok()
                .and_then(|content| serde_json::from_str(content.as_str()).ok())
                .unwrap_or_default(),
        }
    }
}

impl Default for TagConfig {
    fn default() -> Self {
        Self {
            context: Vec::new(),
            persons: Vec::new(),
        }
    }
}
