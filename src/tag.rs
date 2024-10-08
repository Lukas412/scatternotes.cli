use std::borrow::Cow;
use std::collections::HashSet;
use std::fmt::Display;
use std::ops::Not;

use serde::Serialize;

use crate::person::Person;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub enum Tag<'a> {
    Name(Cow<'a, str>),
    Person(Person<'a>),
    Todo(TodoTag),
    Action(ActionTag),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize)]
pub enum TodoTag {
    Todo,
    Done,
    Idea,
    MustDo,
    Asap,
    Remind,
    Review,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize)]
pub enum ActionTag {
    Load,
    Expand,
    Steps,
    Split,
}

impl<'a> Tag<'a> {
    pub fn all(input: &'a str) -> HashSet<Self> {
        let mut result = HashSet::new();
        for mut input in iter_non_code_blocks(input) {
            while !input.is_empty() {
                match Self::parse_next(input) {
                    Ok((_, remaining, tag)) => {
                        input = remaining;
                        result.insert(tag);
                    }
                    Err(remaining) => input = remaining,
                }
            }
        }
        result
    }

    pub fn parse_next(input: &'a str) -> Result<(&'a str, &'a str, Tag<'a>), &'a str> {
        let Some(index) = input.find(|char| matches!(char, '#' | '@' | '~')) else {
            return Err("");
        };
        let (preceding, input) = input.split_at(index);
        let (start, input) = input.split_at(1);
        let (text, remaining) = input.split_once(
            |char| matches!(char, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '-' | '+' | '=' | 'ä' | 'Ä' | 'ö' | 'Ö' | 'ü' | 'Ü' | 'ß' ).not()
        )
        .unwrap_or((input, ""));
        if text.is_empty() {
            return Err("");
        }
        let todo = |tag| Ok((preceding, remaining, Tag::Todo(tag)));
        let action = |tag| Ok((preceding, remaining, Tag::Action(tag)));
        return match (start, text) {
            ("#", "todo") => todo(TodoTag::Todo),
            ("#", "done") => todo(TodoTag::Done),
            ("#", "idea") => todo(TodoTag::Idea),
            ("#", "must-do") => todo(TodoTag::MustDo),
            ("#", "asap") => todo(TodoTag::Asap),
            ("#", "review") => todo(TodoTag::Review),
            ("#", "remind") => todo(TodoTag::Remind),
            ("#", _) => Ok((preceding, remaining, Tag::Name(text.into()))),
            ("~", "load") => action(ActionTag::Load),
            ("~", "expand") => action(ActionTag::Expand),
            ("~", "steps") => action(ActionTag::Steps),
            ("~", "split") => action(ActionTag::Split),
            ("@", _) => Ok((preceding, remaining, Tag::Person(text.into()))),
            _ => Err(input),
        };
    }

    pub fn prefix(&self) -> char {
        match self {
            Self::Name(_) | Self::Todo(_) => '#',
            Self::Person(_) => '@',
            Self::Action(_) => '~',
        }
    }

    pub fn text(&self) -> &str {
        match self {
            Tag::Name(value) => value,
            Tag::Person(value) => value.name(),
            Tag::Todo(value) => value.text(),
            Tag::Action(value) => value.text(),
        }
    }

    pub fn contains(&self, other: &str) -> bool {
        self.text().contains(other)
    }

    pub fn is_person(&self) -> bool {
        matches!(self, Self::Person(_))
    }
}

impl<'a> PartialEq<str> for Tag<'a> {
    fn eq(&self, other: &str) -> bool {
        *self.text() == *other
    }
}

impl<'a> Display for Tag<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.prefix(), self.text())
    }
}

impl TodoTag {
    pub fn text(&self) -> &str {
        match self {
            Self::Todo => "todo",
            Self::Done => "done",
            Self::Idea => "idea",
            Self::MustDo => "must-do",
            Self::Asap => "asap",
            Self::Remind => "remind",
            Self::Review => "review",
        }
    }
}

impl ActionTag {
    pub fn text(&self) -> &str {
        match self {
            Self::Load => "load",
            Self::Expand => "expand",
            Self::Steps => "steps",
            Self::Split => "split",
        }
    }
}

fn iter_non_code_blocks(input: &str) -> impl Iterator<Item = &str> {
    let mut in_code_block = false;
    input
        .split("```")
        .filter(move |_| {
            in_code_block = !in_code_block;
            in_code_block
        })
        .flat_map(|text| {
            let mut in_code_inline = false;
            text.split('`').filter(move |_| {
                in_code_inline = !in_code_inline;
                in_code_inline
            })
        })
}
