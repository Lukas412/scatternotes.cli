use std::{
    io::Write,
    fs::{read_to_string, OpenOptions},
    path::{Path, PathBuf},
};

use rand::{distributions::Alphanumeric, thread_rng, Rng};

use crate::config::Config;

pub use self::{error::{NoteError, NoteLoadingError, NoteSavingError}, tag::{Tag, TagIter}};

mod tag;

mod error {
    use std::{error::Error, fmt::{Debug, Display}, io, path::{Path, PathBuf}};

    #[derive(Debug)]
    pub struct NoteLoadingError<'a> {
        pub error: io::Error,
        pub path: &'a Path,
    }

    impl<'a> Display for NoteLoadingError<'a> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            writeln!(f, "The file {} could not be opened.\nSource: {}", self.path.display(), self.error)
        }
    }

    impl<'a> Error for NoteLoadingError<'a> {}

    #[derive(Debug)]
    pub struct NoteSavingError {
        pub error: io::Error,
        pub path: PathBuf,
    }

    impl Display for NoteSavingError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            writeln!(f, "The file {} could not be saved.\nSource: {}", self.path.display(), self.error)
        }
    }

    impl Error for NoteSavingError {}

    #[derive(Debug)]
    pub enum NoteError<'a> {
        Loading(NoteLoadingError<'a>),
        Saving(NoteSavingError),
    }

    impl<'a> Display for NoteError<'a> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Loading(error) => Display::fmt(&error, f),
                Self::Saving(error) => Display::fmt(&error, f),
            }
        }
    }

    impl<'a> Error for NoteError<'a> {}

    impl<'a> From<NoteLoadingError<'a>> for NoteError<'a> {
        fn from(value: NoteLoadingError<'a>) -> Self {
            NoteError::Loading(value)
        }
    }

    impl<'a> From<NoteSavingError> for NoteError<'a> {
        fn from(value: NoteSavingError) -> Self {
            NoteError::Saving(value)
        }
    }
}

pub struct Note {
    key: String,
    content: String,
}

impl Note {
    pub fn load_from(path: &Path) -> Result<Self, NoteLoadingError> {
        let key = Self::get_key_from_path_or_new_key(path);
        let content = read_to_string(path)
            .map_err(|error| NoteLoadingError { error, path })?;
        Ok(Self { key, content })
    }

    pub fn save(&self, config: &Config) -> Result<(), NoteSavingError> {
        let mut path: PathBuf = [config.path(), self.key.as_ref()].iter().collect();
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
        write!(file, "{}", &self.content)
            .map_err(|error| NoteSavingError { error, path })
    }

    pub fn key(&self) -> &str {
        self.key.as_str()
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

    pub fn tags(&self) -> impl Iterator<Item = Tag> {
        TagIter::new(&self.content)
    }
}

