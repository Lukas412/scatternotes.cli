use std::{
    fs::{read_to_string, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

use rand::{distributions::Alphanumeric, thread_rng, Rng};

use crate::{config::Config, tag::Tag};

pub use self::error::{NoteError, NoteLoadingError, NoteSavingError};

mod error {
    use std::{
        error::Error,
        fmt::{Debug, Display},
        io,
        path::PathBuf,
    };

    #[derive(Debug)]
    pub struct NoteLoadingError {
        pub error: io::Error,
        pub path: PathBuf,
    }

    impl Display for NoteLoadingError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            writeln!(
                f,
                "The file {} could not be opened.\nSource: {}",
                self.path.display(),
                self.error
            )
        }
    }

    impl Error for NoteLoadingError {}

    #[derive(Debug)]
    pub struct NoteSavingError {
        pub error: io::Error,
        pub path: PathBuf,
    }

    impl Display for NoteSavingError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            writeln!(
                f,
                "The file {} could not be saved.\nSource: {}",
                self.path.display(),
                self.error
            )
        }
    }

    impl Error for NoteSavingError {}

    #[derive(Debug)]
    pub enum NoteError {
        Loading(NoteLoadingError),
        Saving(NoteSavingError),
    }

    impl Display for NoteError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Loading(error) => Display::fmt(&error, f),
                Self::Saving(error) => Display::fmt(&error, f),
            }
        }
    }

    impl Error for NoteError {}

    impl From<NoteLoadingError> for NoteError {
        fn from(value: NoteLoadingError) -> Self {
            NoteError::Loading(value)
        }
    }

    impl From<NoteSavingError> for NoteError {
        fn from(value: NoteSavingError) -> Self {
            NoteError::Saving(value)
        }
    }
}

pub struct Note {
    key: String,
    tags: Vec<Tag>,
    content: String,
}

impl Note {
    pub fn new() -> Self {
        Self {
            key: Self::generate_new_key(),
            tags: Vec::new(),
            content: String::new(),
        }
    }

    pub fn with_tags(mut self, tags: impl IntoIterator<Item = Tag>) -> Self {
        self.add_tags(tags);
        self
    }

    pub fn add_tags(&mut self, tags: impl IntoIterator<Item = Tag>) {
        for tag in tags {
            if !self.has_tag(&tag) {
                self.tags.push(tag);
            }
        }
    }

    pub fn has_tag(&self, tag: &Tag) -> bool {
        self.tags.iter().any(|note_tag| note_tag == tag)
    }

    pub fn load_from(path: PathBuf) -> Result<Self, NoteLoadingError> {
        let key = Self::get_key_from_path_or_new_key(path.as_ref());
        let tags = Vec::new();
        match read_to_string(&path) {
            Ok(content) => return Ok(Self { key, tags, content }),
            Err(error) => return Err(NoteLoadingError { error, path }),
        };
    }

    pub fn save(&self, config: &Config) -> Result<PathBuf, NoteSavingError> {
        let mut path: PathBuf = config.path().join(&self.key);
        path.set_extension("md");
        let file = OpenOptions::new()
            .create_new(true)
            .create(true)
            .truncate(true)
            .write(true)
            .open(&path);
        let mut file = match file {
            Ok(file) => file,
            Err(error) => return Err(NoteSavingError { error, path }),
        };
        let result = 
        match result {
            Ok(_) => return Ok(path),
            Err(error) => return Err(NoteSavingError { error, path }),
        }
    }

    pub fn key(&self) -> &str {
        self.key.as_str()
    }

    fn get_key_from_path_or_new_key(path: &Path) -> String {
        path.to_str()
            .filter(|key| key.len() == 23)
            .filter(|key| key.ends_with(".md"))
            .filter(|key| {
                let valid_key_chars = key
                    .chars()
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

    pub fn tags(&self) -> impl Iterator<Item = &Tag> {
        self.tags.iter()
    }
}
