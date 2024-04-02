use std::path::PathBuf;

use chrono::Local;
use rand::{distributions::Alphanumeric, thread_rng, Rng};

use crate::config::Config;

pub fn generate_new_note_path(config: &Config) -> PathBuf {
    loop {
        let note_name = generate_note_name();

        let note_path = config.path().join(&note_name);
        if note_path.exists() {
            continue;
        }

        return note_path;
    }
}

fn generate_note_name() -> String {
    let date = Local::now().format("%Y-%m-%d");
    let random_key: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(20)
        .map(char::from)
        .collect();

    format!("{}_{}.md", date, random_key)
}
