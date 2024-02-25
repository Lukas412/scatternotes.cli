use std::{fs, io::{self, Write}, path::Path};

use clap::Parser;
use cli::{
    Commands::{Add, List, Sort}, ScatternotesCli
};
use config::Config;
use note::{Note, NoteError};

mod cli;
mod config;
mod note;
mod parse;

fn main() {
    let cli = ScatternotesCli::parse();
    let config = Config::load();
    let mut stdout = io::stdout();
    
    match cli.commands() {
        Add(command) => {
            let file = command.file();
            if let Err(error) = add_file(&config, file) {
                println!("{}", error);
            }
        },
        Sort(_command) => {},
        List(command) => {
            let show_tags = command.tags();
            let dir =fs::read_dir(config.path()).unwrap();
            let notes = dir
                .filter_map(|entry| {
                    let path = entry.ok()?.path();
                    let note = path.is_file().then(|| Note::load_from(&path))?.ok()?;
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
        },
    }
}

fn add_file<'a>(config: &'a Config, path: &'a Path) -> Result<(), NoteError<'a>> {
    Ok(Note::load_from(path)?.save(config)?)
}

