use clap::{Arg, ArgAction, ArgMatches, Command};

use crate::note::NotesRepository;
use crate::output::{OutputFmt, Term};

pub const NAME: &'static str = "search";

pub fn command() -> Command {
    Command::new("search")
        .args([
            Arg::new("tags")
                .num_args(1..)
                .help("the tags to search for. (the tags are additive)"),
            Arg::new("show-tags")
                .short('t')
                .long("show-tags")
                .action(ArgAction::SetTrue)
                .help("display the tags of the notes, which matched the search parameters"),
        ])
        .about("search for notes using tags")
}

pub fn run(command: &ArgMatches, term: &mut Term, notes_repository: &NotesRepository) {
    let Some(tags) = command.get_many::<String>("tags") else {
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

    let show_tags = command.get_flag("show-tags");
    for note in notes {
        term.list(note, show_tags);
    }
}
