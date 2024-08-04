use std::fmt::Write;
use std::fs::read_to_string;
use std::mem;
use std::path::{Path, PathBuf};

pub use self::tag::Tag;
use self::todo::Todo;

mod tag;
mod todo;

#[derive(Clone)]
pub struct Note {
    path: PathBuf,
    tags: Vec<Tag>,
    content: String,
}

impl Note {
    pub fn load(path: PathBuf) -> eyre::Result<Self> {
        let content = read_to_string(&path)?;
        let tags = Tag::parse_all(&content);
        Ok(Self {
            path,
            tags,
            content,
        })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn tags(&self) -> &[Tag] {
        &self.tags
    }

    pub fn join_tags(&self, separator: &str) -> eyre::Result<String> {
        let mut b = String::new();
        let mut iter = self.tags.iter();
        if let Some(first) = iter.next() {
            write!(b, "{}", first)?;
        }
        for tag in iter {
            write!(b, "{}{}", separator, tag)?;
        }
        Ok(b)
    }

    pub fn todos(&self) -> impl Iterator<Item = Todo> {
        self.parts().filter_map(|part| Todo::parse(part))
    }
}

impl Note {
    fn parts(&self) -> impl Iterator<Item = &str> {
        self.content
            .split("\n\n")
            .map(|part| part.trim())
            .filter(|part| !part.is_empty())
    }
}

pub struct MutNote;

impl MutNote {
    pub fn move_tags_to_front(note: &mut Note, names: &[&String]) {
        let mut tags = mem::take(&mut note.tags);
        tags.sort_by_key(|tag| {
            let tag_text = tag.text();
            !names.iter().any(|name| tag_text.contains(*name))
        });
        note.tags = tags;
    }
}
