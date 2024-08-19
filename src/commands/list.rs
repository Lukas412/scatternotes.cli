use clap::{Arg, ArgAction, ArgMatches, Command};

use crate::config::Config;
use crate::note::Note;
use crate::output::{OutputFmt, Term};

pub const NAME: &'static str = "list";
pub const ARG_TAGS: &'static str = "tags";

pub fn command() -> Command {
    Command::new(NAME)
        .args([Arg::new(ARG_TAGS)
            .short('t')
            .long(ARG_TAGS)
            .action(ArgAction::SetTrue)
            .help("display the tags of the notes")])
        .about("list all possible note files")
}

pub fn run(command: &ArgMatches, term: &mut Term, config: &Config) {
    let Ok(notes) = Note::all_notes(config) else {
        term.file_error(config.path(), "could not read notes directory!");
        term.end();
        return;
    };
    let show_tags = command.get_flag(ARG_TAGS);

    for note in notes {
        term.list(&note, show_tags);
    }
}
