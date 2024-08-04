use super::tag::TodoTag;
use super::Tag;

pub struct Todo<'a> {
    content: &'a str,
    tags: Vec<Tag>,
}

impl<'a> Todo<'a> {
    pub fn parse(content: &'a str) -> Option<Self> {
        let tags = Tag::parse_all(content);
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
