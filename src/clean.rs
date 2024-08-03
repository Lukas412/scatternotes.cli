use std::fs;

use termfmt::TermFmt;

use crate::config::Config;
use crate::is_valid_note_name;
use crate::note::{note_date, note_header, NotesRepository};
use crate::output::{DataBundle, OutputData, OutputFmt};

pub fn clean_notes(
    config: &Config,
    notes_repository: &NotesRepository,
    term: &mut TermFmt<OutputData, DataBundle>,
) {
    term.headline("CLEANUP NOTES");

    let Ok(notes) = notes_repository.all_notes() else {
        term.error("could not read note files.");
        return;
    };

    for note in notes {
        if note.tags().iter().any(|tag| tag.text() == "just-a-test") {
            term.cleanup_remove(note.clone(), true);
            if let Err(error) = fs::remove_file(note.path()) {
                term.error(error);
            };
            continue;
        }

        let note = if !is_valid_note_name(note.path()) {
            term.cleanup_rename(&note);
            let Some(date) = note_date(config, note.path()) else {
                term.error("could not get date of note!");
                continue;
            };
            let new_note = generator.generate_with_date(date);
            if let Err(error) = fs::rename(note.path(), &new_note) {
                term.error(error);
                continue;
            };
            new_note
        } else {
            note.path().to_owned()
        };

        println!("{}", note.display());
        note_header(&note);
    }
}
