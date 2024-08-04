use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use std::process::Command;

use chrono::NaiveDate;
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{alphanumeric1, char, space0, u16, u8};
use nom::sequence::{delimited, tuple};
use nom::IResult;

use crate::config::Config;

use super::dto::MutNote;
use super::Note;

pub struct NotesRepository {
    config: Config,
}

impl NotesRepository {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn search<'a>(
        &self,
        queries: &'a [&'a String],
    ) -> eyre::Result<impl Iterator<Item = Note> + 'a> {
        Ok(self.all_notes()?.filter_map(|mut note| {
            let matches = queries
                .iter()
                .all(|query| note.tags().iter().any(|tag| tag.text().contains(*query)));
            if !matches {
                return None;
            }
            MutNote::move_tags_to_front(&mut note, queries);
            Some(note)
        }))
    }

    pub fn all_notes(&self) -> eyre::Result<impl Iterator<Item = Note>> {
        let result = fs::read_dir(self.config.path())?
            .into_iter()
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .filter(|entry| {
                entry
                    .extension()
                    .map(|extension| extension == "md")
                    .unwrap_or(false)
            })
            .filter_map(|path| Note::load(path).ok());
        Ok(result)
    }

    /// Determine note date by searching in the following order:
    /// - in the note filename
    /// - the content of the file
    /// - the git history when the file was added
    pub fn search_date(&self, config: &Config, file: &Path) -> Option<NaiveDate> {
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
