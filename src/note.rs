use std::ffi::OsStr;
use std::fs::{self};
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

pub fn note_date(config: &Config, file: &Path) -> Option<NaiveDate> {
    if let Some(date) = file
        .file_name()
        .and_then(OsStr::to_str)
        .and_then(search_date)
    {
        return Some(date);
    }
    fs::read_to_string(file)
        .ok()
        .and_then(|content| search_date(&content))
        .or_else(|| git_log_first_date(config, file))
}

fn search_date(input: &str) -> Option<NaiveDate> {
    for (index, _) in input.char_indices() {
        if let Some(date) = date(&input[index..]) {
            return Some(date);
        }
    }
    None
}

fn date(input: &str) -> Option<NaiveDate> {
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
    let date = search_date(&stdout);
    date
}

pub fn note_header(file: &Path) -> Option<String> {
    let content = fs::read_to_string(file).ok()?;

    fn sep(input: &str) -> IResult<&str, &str> {
        tag("---\n")(input)
    }

    fn key_value_pair(input: &str) -> IResult<&str, (&str, char, &str)> {
        alt((
            tuple((space0, char('-'), take_until("\n"))),
            tuple((alphanumeric1, char(':'), take_until("\n"))),
        ))(input)
    }

    if delimited(sep, key_value_pair, sep)(&content).is_ok() {
        println!("{}", content.lines().take(6).join("\n"));
    }
    None
}
