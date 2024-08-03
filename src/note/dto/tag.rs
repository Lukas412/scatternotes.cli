use std::fmt::Display;
use std::ops::Not;

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub enum Tag {
    Name(String),
    Person(String),
    Todo { done: bool },
}

impl Tag {
    pub fn parse_all(input: &str) -> Vec<Tag> {
        let mut result = Vec::new();
        let mut input = input;
        while !input.is_empty() {
            let Some(first_char) = input.chars().next() else {
                continue;
            };
            if matches!(first_char, '#' | '@').not() {
                input = &input[first_char.len_utf8()..];
                continue;
            }
            let Some((remaining, tag)) = Self::parse_single(input) else {
                input = &input[first_char.len_utf8()..];
                continue;
            };
            input = remaining;
            result.push(tag);
        }
        result
    }

    fn parse_single(input: &str) -> Option<(&str, Tag)> {
        let (start, input) = input.split_at(1);
        let text: String = input.chars().take_while(valid_tag_char).collect();
        if text.is_empty() {
            return None;
        }
        let remaining = &input[text.len()..];
        if start == "#" {
            if text == "todo" {
                return Some((remaining, Tag::Todo { done: false }));
            }
            if text == "done" {
                return Some((remaining, Tag::Todo { done: true }));
            }
            return Some((remaining, Tag::Name(text)));
        }
        if start == "@" {
            return Some((remaining, Tag::Person(text)));
        }
        None
    }

    pub fn text(&self) -> &str {
        match self {
            Tag::Name(value) => value,
            Tag::Person(value) => value,
            Tag::Todo { done: false } => "todo",
            Tag::Todo { done: true } => "done",
        }
    }
}

impl Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let start = match self {
            Tag::Name(_) | Tag::Todo { .. } => '#',
            Tag::Person(_) => '@',
        };
        write!(f, "{}{}", start, self.text())
    }
}

fn valid_tag_char(char: &char) -> bool {
    matches!(char, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '-' | '+' | '=' | 'ä' | 'Ä' | 'ö' | 'Ö' | 'ü' | 'Ü' | 'ß' )
}