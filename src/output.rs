use std::fmt::Display;
use std::path::{Path, PathBuf};

use serde::Serialize;
use termfmt::{
    termarrow, termarrow_fg, termerr, termh1, terminfo, BundleFmt, DataFmt, Fg, TermFmt,
};

use crate::config::Config;
use crate::note::{Note, Tag};

pub enum OutputData {
    Info(String),
    Error(String),
    Headline(String),
    Command(String),
    File(PathBuf),
    List(ListEntry),
    CleanupRemove(ListEntry),
    CleanupRename(PathBuf),
    End,
}

#[derive(Default, Serialize)]
pub struct DataBundle {
    config: Option<Config>,
    #[serde(rename = "output", skip_serializing_if = "Vec::is_empty")]
    generate_output: Vec<PathBuf>,
    #[serde(rename = "output", skip_serializing_if = "Vec::is_empty")]
    list_output: Vec<ListEntry>,
    #[serde(rename = "output", skip_serializing_if = "Vec::is_empty")]
    command_output: Vec<String>,
    #[serde(rename = "remove", skip_serializing_if = "Vec::is_empty")]
    cleanup_remove_output: Vec<ListEntry>,
    #[serde(rename = "rename", skip_serializing_if = "Vec::is_empty")]
    cleanup_rename_output: Vec<PathBuf>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    info: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    error: Vec<String>,
}

#[derive(Serialize)]
pub struct ListEntry {
    #[serde(skip)]
    note: Note,
    file: PathBuf,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    tags: Vec<Tag>,
}

impl DataBundle {
    pub fn new(config: Config) -> Self {
        Self {
            config: Some(config),
            ..Default::default()
        }
    }
}

impl DataFmt for OutputData {
    fn plain(self) {
        match self {
            OutputData::Info(value) => println!("{}", value),
            OutputData::Error(value) => eprintln!("{}", value),
            OutputData::Command(value) => println!("{}", value),
            OutputData::File(file) => println!("{}", file.display()),
            OutputData::List(ListEntry { note, file, tags }) => {
                if tags.is_empty() {
                    println!("{}", file.display())
                } else {
                    println!("{}|{}", file.display(), note.join_tags(",").unwrap())
                }
            }
            OutputData::CleanupRemove(ListEntry { note, file, tags }) => {
                if tags.is_empty() {
                    println!("delete {}", file.display())
                } else {
                    println!(
                        "delete {}|{}",
                        file.display(),
                        note.join_tags(", ").unwrap()
                    )
                }
            }
            OutputData::CleanupRename(file) => {
                println!("rename {}", file.display())
            }
            OutputData::Headline(_) | OutputData::End => {}
        }
    }

    fn interactive(self) {
        match self {
            OutputData::Info(value) => terminfo(value),
            OutputData::Error(value) => termerr(value),
            OutputData::Headline(value) => termh1(value),
            OutputData::Command(value) => termarrow(value),
            OutputData::File(file) => termarrow(file.display()),
            OutputData::List(ListEntry { note, file, tags }) => {
                termh1(file.display());
                if !tags.is_empty() {
                    termarrow(note.join_tags(", ").unwrap())
                }
            }
            OutputData::CleanupRemove(ListEntry { file, .. }) => {
                termarrow_fg(Fg::Red, format_args!("delete: {}", file.display()));
            }
            OutputData::CleanupRename(file) => {
                termarrow_fg(Fg::Yellow, format_args!("rename: {}", file.display()));
            }
            OutputData::End => println!(),
        }
    }
}

impl BundleFmt for DataBundle {
    type Data = OutputData;

    fn push(&mut self, value: OutputData) {
        match value {
            OutputData::File(value) => self.generate_output.push(value),
            OutputData::List(value) => self.list_output.push(value),
            OutputData::Command(value) => self.command_output.push(value),
            OutputData::Info(value) => self.info.push(value),
            OutputData::CleanupRemove(value) => self.cleanup_remove_output.push(value),
            OutputData::CleanupRename(value) => self.cleanup_rename_output.push(value),
            OutputData::Error(_) | OutputData::Headline(_) | OutputData::End => {}
        }
    }

    fn clear(&mut self) {
        self.generate_output.clear();
        self.list_output.clear();
        self.command_output.clear();
    }

    fn csv<Writer>(&self, mut writer: csv::Writer<Writer>) -> eyre::Result<()>
    where
        Writer: std::io::Write,
    {
        for output in self.generate_output.iter() {
            writer.serialize(output)?;
        }
        for output in self.list_output.iter() {
            writer.serialize((output.file.clone(), output.note.join_tags(" ").unwrap()))?;
        }
        for output in self.command_output.iter() {
            writer.serialize(output)?;
        }
        for output in self.info.iter() {
            writer.serialize(output)?;
        }
        for output in self.error.iter() {
            writer.serialize(output)?;
        }
        Ok(())
    }
}

pub trait OutputFmt {
    fn info(&mut self, value: impl Display);
    fn error(&mut self, value: impl Display);
    fn file_error(&mut self, file: impl AsRef<Path>, value: impl Display);
    fn headline(&mut self, value: impl Display);
    fn command(&mut self, value: &str);
    fn file(&mut self, file: impl Into<PathBuf>);
    fn list(&mut self, note: Note, with_tags: bool);
    fn cleanup_remove(&mut self, note: Note, with_tags: bool);
    fn cleanup_rename(&mut self, note: &Note);
    fn end(&mut self);
}

impl OutputFmt for TermFmt<OutputData, DataBundle> {
    fn info(&mut self, value: impl Display) {
        self.output(OutputData::Info(format!("{}", value)));
    }

    fn error(&mut self, value: impl Display) {
        self.output(OutputData::Error(format!("{}", value)));
    }

    fn file_error(&mut self, file: impl AsRef<Path>, value: impl Display) {
        self.output(OutputData::Error(format!(
            "{}: {}",
            file.as_ref().display(),
            value
        )));
    }

    fn headline(&mut self, value: impl Display) {
        self.output(OutputData::Headline(format!("{}", value)));
    }

    fn command(&mut self, value: &str) {
        self.output(OutputData::Command(value.to_owned()))
    }

    fn file(&mut self, file: impl Into<PathBuf>) {
        self.output(OutputData::File(file.into()));
    }

    fn list(&mut self, note: Note, with_tags: bool) {
        let tags = with_tags.then(|| note.tags().to_vec()).unwrap_or_default();
        self.output(OutputData::List(ListEntry {
            file: note.path().to_owned(),
            note,
            tags,
        }));
    }

    fn cleanup_remove(&mut self, note: Note, with_tags: bool) {
        let tags = with_tags.then(|| note.tags().to_vec()).unwrap_or_default();
        self.output(OutputData::CleanupRemove(ListEntry {
            file: note.path().to_owned(),
            note,
            tags,
        }));
    }

    fn cleanup_rename(&mut self, note: &Note) {
        self.output(OutputData::CleanupRename(note.path().to_owned()))
    }

    fn end(&mut self) {
        self.output(OutputData::End);
    }
}
