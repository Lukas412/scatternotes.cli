use std::fmt::Display;
use std::ops::Not;

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub enum Tag {
    Name(String),
    Person(String),
    Todo(TodoTag),
    Action(ActionTag),
}

#[derive(Debug, Clone, Serialize)]
pub enum TodoTag {
    Todo,
    Done,
    Idea,
    MustDo,
    Asap,
    Remind,
    Review,
}

#[derive(Debug, Clone, Serialize)]
pub enum ActionTag {
    Load,
    Expand,
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

    pub fn parse_single(input: &str) -> Option<(&str, Tag)> {
        let (start, input) = input.split_at(1);
        let text: String = input.chars().take_while(valid_tag_char).collect();
        if text.is_empty() {
            return None;
        }
        let remaining = &input[text.len()..];
        let todo_result = |tag| Some((remaining, Tag::Todo(tag)));
        return match (start, text.as_str()) {
            ("#", "todo") => todo_result(TodoTag::Todo),
            ("#", "done") => todo_result(TodoTag::Done),
            ("#", "idea") => todo_result(TodoTag::Idea),
            ("#", "must-do") => todo_result(TodoTag::MustDo),
            ("#", "asap") => todo_result(TodoTag::Asap),
            ("#", "review") => todo_result(TodoTag::Review),
            ("#", "remind") => todo_result(TodoTag::Remind),
            ("#", _) => Some((remaining, Tag::Name(text))),
            ("~", "load") => Some((remaining, Tag::Action(ActionTag::Load))),
            ("~", "expand") => Some((remaining, Tag::Action(ActionTag::Expand))),
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
        }
    }
}
