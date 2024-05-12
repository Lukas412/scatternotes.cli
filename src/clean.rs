use termfmt::TermFmt;

use crate::config::Config;
use crate::output::{DataBundle, OutputData, OutputFmt};
use crate::{note_files, read_note_tags};

pub fn clean_notes(config: &Config, term: &mut TermFmt<OutputData, DataBundle>) {
    term.headline("NOTES CLEANUP");

    let Some(notes) = note_files(config) else {
        term.error("could not read note files.");
        return;
    };

    for note in notes {
        let Ok(note_tags) = read_note_tags(&note).map_err(|error| {
            term.error(format_args!("cannot read note tags: {}", error));
        }) else {
            return;
        };

        if note_tags.iter().any(|tag| tag == "just-a-test") {
            term.list_with_tags(note, note_tags);
        }
    }
}
