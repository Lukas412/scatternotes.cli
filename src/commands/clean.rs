use std::fs;

use crate::config::Config;
use crate::is_valid_note_name;
use crate::note::NotesRepository;
use crate::output::{OutputFmt, Term};
use crate::NameGenerator;

pub const NAME: &'static str = "clean";

pub fn command() -> clap::Command {
    clap::Command::new(NAME).about("clean the notes directory")
}

pub fn run(term: &mut Term, config: &Config, notes_repository: &NotesRepository) {
    term.headline("CLEANUP NOTES");

    let Ok(notes) = notes_repository.all_notes() else {
        term.error("could not read note files.");
        return;
    };

    let mut changes_done = false;
    for note in notes {
        if note.tags().iter().any(|tag| tag.text() == "just-a-test") {
            changes_done = true;
            term.cleanup_remove(&note, true);
            if let Err(error) = fs::remove_file(note.path()) {
                term.error(error);
            };
            continue;
        }

        let mut generator = NameGenerator::new();
        let note = if !is_valid_note_name(note.path()) {
            changes_done = true;
            term.cleanup_rename(&note);
            let Some(date) = notes_repository.search_date(config, note.path()) else {
                term.error("could not get date of note!");
                continue;
            };
            let new_note = generator.generate_with_date(date, config);
            if let Err(error) = fs::rename(note.path(), &new_note) {
                term.error(error);
                continue;
            };
            new_note
        } else {
            note.path().to_owned()
        };

        //note_header(&note);
    }

    if !changes_done {
        term.hint("no changes done");
    }
}
