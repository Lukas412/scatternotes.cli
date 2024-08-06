use clap::{Arg, ArgMatches, Command};

use crate::output::Term;

pub const NAME: &'static str = "carlender";

pub const CMD_GET: &'static str = "get";
pub const CMD_CLEAN: &'static str = "clean";

pub const ARG_DAY: &'static str = "day";

pub fn command() -> Command {
    Command::new(NAME)
        .subcommands([
            Command::new(CMD_CLEAN)
                .arg(
                    Arg::new(ARG_DAY)
                        .num_args(..)
                        .help("the days to clean (all if none given)"),
                )
                .about("clean up the carlenders"),
            Command::new(CMD_GET)
                .arg(
                    Arg::new(ARG_DAY)
                        .num_args(1..)
                        .help("the day of the carlender"),
                )
                .about("get the filepath of the carlender for a given day"),
        ])
        .about("operate on the carlenders")
}

pub fn run(command: &ArgMatches, term: &Term) {}
