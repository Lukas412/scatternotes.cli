use clap::{Arg, ArgAction, ArgMatches, Command};

use crate::config::Config;
use crate::output::Term;

pub const NAME: &'static str = "todo";

pub fn command() -> Command {
    Command::new("todo")
        .subcommands([Command::new("list").args([
            Arg::new("tags")
                .num_args(1..)
                .help("the tags to filter the todos for"),
            Arg::new("show-tags")
                .action(ArgAction::SetTrue)
                .help("display the tags of the todos"),
        ])])
        .about("find you todos")
}

pub fn run(command: &ArgMatches, term: &Term, config: &Config) {}
