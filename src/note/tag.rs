use std::{fmt::Display, ops::Not};

#[derive(Debug, PartialEq, Eq)]
pub struct RawTag<'a> {
    name: &'a str,
}

impl<'a> RawTag<'a> {
    pub fn parse(input: &'a str) -> Option<(&'a str, Self)> {
        let name = input.strip_prefix('#')?;
        let length = Self::tag_length(name);
        if length == 0 {
            return None;
        }
        let (name, input) = name.split_at(length);
        Some(( input, Self { name } ))
    }

    fn tag_length(name: &str) -> usize {
        name
            .chars()
            .enumerate()
            .find_map(|(index, char)| {
                matches!(char, 'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | 'ä' | 'ö' | 'ü' | '-' | '_')
                    .not()
                    .then_some(index)
            })
            .unwrap_or(name.len())
    }
}

impl<'a> Display for RawTag<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{}", self.name)
    }
}

pub struct TagIter<'a> {
    content: &'a str,
}

impl<'a> TagIter<'a> {
    pub fn new(content: &'a str) -> Self {
        Self { content }
    }
}

impl<'a> Iterator for TagIter<'a> {
    type Item = RawTag<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let Some(tag_start) = self.content.find('#') else {
                self.content = "";
                return None;
            };
            self.content = &self.content[tag_start..];
            let Some((tail, tag)) = RawTag::parse(self.content) else {
                self.content = &self.content[1..];
                continue;
            };
            self.content = tail;
            return Some(tag);
        }
    }
}
