use crate::config::Config;
use crate::generate::NameGenerator;

pub use dto::{Note, Tag};
pub use repository::NotesRepository;

mod dto;
mod repository;

pub struct NoteService {
    config: Config,
    generator: Option<NameGenerator>,
}

impl NoteService {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            generator: None,
        }
    }

    pub fn generate_new(&mut self) -> Note {
        if self.generator.is_none() {
            self.generator = Some(NameGenerator::new());
        }
        let generator = self.generator.as_mut().unwrap();
        let path = generator.generate(&self.config);
        Note::new(path)
    }
}
