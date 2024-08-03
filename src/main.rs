use std::path::Path;

use clap::{value_parser, Arg, ArgAction, Command};
use commit::commit_notes;
use config::Config;
use nom::{
    bytes::complete::{tag, take_while_m_n},
    character::complete::{char, digit1},
    sequence::tuple,
    IResult,
};
use termfmt::{TermFmtExt, TermFmtsExt};

use self::{
    clean::clean_notes,
    generate::NameGenerator,
    note::NotesRepository,
    output::{DataBundle, OutputFmt},
};

mod clean;
mod commit;
mod config;
mod generate;
mod note;
mod output;

fn main() -> eyre::Result<()> {
    let cli = Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand_required(true)
        .subcommands([
            Command::new("generate")
                .args([Arg::new("count")
                    .value_parser(value_parser!(usize))
                    .help("the number of notes to generate (1 by default)")])
                .about("generate new note paths that do not yet exist"),
            Command::new("list")
                .args([Arg::new("show-tags")
                    .short('t')
                    .long("show-tags")
                    .action(ArgAction::SetTrue)
                    .help("display the tags of the notes")])
                .about("list all possible note files"),
            Command::new("search")
                .args([
                    Arg::new("tags")
                        .num_args(1..)
                        .help("the tags to search for. (the tags are additive)"),
                    Arg::new("show-tags")
                        .short('t')
                        .long("show-tags")
                        .action(ArgAction::SetTrue)
                        .help("display the tags of the notes, which matched the search parameters"),
                ])
                .about("search for notes using tags"),
            Command::new("todo")
                .subcommands([Command::new("list").args([
                    Arg::new("tags")
                        .num_args(1..)
                        .help("the tags to filter the todos for"),
                    Arg::new("show-tags")
                        .action(ArgAction::SetTrue)
                        .help("display the tags of the todos"),
                ])])
                .about("find you todos"),
            Command::new("commit")
                .about("commit the changes using git and push them to the remote"),
            Command::new("clean").about("clean the notes directory"),
        ])
        .termfmts()
        .get_matches();

    let config = Config::load();
    let mut term = cli.termfmt(DataBundle::new(config.clone()));

    let notes_repository = NotesRepository::new(config.clone());

    match cli.subcommand() {
        Some(("generate", command)) => {
            term.headline("GENERATE");

            let count: usize = command.get_one("count").copied().unwrap_or(1);
            let mut generator = NameGenerator::with_capacity(&config, count);

            for _ in 0..count {
                let note_path = generator.generate();
                term.file(note_path);
            }

            term.flush()?;
        }
        Some(("list", command)) => {
            let Ok(notes) = notes_repository.all_notes() else {
                term.file_error(config.path(), "could not read notes directory!");
                term.end();
                return Ok(());
            };
            let show_tags = command.get_flag("show-tags");

            for note in notes {
                term.list(note, show_tags);
            }
            term.flush()?;
        }
        Some(("search", command)) => {
            let Some(tags) = command.get_many::<String>("tags") else {
                term.error("please provide tags to search by!");
                term.end();
                return Ok(());
            };

            let queries: Vec<_> = tags.into_iter().collect();
            let Ok(notes) = notes_repository.search(queries.as_slice()) else {
                term.error("could not read notes directory!");
                term.end();
                return Ok(());
            };

            let show_tags = command.get_flag("show-tags");
            for note in notes {
                term.list(note, show_tags);
            }
            term.flush()?;
        }
        Some(("commit", _)) => {
            commit_notes(&config, &mut term);
            term.flush()?;
        }
        Some(("clean", _)) => {
            clean_notes(&config, &notes_repository, &mut term);
            term.flush()?;
        }
        Some((command, _)) => {
            term.error(format!("command not implemented: {}", command));
        }
        None => {}
    };
    term.end();
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
