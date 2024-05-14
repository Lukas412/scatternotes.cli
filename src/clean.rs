use std::fs;

use termfmt::TermFmt;

use crate::config::Config;
use crate::generate::NameGenerator;
use crate::note::note_date;
use crate::output::{DataBundle, OutputData, OutputFmt};
use crate::{is_valid_note_name, note_files, read_note_tags};

pub fn clean_notes(config: &Config, term: &mut TermFmt<OutputData, DataBundle>) {
    term.headline("CLEANUP NOTES");

    let Some(notes) = note_files(config) else {
        term.error("could not read note files.");
        return;
    };

    let mut generator = NameGenerator::new(&config);

    for note in notes {
        let Ok(note_tags) = read_note_tags(&note).map_err(|error| {
            term.error(format_args!("cannot read note tags: {}", error));
        }) else {
            return;
        };

        if note_tags.iter().any(|tag| tag == "just-a-test") {
            term.cleanup_remove(&note, note_tags);
            if let Err(error) = fs::remove_file(note) {
                term.error(error);
            };
            continue;
        }

        let note = if !is_valid_note_name(&note) {
            term.cleanup_rename(&note);
            let Some(date) = note_date(config, &note) else {
                term.error("could not get date of note!");
                continue;
            };
            let new_note = generator.generate_with_date(date);
            if let Err(error) = fs::rename(&note, &new_note) {
                term.error(error);
                continue;
            };
            new_note
        } else {
            note
        };

        let Ok(content) = fs::read_to_string(&note) else {
            continue;
        };
    }
}
