use std::path::{Path, PathBuf};

use clap::{Args, Parser, Subcommand};

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
    file: PathBuf
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
    #[arg(short, long, default_value="false")]
    tags: bool
}

impl ListCommand {
    pub fn tags(&self) -> bool {
        self.tags
    }
}

