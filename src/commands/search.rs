use clap::{Arg, ArgAction, ArgMatches, Command};

use crate::note::NotesRepository;
use crate::output::{OutputFmt, Term};

pub const NAME: &'static str = "search";

pub const ARG_QUERIES: &'static str = "queries";
pub const ARG_TAGS: &'static str = "tags";

pub fn command() -> Command {
    Command::new(NAME)
        .args([
            Arg::new(ARG_QUERIES)
                .num_args(1..)
                .help("the tags to search for. (the tags are additive)"),
            Arg::new(ARG_TAGS)
                .long(ARG_TAGS)
                .action(ArgAction::SetTrue)
                .help("display the tags of the notes, which matched the search parameters"),
        ])
        .about("search for notes using tags")
}

pub fn run(command: &ArgMatches, term: &mut Term, notes_repository: &NotesRepository) {
    let Some(tags) = command.get_many::<String>(ARG_QUERIES) else {
        term.error("please provide tags to search by!");
        term.end();
        return;
    };

    let queries: Vec<_> = tags.into_iter().collect();
    let Ok(notes) = notes_repository.search(queries.as_slice()) else {
        term.error("could not read notes directory!");
        term.end();
        return;
    };

    let show_tags = command.get_flag(ARG_TAGS);
    for note in notes {
        term.list(note, show_tags);
    }
}
