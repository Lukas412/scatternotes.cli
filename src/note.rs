use core::{slice, str};
use std::collections::HashSet;
use std::ffi::OsStr;
use std::fmt::Write;
use std::fs::{self, read_to_string};
use std::path::{Path, PathBuf};
use std::process::Command;

use chrono::NaiveDate;
use itertools::Itertools;
use nom::character::complete::{char, u16, u8};
use nom::IResult;

use crate::config::Config;
use crate::tag::Tag;

#[derive(Clone)]
pub struct Note {
    path: PathBuf,
    content: String,
    tags: HashSet<Tag<'static>>,
}

impl Note {
    pub fn search<'a>(
        config: &Config,
        queries: &'a [&'a String],
    ) -> eyre::Result<impl Iterator<Item = Self> + 'a> {
        Ok(Self::all_notes(config)?
            .filter(|note| queries.iter().all(|query| note.any_tag_contains(query))))
    }

    pub fn all_notes(config: &Config) -> eyre::Result<impl Iterator<Item = Note>> {
        let result = fs::read_dir(config.path())?
            .into_iter()
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .filter(|entry| {
                entry
                    .extension()
                    .map(|extension| extension == "md")
                    .unwrap_or(false)
            })
            .flat_map(|path| Note::load(path).ok());
        Ok(result)
    }

    pub fn load(path: PathBuf) -> eyre::Result<Self> {
        let content = read_to_string(&path)?;
        let static_content =
            str::from_utf8(unsafe { slice::from_raw_parts(content.as_ptr(), content.len()) })?;
        let tags = Tag::all(static_content);
        Ok(Self {
            path,
            content,
            tags,
        })
    }

    pub fn name(&self) -> &str {
        self.path.file_name().and_then(OsStr::to_str).unwrap()
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn cloned_tags<'a>(&'a self) -> impl Iterator<Item = String> + 'a {
        self.tags.iter().map(|tag| tag.text().to_owned())
    }

    pub fn has_tag(&self, name: &str) -> bool {
        self.tags.iter().any(|tag| tag == name)
    }

    pub fn any_tag_contains(&self, name: &str) -> bool {
        self.tags.iter().any(|tag| tag.contains(name))
    }

    pub fn join_tags(&self, separator: &str) -> eyre::Result<String> {
        join_tags_impl(self.tags.iter(), separator)
    }

    pub fn join_tags_sorted(&self, separator: &str, front: &[&String]) -> eyre::Result<String> {
        let iter = self.tags.iter().sorted_by_key(|tag| {
            let tag_text = tag.text();
            !front.iter().any(|name| tag_text.contains(*name))
        });
        join_tags_impl(iter, separator)
    }

    pub fn persons(&self) -> impl Iterator<Item = &str> {
        self.tags
            .iter()
            .filter_map(|tag| tag.is_person().then(|| tag.text()))
    }

    pub fn parts(&self) -> impl Iterator<Item = &str> {
        self.content
            .split("\n\n")
            .map(|part| part.trim())
            .filter(|part| !part.is_empty())
    }

    /// Determine note date by searching in the following order:
    /// - in the note filename
    /// - the content of the file
    /// - the git history when the file was added
    pub fn search_date(config: &Config, file: &Path) -> Option<NaiveDate> {
        if let Some(date) = file
            .file_name()
            .and_then(OsStr::to_str)
            .and_then(search_date_in_text)
        {
            return Some(date);
        }
        fs::read_to_string(file)
            .ok()
            .and_then(|content| search_date_in_text(&content))
            .or_else(|| git_log_first_date(config, file))
    }
}

fn search_date_in_text(input: &str) -> Option<NaiveDate> {
    input
        .char_indices()
        .find_map(|(index, _)| parse_date(&input[index..]))
}

fn parse_date(input: &str) -> Option<NaiveDate> {
    fn date_impl(input: &str) -> IResult<&str, Option<NaiveDate>> {
        let (input, year) = u16(input)?;
        let (input, _) = char('-')(input)?;
        let (input, month) = u8(input)?;
        let (input, _) = char('-')(input)?;
        let (input, day) = u8(input)?;
        Ok((
            input,
            NaiveDate::from_ymd_opt(year as i32, month as u32, day as u32),
        ))
    }

    let (_, date) = date_impl(input).ok()?;
    date
}

fn git_log_first_date(config: &Config, file: &Path) -> Option<NaiveDate> {
    let output = Command::new("git")
        .args(["log", "--reverse", "--date=short", file.to_str()?])
        .current_dir(config.path())
        .output()
        .ok()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let date = search_date_in_text(&stdout);
    date
}

fn join_tags_impl<'a, 'b>(
    mut iter: impl Iterator<Item = &'a Tag<'a>>,
    separator: &'b str,
) -> eyre::Result<String> {
    let mut b = String::new();
    if let Some(first) = iter.next() {
        write!(b, "{}", first)?;
    }
    for tag in iter {
        write!(b, "{}{}", separator, tag)?;
    }
    Ok(b)
}
