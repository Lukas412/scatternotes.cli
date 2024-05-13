use std::collections::HashSet;
use std::path::PathBuf;

use chrono::{Local, NaiveDate};
use rand::{distributions::Alphanumeric, thread_rng, Rng};

use crate::config::Config;

pub struct NameGenerator<'a> {
    existing: HashSet<PathBuf>,
    config: &'a Config,
}

impl<'a> NameGenerator<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            existing: HashSet::new(),
            config,
        }
    }

    pub fn with_capacity(config: &'a Config, capacity: usize) -> Self {
        Self {
            existing: HashSet::with_capacity(capacity),
            config,
        }
    }

    pub fn generate(&mut self) -> PathBuf {
        self.generate_date(Local::now().date_naive())
    }

    pub fn generate_date(&mut self, date: NaiveDate) -> PathBuf {
        loop {
            let note_name = generate_note_name(date);

            let note_path = self.config.path().join(&note_name);
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
