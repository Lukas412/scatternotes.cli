use std::collections::HashSet;
use std::fmt::Display;
use std::ops::Not;

use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub enum Tag {
    Name(String),
    Person(String),
    Todo(TodoTag),
    Action(ActionTag),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub enum TodoTag {
    Todo,
    Done,
    Idea,
    MustDo,
    Asap,
    Remind,
    Review,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub enum ActionTag {
    Load,
    Expand,
    Steps,
    Split,
}

impl Tag {
    pub fn parse_all(input: &str) -> HashSet<Tag> {
        let mut result = HashSet::new();
        let mut input = input;
        while !input.is_empty() {
            let Some(first_char) = input.chars().next() else {
                continue;
            };
            if matches!(first_char, '#' | '@' | '~').not() {
                input = &input[first_char.len_utf8()..];
                continue;
            }
            let Some((remaining, tag)) = Self::parse_single(input) else {
                input = &input[first_char.len_utf8()..];
                continue;
            };
            input = remaining;
            result.insert(tag);
        }
        result
    }

    pub fn parse_single(input: &str) -> Option<(&str, Tag)> {
        let (start, input) = input.split_at(1);
        let text: String = input.chars().take_while(valid_tag_char).collect();
        if text.is_empty() {
            return None;
        }
        let remaining = &input[text.len()..];
        let todo = |tag| Some((remaining, Tag::Todo(tag)));
        let action = |tag| Some((remaining, Tag::Action(tag)));
        return match (start, text.as_str()) {
            ("#", "todo") => todo(TodoTag::Todo),
            ("#", "done") => todo(TodoTag::Done),
            ("#", "idea") => todo(TodoTag::Idea),
            ("#", "must-do") => todo(TodoTag::MustDo),
            ("#", "asap") => todo(TodoTag::Asap),
            ("#", "review") => todo(TodoTag::Review),
            ("#", "remind") => todo(TodoTag::Remind),
            ("#", _) => Some((remaining, Tag::Name(text))),
            ("~", "load") => action(ActionTag::Load),
            ("~", "expand") => action(ActionTag::Expand),
            ("~", "steps") => action(ActionTag::Steps),
            ("~", "split") => action(ActionTag::Split),
            ("@", _) => Some((remaining, Tag::Person(text))),
            _ => None,
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
            Tag::Person(value) => value,
            Tag::Todo(value) => value.text(),
            Tag::Action(value) => value.text(),
        }
    }
}

impl Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.prefix(), self.text())
    }
}

fn valid_tag_char(char: &char) -> bool {
    matches!(char, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '-' | '+' | '=' | 'ä' | 'Ä' | 'ö' | 'Ö' | 'ü' | 'Ü' | 'ß' )
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
