use std::ffi::OsStr;
use std::fs;
use std::path::Path;

use chrono::NaiveDate;
use nom::bytes::complete::tag;
use nom::character::complete::{char, u16, u8};
use nom::sequence::delimited;
use nom::IResult;

pub fn note_date(file: &Path) -> Option<NaiveDate> {
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

pub fn note_header(file: &Path) -> Option<String> {
    let content = fs::read_to_string(file).ok()?;

    fn sep(input: &str) -> IResult<&str, &str> {
        tag("---\n")(input)
    }

    fn key_value_pair(input: &str) -> IResult<&str, &str> {
        todo!()
    }

    delimited(sep, key_value_pair, sep);
    None
}
