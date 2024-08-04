use clap::{ArgMatches, Command};

use crate::output::Term;

pub const NAME: &'static str = "carlender";

pub fn command() -> Command {
    Command::new(NAME)
}

pub fn run(command: &ArgMatches, term: &Term) {}
