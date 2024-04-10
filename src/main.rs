use std::{
    fs::{self, File},
    io::{self, BufRead, BufReader, Write},
    path::{Path, PathBuf},
};

use clap::{value_parser, Arg, Command};
use config::Config;
use generate::generate_new_note_path;

mod config;
mod generate;

fn main() {
    let cli = Command::new("scatternotes")
        .version("0.0.1")
        .about("a cli to create and manage notes")
        .subcommand_required(true)
        .subcommands([
            Command::new("generate")
                .args([Arg::new("count")
                    .value_parser(value_parser!(usize))
                    .help("the number of notes to generate (1 by default)")])
                .about("generate new note paths that do not yet exist"),
            Command::new("list").about("list all possible note files"),
            Command::new("search")
                .args([Arg::new("tags")
                    .num_args(1..)
                    .help("the tags to search for. (the tags are additive)")])
                .about("search for notes using tags"),
        ])
        .get_matches();

    let config = Config::load();
    let mut stdout = io::stdout();
    let mut stderr = io::stderr();

    match cli.subcommand() {
        Some(("generate", command)) => {
            let count: usize = command.get_one("count").copied().unwrap_or(1);
            let mut existing = Vec::with_capacity(count);

            while existing.len() < count {
                let note_path = generate_new_note_path(&config);

                if existing.contains(&note_path) {
                    continue;
                }

                let _ = writeln!(stdout, "{}", note_path.display());
                existing.push(note_path);
            }
        }
        Some(("list", _)) => {
            let Some(files) = note_files(&config) else {
                let _ = writeln!(
                    stderr,
                    "ERROR \"could not read notes directory\" {}",
                    config.path().display()
                );
                return;
            };

            for file in files {
                let _ = writeln!(stdout, "{}", file.display());
            }
        }
        Some(("search", command)) => {
            let Some(tags) = command.get_many::<String>("tags") else {
                let _ = writeln!(stderr, "ERROR \"please provide tags to search by\"");
                return;
            };
            let tags: Vec<_> = tags.into_iter().collect();

            let Some(files) = note_files(&config) else {
                let _ = writeln!(
                    stderr,
                    "ERROR \"could not read notes directory\" {}",
                    config.path().display()
                );
                return;
            };

            for file in files {
                let note_tags = match read_note_tags(&file) {
                    Ok(tags) => tags,
                    Err(err) => {
                        let _ = writeln!(stderr, "ERROR {} \"{}\"", file.display(), err);
                        return;
                    }
                };

                let mut file_tags_match = true;

                for tag in tags.iter() {
                    let tag = note_tags
                        .iter()
                        .find(|note_tag| note_tag.contains(tag.as_str()));

                    if tag.is_some() {
                        continue;
                    }

                    file_tags_match = false;
                    break;
                }

                if !file_tags_match {
                    continue;
                }

                let _ = writeln!(stdout, "{}", file.display());
            }
        }
        Some((command, _)) => {
            writeln!(stderr, "ERROR \"command not implemented\" \"{}\"", command).unwrap()
        }
        None => {}
    };
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

fn read_note_tags(filepath: &Path) -> Result<Vec<String>, String> {
    let file = File::open(filepath).map_err(|err| err.to_string())?;
    let reader = BufReader::new(file);
    let mut tags = Vec::new();

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
            let is_special = matches!(char, '_' | '-' | '+' | '(' | ')' | '=' | '*' | '%');
            let is_umlaut = matches!(char, 'ä' | 'ö' | 'ü' | 'ß');
            if is_letter || is_number || is_special || is_umlaut {
                buffer.push(char);
                continue;
            }

            let Some(buffer) = tag.take() else {
                continue;
            };

            if buffer.is_empty() {
                continue;
            }

            tags.push(buffer);
        }
    }

    Ok(tags)
}
