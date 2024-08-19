#![allow(unused)]

use std::path::Path;

use clap::Command;
use nom::{
    bytes::complete::{tag, take_while_m_n},
    character::complete::{char, digit1},
    sequence::tuple,
    IResult,
};
use termfmt::TermFmtsExt;

use self::name::NameGenerator;

mod carlender;
mod code;
mod commands;
mod config;
mod name;
mod note;
mod output;
mod tag;
mod todo;

fn main() -> eyre::Result<()> {
    let cli = Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand_required(true)
        .subcommands(commands::list())
        .termfmts()
        .get_matches();

    commands::run(cli);

    Ok(())
}

fn is_valid_note_name(note_path: &Path) -> bool {
    let Some(note_name) = note_path.file_name().and_then(|name| name.to_str()) else {
        return false;
    };

    fn date(input: &str) -> IResult<&str, (&str, char, &str, char, &str)> {
        tuple((digit1, char('-'), digit1, char('-'), digit1))(input)
    }
    fn key(input: &str) -> IResult<&str, &str> {
        take_while_m_n(
            20,
            20,
            |char| matches!(char, 'a'..='z' | 'A'..='Z' | '0'..='9'),
        )(input)
    }
    let mut note_name_parser = tuple((date, char('_'), key, tag(".md")));
    note_name_parser(note_name).is_ok()
}
