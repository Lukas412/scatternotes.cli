use chrono::NaiveDate;
use clap::{Arg, ArgMatches, Command};

use crate::carlender::Carlender;
use crate::config::Config;
use crate::output::{OutputFmt, Term};

pub const NAME: &str = "carlender";

pub const CMD_GET: &str = "get";
pub const CMD_CLEAN: &str = "clean";
pub const CMD_SHOW: &str = "show";
pub const CMD_EDIT: &str = "edit";

pub const ARG_DAY: &str = "day";

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
            Command::new(CMD_SHOW)
                .arg(Arg::new(ARG_DAY).help("the day of the carlender you want to view"))
                .about("show the carlender for the given day"),
            Command::new(CMD_EDIT).about("edit the carlender file"),
        ])
        .subcommand_required(true)
        .about("operate on the carlenders")
}

pub fn run(command: &ArgMatches, term: &mut Term, config: &Config) {
    match command.subcommand().unwrap() {
        ("get", command) => {
            let dates = command.get_many::<String>(ARG_DAY).unwrap();
            // run_get(term, config, dates.into_iter());
        }
        (name, _) => {
            term.error(format_args!(
                "subcommand of carlender is not implemented: {}",
                name
            ));
        }
    }
}

fn run_get(term: &mut Term, config: &Config, dates: impl Iterator<Item = NaiveDate>) {
    for date in dates {
        let path = config.carlender(date);
        term.headline("CARLENDER PATH");
        term.file(&path);
    }
}

fn run_clean() {}

fn run_show(term: &mut Term, config: &Config, date: NaiveDate) {}

fn run_edit() {}
