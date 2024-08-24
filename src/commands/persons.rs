use std::collections::HashSet;
use std::fmt::Write;

use clap::{ArgMatches, Command};
use itertools::Itertools;

use crate::config::Config;
use crate::note::Note;
use crate::output::{OutputFmt, Term};
use crate::person::Person;

pub const NAME: &str = "persons";

pub const CMD_INDEX: &str = "index";
pub const CMD_LIST: &str = "list";
pub const CMD_CLEAN: &str = "clean";

pub fn command() -> Command {
    Command::new(NAME)
        .subcommands([
            Command::new(CMD_INDEX).about("create index for persons in notes"),
            Command::new(CMD_LIST).about("list persons from the index"),
            Command::new(CMD_CLEAN).about("clean the persons in notes"),
        ])
        .subcommand_required(true)
        .about("manage the persons in your notes")
}

pub fn run(command: &ArgMatches, term: &mut Term, config: &Config) {
    match command.subcommand().unwrap() {
        (CMD_INDEX, _) => {
            let persons = Person::search_all_persons(config).unwrap();
            Person::save_persons(config, &persons);
            term.headline("FOUND PERSONS");
            if persons.is_empty() {
                term.info("no persons found");
            } else {
                term.persons(&persons);
            }
        }
        (CMD_LIST, _) => {
            let Ok(persons) = Person::load_all_persons(config) else {
                term.error("no 'persons.txt' found");
                term.info("run 'scatternotes persons index' to generate the file");
                return;
            };
            term.headline("PERSONS");
            if persons.is_empty() {
                term.info("no persons");
            } else {
                term.persons(&persons);
            }
        }
        (name, _) => term.error(format_args!(
            "command '{} {}' is not implemented",
            NAME, name
        )),
    }
}
