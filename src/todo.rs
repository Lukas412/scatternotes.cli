use std::collections::HashSet;

use crate::note::Note;
use crate::tag::Tag;

use super::tag::TodoTag;

pub struct Todo<'a> {
    content: &'a str,
    tags: HashSet<Tag<'a>>,
}

impl<'a> Todo<'a> {
    pub fn all(note: &'a Note) -> impl Iterator<Item = Self> {
        note.parts().filter_map(|part| Todo::parse_str(part))
    }

    pub fn parse_str(content: &'a str) -> Option<Self> {
        let tags = Tag::all(content);
        tags.iter()
            .any(|tag| matches!(tag, Tag::Todo(_)))
            .then(|| Self { content, tags })
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn is_done(&self) -> bool {
        self.tags
            .iter()
            .any(|tag| matches!(tag, Tag::Todo(TodoTag::Done)))
    }
}
