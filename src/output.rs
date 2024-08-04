use std::fmt::Display;
use std::path::{Path, PathBuf};

use serde::Serialize;
use termfmt::{termarrow, termarrow_fg, termerr, termh1, termh2, terminfo, BundleFmt, Fg, TermFmt};

use crate::config::Config;
use crate::note::{Note, Tag};

use self::tags::pretty_print_with_tags;

mod tags;

pub type Term = TermFmt<DataBundle>;

#[derive(Default, Serialize)]
pub struct DataBundle {
    config: Option<Config>,
    #[serde(rename = "output", skip_serializing_if = "Vec::is_empty")]
    file_output: Vec<PathBuf>,
    #[serde(rename = "output", skip_serializing_if = "Vec::is_empty")]
    list_output: Vec<ListEntryFmt>,
    #[serde(rename = "output", skip_serializing_if = "Vec::is_empty")]
    command_output: Vec<String>,
    #[serde(rename = "remove", skip_serializing_if = "Vec::is_empty")]
    cleanup_remove_output: Vec<ListEntryFmt>,
    #[serde(rename = "rename", skip_serializing_if = "Vec::is_empty")]
    cleanup_rename_output: Vec<PathBuf>,
    #[serde(rename = "todos", skip_serializing_if = "Vec::is_empty")]
    todos_output: Vec<TodoFmt>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    hint: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    info: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    error: Vec<String>,
}

#[derive(Serialize)]
pub struct ListEntryFmt {
    file: PathBuf,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    tags: Vec<Tag>,
}

#[derive(Serialize)]
pub struct TodoFmt {
    file: PathBuf,
    content: String,
}

impl BundleFmt for DataBundle {
    type Config = Config;

    fn new(config: Config) -> Self {
        Self {
            config: Some(config),
            ..Default::default()
        }
    }

    fn clear(&mut self) {
        self.hint.clear();
        self.info.clear();
        self.error.clear();
        self.file_output.clear();
        self.list_output.clear();
        self.command_output.clear();
    }
}

pub trait OutputFmt {
    fn hint(&mut self, value: impl Display);
    fn info(&mut self, value: impl Display);
    fn error(&mut self, value: impl Display);
    fn file_error(&mut self, file: impl AsRef<Path>, value: impl Display);
    fn headline(&mut self, value: impl Display);
    fn command(&mut self, value: &str);
    fn file(&mut self, file: impl AsRef<Path>);
    fn list(&mut self, note: Note, with_tags: bool);
    fn cleanup_remove(&mut self, note: &Note, with_tags: bool);
    fn cleanup_rename(&mut self, note: &Note);
    fn todo(&mut self, file: impl AsRef<Path>, content: &str);
    fn end(&mut self);
}

impl OutputFmt for TermFmt<DataBundle> {
    fn hint(&mut self, value: impl Display) {
        self.bundle(|bundle| bundle.hint.push(format!("{}", value)));
        self.plain(&value);
        if self.is_interactive() {
            termarrow(value);
        }
    }

    fn info(&mut self, value: impl Display) {
        self.bundle(|bundle| bundle.info.push(format!("{}", value)));
        self.plain(&value);
        if self.is_interactive() {
            terminfo(value);
        }
    }

    fn error(&mut self, value: impl Display) {
        self.bundle(|bundle| bundle.error.push(format!("{}", value)));
        self.plain(&value);
        if self.is_interactive() {
            termerr(value);
        }
    }

    fn file_error(&mut self, file: impl AsRef<Path>, value: impl Display) {
        self.error(format_args!("{}: {}", file.as_ref().display(), value));
    }

    fn headline(&mut self, value: impl Display) {
        if self.is_interactive() {
            termh1(format_args!("{}", value));
        }
    }

    fn command(&mut self, value: &str) {
        self.bundle(|bundle| bundle.command_output.push(value.to_owned()));
        self.plain(value);
        if self.is_interactive() {
            termarrow(value);
        }
    }

    fn file(&mut self, file: impl AsRef<Path>) {
        self.bundle(|bundle| bundle.file_output.push(file.as_ref().to_owned()));
        self.plain(file.as_ref().display());
        if self.is_interactive() {
            termarrow(file.as_ref().display());
        }
    }

    fn list(&mut self, note: Note, with_tags: bool) {
        self.bundle(|bundle| {
            bundle.list_output.push(ListEntryFmt {
                file: note.path().to_owned(),
                tags: with_tags.then(|| note.tags().to_vec()).unwrap_or_default(),
            })
        });
        if self.is_plain() {
            print!("{}", note.path().display());
            if with_tags {
                print!("|{}", note.join_tags(",").unwrap());
            }
            println!();
        }
        if self.is_interactive() {
            termh1(note.path().display());
            if with_tags {
                termarrow(note.join_tags(", ").unwrap());
            }
        }
    }

    fn cleanup_remove(&mut self, note: &Note, with_tags: bool) {
        self.bundle(|bundle| {
            bundle.cleanup_remove_output.push(ListEntryFmt {
                file: note.path().to_owned(),
                tags: with_tags.then(|| note.tags().to_vec()).unwrap_or_default(),
            })
        });
        if self.is_plain() {
            print!("delete {}", note.path().display());
            if with_tags {
                print!("|{}", note.join_tags(",").unwrap());
            }
            println!();
        }
        if self.is_interactive() {
            termarrow_fg(Fg::Red, note.path().display());
        }
    }

    fn cleanup_rename(&mut self, note: &Note) {
        self.bundle(|bundle| bundle.cleanup_rename_output.push(note.path().to_owned()));
        self.plain(format_args!("rename {}", note.path().display()));
        if self.is_interactive() {
            termarrow_fg(Fg::Yellow, note.path().display());
        }
    }

    fn todo(&mut self, file: impl AsRef<Path>, content: &str) {
        self.bundle(|bundle| {
            bundle.todos_output.push(TodoFmt {
                file: file.as_ref().to_owned(),
                content: content.to_owned(),
            })
        });
        if self.is_plain() {
            println!("{}", file.as_ref().display());
            println!("{}", content);
        }
        if self.is_interactive() {
            termh1(file.as_ref().display());
            pretty_print_with_tags(content);
        }
    }

    fn end(&mut self) {
        if self.is_interactive() {
            println!();
        }
    }
}
