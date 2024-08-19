use clap::{ArgMatches, Command};
use termfmt::TermFmtExt;

use crate::config::Config;
use crate::output::OutputFmt;

mod carlender;
mod clean;
mod code;
mod commit;
mod generate;
mod list;
mod persons;
mod search;
mod todo;

pub fn list() -> impl IntoIterator<Item = Command> {
    [
        carlender::command(),
        clean::command(),
        code::command(),
        commit::command(),
        generate::command(),
        list::command(),
        persons::command(),
        search::command(),
        todo::command(),
    ]
}

pub fn run(command: ArgMatches) {
    let config = Config::load();
    let mut term = command.termfmt(&config);

    let Some((name, command)) = command.subcommand() else {
        term.error("please provide a command!");
        term.info("run 'scatternotes --help' for more info");
        return;
    };

    match name {
        carlender::NAME => carlender::run(command, &mut term, &config),
        clean::NAME => clean::run(&mut term, &config),
        code::NAME => code::run(command, &mut term, &config),
        commit::NAME => commit::run(&mut term, &config),
        generate::NAME => generate::run(command, &mut term, &config),
        list::NAME => list::run(command, &mut term, &config),
        persons::NAME => persons::run(command, &mut term, &config),
        search::NAME => search::run(command, &mut term, &config),
        todo::NAME => todo::run(command, &mut term, &config),
        _ => term.error(format_args!("command not implemented: {}", name)),
    }

    term.flush().unwrap();
    term.end();
}
