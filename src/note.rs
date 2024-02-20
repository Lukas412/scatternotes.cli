use std::{
    fs::{self, read_to_string},
    io,
    path::{Path, PathBuf},
};

use rand::{distributions::Alphanumeric, thread_rng, Rng};

use crate::config::Config;

use self::tag::{Tag, TagIter};

mod tag;

pub struct Note {
    key: String,
    content: String,
}

impl Note {
    pub fn load_from(path: &Path) -> io::Result<Self> {
        let key = Self::get_key_from_path_or_new_key(path);
        let content = read_to_string(path)?;
        Ok(Self { key, content })
    }

    pub fn save(&self, config: &Config) -> io::Result<()> {
        let mut path: PathBuf = [config.path(), self.key.as_ref()].iter().collect();
        path.set_extension("md");
        fs::write(path, &self.content)
    }

    fn get_key_from_path_or_new_key(path: &Path) -> String {
        path.to_str()
            .filter(|key| key.len() == 23)
            .filter(|key| key.ends_with(".md"))
            .filter(|key| {
                let valid_key_chars = key.chars()
                    .take_while(|char| matches!(char, 'A' ..= 'Z' | '0' ..= '9'))
                    .count();
                valid_key_chars == 20
            })
            .map(|key| key[..20].to_owned())
            .unwrap_or_else(|| Self::generate_new_key())
    }

    fn generate_new_key() -> String {
        thread_rng()
            .sample_iter(&Alphanumeric)
            .map(|char| char.to_ascii_uppercase() as char)
            .take(20)
            .collect()
    }

    fn iter(&self) -> impl Iterator<Item = Tag> {
        TagIter::new(&self.content)
    }
}

