use std::{
    collections::{HashSet, VecDeque},
    fs::{self, File},
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

use clap::{value_parser, Arg, ArgAction, Command};
use commit::commit_notes;
use config::Config;
use generate::generate_new_note_path;
use termfmt::{TermFmtExt, TermFmtsExt};

use self::{
    clean::clean_notes,
    output::{DataBundle, OutputFmt},
};

mod clean;
mod commit;
mod config;
mod generate;
mod output;

fn main() -> eyre::Result<()> {
    let cli = Command::new("scatternotes")
        .version("0.0.1")
        .about("a cli to create and manage notes")
        .subcommand_required(true)
        .subcommands([
            Command::new("generate")
                .alias("g")
                .args([Arg::new("count")
                    .value_parser(value_parser!(usize))
                    .help("the number of notes to generate (1 by default)")])
                .about("generate new note paths that do not yet exist"),
            Command::new("list")
                .alias("l")
                .args([Arg::new("show-tags")
                    .short('t')
                    .long("show-tags")
                    .action(ArgAction::SetTrue)
                    .help("display the tags of the notes")])
                .about("list all possible note files"),
            Command::new("search")
                .alias("s")
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
            Command::new("commit")
                .alias("c")
                .about("commit the changes using git and push them to the remote"),
            Command::new("clean").about("clean the notes directory"),
        ])
        .termfmts()
        .get_matches();

    let config = Config::load();
    let mut term = cli.termfmt(DataBundle::new(config.clone()));

    match cli.subcommand() {
        Some(("generate", command)) => {
            term.headline("GENERATE");

            let count: usize = command.get_one("count").copied().unwrap_or(1);
            let mut existing = HashSet::with_capacity(count);

            while existing.len() < count {
                let note_path = generate_new_note_path(&config);

                if existing.contains(&note_path) {
                    continue;
                }

                term.file(note_path.clone());
                existing.insert(note_path);
            }
            term.flush()?;
        }
        Some(("list", command)) => {
            let Some(files) = note_files(&config) else {
                term.file_error(config.path(), "could not read notes directory!");
                term.end();
                return Ok(());
            };
            let show_tags = command.get_flag("show-tags");

            for file in files {
                if !show_tags {
                    term.list(file);
                    continue;
                }

                let note_tags = match read_note_tags(&file) {
                    Ok(tags) => tags,
                    Err(err) => {
                        term.file_error(file, err);
                        continue;
                    }
                };

                term.list_with_tags(file, note_tags)
            }
            term.flush()?;
        }
        Some(("search", command)) => {
            let Some(tags) = command.get_many::<String>("tags") else {
                term.error("please provide tags to search by!");
                term.end();
                return Ok(());
            };
            let tags: Vec<_> = tags.into_iter().collect();
            let show_tags = command.get_flag("show-tags");

            let Some(files) = note_files(&config) else {
                term.error("could not read notes directory!");
                term.end();
                return Ok(());
            };

            let mut found_tags = Vec::new();
            for file in files {
                let mut note_tags = match read_note_tags(&file) {
                    Ok(tags) => tags,
                    Err(err) => {
                        term.file_error(file, err);
                        term.end();
                        return Ok(());
                    }
                };

                let mut file_tags_match = true;

                for tag in tags.iter() {
                    let tag_position = note_tags
                        .iter()
                        .position(|note_tag| note_tag.contains(tag.as_str()));

                    if let Some(index) = tag_position {
                        if let Some(tag) = note_tags.swap_remove_back(index) {
                            found_tags.push(tag);
                        }
                        continue;
                    }

                    file_tags_match = false;
                    break;
                }
                for tag in found_tags.drain(0..).rev() {
                    note_tags.push_front(tag);
                }

                if !file_tags_match {
                    continue;
                }

                if show_tags {
                    term.list_with_tags(file, note_tags);
                } else {
                    term.list(file);
                }
            }
            term.flush()?;
        }
        Some(("commit", _)) => {
            commit_notes(&config, &mut term);
            term.flush()?;
        }
        Some(("clean", _)) => {
            clean_notes(&config, &mut term);
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

fn note_files(config: &Config) -> Option<impl Iterator<Item = PathBuf>> {
    let result = fs::read_dir(config.path())
        .ok()?
        .into_iter()
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|entry| {
            entry
                .extension()
                .map(|extension| extension == "md")
                .unwrap_or(false)
        });
    Some(result)
}

fn read_note_tags(filepath: &Path) -> Result<VecDeque<String>, String> {
    let file = File::open(filepath).map_err(|err| err.to_string())?;
    let reader = BufReader::new(file);
    let mut tags = VecDeque::new();

    let mut tag: Option<String> = None;

    for line in reader.lines().filter_map(Result::ok) {
        for char in line.chars() {
            let Some(buffer) = &mut tag else {
                if char == '#' {
                    tag = Some(String::new());
                }
                continue;
            };

            let is_letter = matches!(char, 'a' ..= 'z' | 'A' ..= 'Z' );
            let is_number = matches!(char, '0'..='9');
            let is_special = matches!(char, '_' | '-' | '+' | '=');
            let is_umlaut = matches!(char, 'ä' | 'ö' | 'ü' | 'ß');
            if is_letter || is_number || is_special || is_umlaut {
                buffer.push(char);
                continue;
            }

            let Some(buffer) = tag.take() else {
                continue;
            };

            if buffer.is_empty() || tags.contains(&buffer) {
                continue;
            }

            tags.push_back(buffer);
        }

        let Some(buffer) = tag.take() else {
            continue;
        };

        if buffer.is_empty() || tags.contains(&buffer) {
            continue;
        }

        tags.push_back(buffer);
    }

    Ok(tags)
}
