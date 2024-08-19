use std::collections::HashSet;

use clap::{ArgMatches, Command};
use itertools::Itertools;

use crate::config::Config;
use crate::note::Note;
use crate::output::{OutputFmt, Term};

pub const NAME: &str = "persons";

pub const CMD_INDEX: &str = "index";
pub const CMD_CLEAN: &str = "clean";

pub fn command() -> Command {
    Command::new(NAME)
        .subcommands([
            Command::new(CMD_INDEX).about("create index for persons in notes"),
            Command::new(CMD_CLEAN).about("clean the persons in notes"),
        ])
        .subcommand_required(true)
        .about("manage the persons in your notes")
}

pub fn run(command: &ArgMatches, term: &mut Term, config: &Config) {
    match command.subcommand().unwrap() {
        (CMD_INDEX, _) => {
            let mut persons = HashSet::new();
            for note in Note::all_notes(config).unwrap() {
                for person in note.persons() {
                    if persons.get(person).is_some() {
                        continue;
                    }
                    persons.insert(person.to_owned());
                }
            }
            println!("{}", persons.iter().join(", "));
        }
        (name, _) => term.error(format_args!(
            "command '{} {}' is not implemented",
            NAME, name
        )),
    }
}
