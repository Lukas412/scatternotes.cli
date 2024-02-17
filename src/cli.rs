use std::path::{Path, PathBuf};

use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[command(version, about)]
#[command(propagate_version = true)]
pub struct ScatternotesCli {
    #[command(subcommand)]
    commands: Commands,
}

impl ScatternotesCli {
    pub fn commands(&self) -> &Commands {
        &self.commands
    }
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Add(AddCommand),
    Sort(SortCommand),
    List(ListCommand),
}


#[derive(Debug, Args)]
pub struct AddCommand {
    file: PathBuf,
}

impl AddCommand {
    pub fn file(&self) -> &Path {
        &self.file
    }
}

#[derive(Debug, Args)]
pub struct SortCommand {
    files: Vec<PathBuf>
}

#[derive(Debug, Args)]
pub struct ListCommand {
    item: ListItem
}

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum ListItem {
    Tags,
    Files
}

