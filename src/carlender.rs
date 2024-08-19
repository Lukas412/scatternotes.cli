use std::borrow::Cow;
use std::fs;
use std::path::PathBuf;

use chrono::{NaiveDate, NaiveTime};
use termfmt::parse::time::parse_time;

use crate::config::Config;

pub struct Carlender {
    path: PathBuf,
    date: NaiveDate,
    content: String,
}

pub struct Event<'a> {
    start: NaiveTime,
    end: Option<NaiveTime>,
    content: Cow<'a, str>,
}

impl Carlender {
    pub fn load(config: &Config, date: NaiveDate) -> eyre::Result<Self> {
        let path = config.carlender(date);
        let content = fs::read_to_string(&path)?;
        Ok(Self {
            path,
            date,
            content,
        })
    }

    pub fn events(&self) -> impl Iterator<Item = Event> {
        self.parts().filter_map(|part| Event::parse(part))
    }
}

impl Carlender {
    fn parts(&self) -> impl Iterator<Item = &str> {
        self.content.split("\n\n")
    }
}

impl<'a> Event<'a> {
    fn parse(input: &'a str) -> Option<Self> {
        let (times, content) = input.split_once('\n')?;
        let (start, end) = parse_start_and_end(input)?;
        let content = content.trim().into();
        Some(Self {
            start,
            end,
            content,
        })
    }
}

fn parse_start_and_end(input: &str) -> Option<(NaiveTime, Option<NaiveTime>)> {
    let input = input.trim_start();
    let (input, start) = parse_time(input)?;
    let mut input = input.trim_start();
    for prefix in ["to", "bis", "unitl", "-"] {
        if let Some(remaining) = input.strip_prefix(prefix) {
            input = remaining;
            break;
        }
    }
    let input = input.trim_start();
    let end = parse_time(input).map(|(_, end)| end);
    Some((start, end))
}
