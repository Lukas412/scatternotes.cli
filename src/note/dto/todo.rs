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
}
