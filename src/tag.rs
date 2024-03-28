use std::fmt::Display;

use chrono::{DateTime, Datelike, Duration, Local, Timelike};
pub use config::TagConfig;

mod config;

#[derive(PartialEq, Eq)]
pub enum Tag {
    Context(ContextTag),
    Time(TimeTag),
    Static(StaticTag),
    Other(OtherTag)
}

#[derive(PartialEq, Eq)]
pub enum ContextTag {
    Work,
    Personal
}

#[derive(PartialEq, Eq)]
pub struct PersonTag {
    value: String,
}

#[derive(PartialEq, Eq)]
pub enum TimeTag {
    DateTime(Date, Time),
    Date(Date),
    Time(Time)
}

#[derive(PartialEq, Eq)]
pub struct Date {
    year: u16,
    month: u8,
    day: u8,
}

#[derive(PartialEq, Eq)]
pub struct Time {
    hour: u8,
    minute: u8,
}

#[derive(PartialEq, Eq)]
pub struct StaticTag(&'static str);

#[derive(PartialEq, Eq)]
pub struct OtherTag {
    value: String
}

impl Tag {
    pub fn work() -> Self {
        Self::Context(ContextTag::Work)
    }

    pub fn personal() -> Self {
        Self::Context(ContextTag::Personal)
    }

    pub fn today() -> Self {
        let date = Local::now();
        Self::Time(TimeTag::Date(Date::new(&date)))
    }

    pub fn yesterday() -> Self {
        let date = Local::now() - Duration::try_days(1).unwrap();
        Self::Time(TimeTag::Date(Date::new(&date)))
    }

    pub fn now() -> Self {
        let date = Local::now();
        Self::Time(TimeTag::Time(Time::new(&date)))
    }

    pub fn from_str(value: &'static str) -> Self {
        Self::Static(StaticTag(value))
    }

    pub fn other(value: String) -> Self {
        Self::Other(OtherTag { value })
    }
}

impl Date {
    fn new(value: &DateTime<Local>) -> Self {
        Self {
            year: value.year() as u16,
            month: value.month() as u8,
            day: value.day() as u8,
        }
    }
}

impl Time {
    fn new(value: &DateTime<Local>) -> Self {
        Self {
            hour: value.hour() as u8,
            minute: value.minute() as u8,
        }
    }
}

impl Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Context(tag) => write!(f, "{}", tag),
            Self::Time(tag) => write!(f, "{}", tag),
            Self::Static(tag) => write!(f, "{}", tag),
            Self::Other(tag) => write!(f, "{}", tag),
        }
    }
}

impl Display for ContextTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Work => write!(f, "#work"),
            Self::Personal => write!(f, "#personal"),
        }
    }
}

impl Display for TimeTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DateTime(date, time) => write!(f, "#{}_{}", date, time),
            Self::Date(date) => write!(f, "#{}", date),
            Self::Time(time) => write!(f, "#{}", time),
        }
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}-{:02}", self.hour, self.minute)
    }
}

impl Display for StaticTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{}", self.0)
    }
}

impl Display for OtherTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{}", self.value)
    }
}

