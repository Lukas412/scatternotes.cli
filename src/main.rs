use std::{
    ffi::OsStr,
    fs::{self, read_dir},
    io::{self, Write},
    os::unix::ffi::OsStrExt,
    path::PathBuf,
    str::from_utf8,
};

use clap::{builder::styling::EffectIndexIter, Arg, ArgAction, Command};
use config::Config;
use note::{Note, NoteError};
use tag::Tag;

mod cli;
mod config;
mod note;
mod parse;
mod tag;

fn main() {
    let cli = Command::new("scatternotes")
        .version("0.0.1")
        .about("a cli to create and manage notes")
        .subcommand_required(true)
        .subcommands([
            Command::new("cleanup"),
            Command::new("create")
                .short_flag('c')
                .about("create a new note")
                .args([
                    Arg::new("tags")
                        .short('t')
                        .long("tags")
                        .num_args(0..)
                        .help("tags to put in the new note"),
                    Arg::new("work")
                        .short('w')
                        .long("work")
                        .action(ArgAction::SetTrue)
                        .conflicts_with("personal")
                        .help("put new note in work context"),
                    Arg::new("personal")
                        .short('p')
                        .long("personal")
                        .action(ArgAction::SetTrue)
                        .conflicts_with("work")
                        .help("put new note in work context"),
                    Arg::new("today")
                        .long("today")
                        .action(ArgAction::SetTrue)
                        .conflicts_with("yesterday")
                        .help("add today tag and today's date tag"),
                    Arg::new("yesterday")
                        .long("yesterday")
                        .action(ArgAction::SetTrue)
                        .conflicts_with("today")
                        .help("add today tag and yesterday's date tag"),
                    Arg::new("daily")
                        .long("daily")
                        .action(ArgAction::SetTrue)
                        .conflicts_with_all(["personal", "yesterday"])
                        .help("add daily, work and today's date tag"),
                ]),
            Command::new("add").args([Arg::new("filename").short('f').help("add a file to the ")]),
            Command::new("list").args([Arg::new("tags").action(ArgAction::SetTrue)]),
            Command::new("search").args([Arg::new("tags").num_args(1..)]),
        ])
        .get_matches();

    let config = Config::load();
    let mut stdout = io::stdout();

    match cli.subcommand() {
        Some(("cleanup", command)) => {}
        Some(("create", command)) => {
            let tags = command.get_many::<String>("tags");
            let daily = command.get_flag("daily");
            let work = daily || command.get_flag("work");
            let personal = command.get_flag("personal");
            let today = daily || command.get_flag("today");
            let yesterday = command.get_flag("yesterday");
            let mut note = Note::new().with_tags(
                [
                    work.then(|| Tag::work()),
                    personal.then(|| Tag::personal()),
                    today.then(|| Tag::today()),
                    yesterday.then(|| Tag::yesterday()),
                    daily.then(|| Tag::from_str("daily")),
                ]
                .into_iter()
                .flatten(),
            );
            if let Some(tags) = tags {
                note.add_tags(tags.into_iter().cloned().map(Tag::other))
            };
            let path = note.save(&config).unwrap();
            writeln!(stdout, "{}", path.display()).unwrap();
        }
        Some(("add", command)) => {
            let Some(file) = command.get_one::<String>("filename") else {
                return;
            };
            if let Err(error) = add_file(&config, file.into()) {
                println!("{}", error);
            }
        }
        Some(("list", command)) => {
            let show_tags = command.get_flag("tags");
            let dir = fs::read_dir(config.path()).unwrap();
            let notes = dir.filter_map(|entry| {
                let path = entry.ok()?.path();
                let note = path
                    .is_file()
                    .then(|| Note::load_from(path.clone()))?
                    .ok()?;
                Some((path, note))
            });
            for (path, note) in notes {
                write!(stdout, "{} {}", path.display(), note.key()).unwrap();
                if !show_tags {
                    writeln!(stdout, "").unwrap();
                    continue;
                }
                for tag in note.tags() {
                    write!(stdout, " {}", tag).unwrap();
                }
                writeln!(stdout, "").unwrap();
            }
        }
        Some(("search", command)) => {
            let Some(tags) = command.get_many::<String>("tags") else {
                return;
            };
            iter_notes(&config);
        }
        Some((command, _)) => writeln!(stdout, "command not found: {}", command).unwrap(),
        None => {}
    }
}

fn iter_notes(config: &Config) -> Result<impl Iterator<Item = PathBuf>, io::Error> {
    let iter = read_dir(config.path())?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.file_name().map(is_os_note_name).unwrap_or(false));
    Ok(iter)
}

fn is_os_note_name(name: &OsStr) -> bool {
    from_utf8(name.as_bytes())
        .map(is_note_name)
        .unwrap_or(false)
}

fn is_note_name(name: &str) -> bool {
    let length_is_right = name.len() == 23;
    let extension_is_right = name.ends_with(".md");
    let content_is_right = name[0..20]
        .chars()
        .all(|char| matches!(char, 'A' ..= 'Z' | '0' ..= '9'));
    return length_is_right && extension_is_right && content_is_right;
}

fn add_file(config: &Config, path: PathBuf) -> Result<PathBuf, NoteError> {
    Ok(Note::load_from(path)?.save(config)?)
}
