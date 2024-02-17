use std::{fs, io};

use clap::Parser;
use cli::{
    ScatternotesCli,
    Commands::{Add, Sort, List}
};
use config::Config;
use note::Note;

mod cli;
mod config;
mod note;
mod parse;

fn main() -> io::Result<()> {
    let cli = ScatternotesCli::parse();
    let config = Config::load();
    
    match cli.commands() {
        Add(command) => {
            let file = command.file();
            Note::load_from(file)?.save()?;
        },
        Sort(command) => {},
        List(command) => {},
    }

    Ok(())
}

