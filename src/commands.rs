use clap::{ArgMatches, Command};
use termfmt::TermFmtExt;

use crate::config::Config;
use crate::note::NotesRepository;
use crate::output::OutputFmt;

mod carlender;
mod clean;
mod commit;
mod generate;
mod list;
mod search;
mod todo;

pub fn list() -> impl IntoIterator<Item = Command> {
    [
        carlender::command(),
        clean::command(),
        commit::command(),
        generate::command(),
        list::command(),
        search::command(),
        todo::command(),
    ]
}

pub fn run(command: ArgMatches) {
    let config = Config::load();
    let notes_repository = NotesRepository::new(config.clone());

    let mut term = command.termfmt(&config);

    let Some((name, command)) = command.subcommand() else {
        term.error("please provide a command!");
        term.info("run 'scatternotes --help' for more info");
        return;
    };

    match name {
        carlender::NAME => carlender::run(command, &mut term),
        clean::NAME => clean::run(&mut term, &config, &notes_repository),
        commit::NAME => commit::run(&mut term, &config),
        generate::NAME => generate::run(command, &mut term, &config),
        list::NAME => list::run(command, &mut term, &config, &notes_repository),
        search::NAME => search::run(command, &mut term, &notes_repository),
        todo::NAME => todo::run(command, &mut term, &notes_repository),
        _ => term.error(format_args!("command not implemented: {}", name)),
    }

    term.flush().unwrap();
    term.end();
}
