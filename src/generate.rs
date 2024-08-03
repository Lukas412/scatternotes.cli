use std::collections::HashSet;
use std::path::PathBuf;

use chrono::{Local, NaiveDate};
use rand::{distributions::Alphanumeric, thread_rng, Rng};

use crate::config::Config;

pub struct NameGenerator {
    existing: HashSet<PathBuf>,
}

impl NameGenerator {
    pub fn new() -> Self {
        Self {
            existing: HashSet::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            existing: HashSet::with_capacity(capacity),
        }
    }

    pub fn generate(&mut self, config: &Config) -> PathBuf {
        self.generate_with_date(Local::now().date_naive(), config)
    }

    pub fn generate_with_date(&mut self, date: NaiveDate, config: &Config) -> PathBuf {
        loop {
            let note_name = generate_note_name(date);

            let note_path = config.path().join(&note_name);
            if note_path.exists() {
                continue;
            }

            if self.existing.contains(&note_path) {
                continue;
            }

            self.existing.insert(note_path.clone());
            return note_path;
        }
    }
}

fn generate_note_name(date: NaiveDate) -> String {
    let date = date.format("%Y-%m-%d");
    let random_key: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(20)
        .map(char::from)
        .collect();

    format!("{}_{}.md", date, random_key)
}
