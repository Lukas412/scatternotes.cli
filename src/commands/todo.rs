use clap::{Arg, ArgAction, ArgMatches, Command};

use crate::note::NotesRepository;
use crate::output::{OutputFmt, Term};

pub const NAME: &'static str = "todo";

pub const CMD_LIST: &'static str = "list";
pub const CMD_SEARCH: &'static str = "search";

pub const ARG_QUERIES: &'static str = "queries";
pub const ARG_DONE: &'static str = "done";

pub fn command() -> Command {
    let arg_done = Arg::new("done")
        .long("done")
        .action(ArgAction::SetTrue)
        .help("also list done tags");
    Command::new(NAME)
        .subcommand_required(true)
        .subcommands([
            Command::new(CMD_LIST)
                .arg(arg_done.clone())
                .about("list all todos"),
            Command::new(CMD_SEARCH)
                .args([
                    Arg::new(ARG_QUERIES)
                        .num_args(1..)
                        .help("the tags to search for. (the tags are additive)"),
                    arg_done,
                ])
                .about("search through your todos"),
        ])
        .about("find you todos")
}

pub fn run(command: &ArgMatches, term: &mut Term, notes_repository: &NotesRepository) {
    match command.subcommand().unwrap() {
        (CMD_LIST, command) => {
            let view_done = command.get_flag(ARG_DONE);
            run_list(term, notes_repository, view_done);
        }
        (CMD_SEARCH, command) => {
            let queries: Vec<_> = command
                .get_many::<String>(ARG_QUERIES)
                .unwrap_or_default()
                .into_iter()
                .collect();
            let view_done = command.get_flag(ARG_DONE);
            run_search(term, notes_repository, queries.as_slice(), view_done)
        }
        (_, _) => term.error("command not implemented yet"),
    }
}

fn run_list(term: &mut Term, notes_repository: &NotesRepository, view_done: bool) {
    for note in notes_repository.all_notes().unwrap() {
        for todo in note.todos() {
            if !view_done && todo.is_done() {
                continue;
            }
            term.todo(note.path(), todo.content());
        }
    }
}

fn run_search(
    term: &mut Term,
    notes_repository: &NotesRepository,
    queries: &[&String],
    view_done: bool,
) {
    for note in notes_repository.search(queries).unwrap() {
        for todo in note.todos() {
            if !view_done && todo.is_done() {
                continue;
            }
            term.todo(note.path(), todo.content());
        }
    }
}
