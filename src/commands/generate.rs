use clap::{value_parser, Arg, ArgMatches, Command};

use crate::config::Config;
use crate::name::NameGenerator;
use crate::output::{OutputFmt, Term};

pub const NAME: &'static str = "generate";

pub fn command() -> Command {
    Command::new(NAME)
        .args([Arg::new("count")
            .value_parser(value_parser!(usize))
            .help("the number of notes to generate (1 by default)")])
        .about("generate new note paths that do not yet exist")
}

pub fn run(command: &ArgMatches, term: &mut Term, config: &Config) {
    term.headline("GENERATE");

    let count: usize = command.get_one("count").copied().unwrap_or(1);
    let mut generator = NameGenerator::with_capacity(count);

    for _ in 0..count {
        let note_path = generator.generate(&config);
        term.file(note_path);
    }
}
