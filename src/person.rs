use std::borrow::Cow;
use std::collections::HashSet;
use std::fmt::Display;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufWriter, Read, Write};
use std::path::PathBuf;

use serde::Serialize;

use crate::config::Config;
use crate::note::Note;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct Person<'a> {
    name: Cow<'a, str>,
}

impl Person<'static> {
    pub fn load_all_persons(config: &Config) -> eyre::Result<HashSet<Self>> {
        let content = fs::read_to_string(persons_meta_file(config))?;
        Ok(Self::load_all_persons_from(&content))
    }

    pub fn load_all_persons_from(content: &str) -> HashSet<Self> {
        content
            .lines()
            .filter(|line| !line.is_empty() || !line.starts_with("#"))
            .flat_map(|line| {
                line.trim()
                    .split(|char| !matches!(char, 'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_'))
                    .filter(|name| !name.is_empty())
            })
            .map(|name| Self::new(name.to_owned()))
            .collect()
    }

    pub fn search_all_persons(config: &Config) -> eyre::Result<HashSet<Self>> {
        let mut persons = HashSet::new();
        for note in Note::all_notes(config)? {
            for person in note.persons() {
                persons.insert(person.into_owned());
            }
        }
        Ok(persons)
    }

    pub fn save_persons<'a>(config: &Config, persons: &HashSet<Person<'a>>) -> eyre::Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .read(true)
            .append(true)
            .open(persons_meta_file(config))?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let previous = Self::load_all_persons_from(&content);
        let mut writer = BufWriter::new(file);
        for person in persons.difference(&previous) {
            write!(writer, "\n{}", person.name())?;
        }
        Ok(())
    }
}

impl<'a> Person<'a> {
    pub fn new(name: impl Into<Cow<'a, str>>) -> Self {
        Self { name: name.into() }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn into_owned(&self) -> Person<'static> {
        let name = self.name.clone().to_lowercase();
        Person {
            name: Cow::from(name),
        }
    }
}

impl<'a> From<&'a str> for Person<'a> {
    fn from(value: &'a str) -> Self {
        Person::new(value)
    }
}

impl<'a> From<String> for Person<'a> {
    fn from(mut value: String) -> Self {
        value.make_ascii_lowercase();
        Person::new(value)
    }
}

impl<'a> Display for Person<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (first, text) = self.name.split_at(1);
        if let Some(first) = first.chars().next() {
            write!(f, "{}", first.to_ascii_uppercase())?;
        }
        write!(f, "{}", text)
    }
}

fn persons_meta_file(config: &Config) -> PathBuf {
    config.meta("persons.txt")
}
