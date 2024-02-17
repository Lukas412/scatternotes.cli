use std::{
    fmt::write,
    fs::{read_to_string, OpenOptions},
    io::{self, read_to_string, BufWriter, Write},
    ops::Not,
    path::{Path, PathBuf},
    rc::Rc,
};

use rand::{distributions::Alphanumeric, thread_rng, Rng};

pub struct Note {
    key: String,
    content: String,
    tags: Vec<Tag>,
}

impl Note {
    pub fn load_from(path: &Path) -> io::Result<Self> {
        let key = Self::get_key_from_path_or_new_key(path);
        let content = read_to_string(path)?;
        let mut tags = Vec::new();
        Tag::parse_all_into(&mut tags, &content);
        Ok(Self { key, content, tags })
    }

    pub fn save(&self) -> io::Result<()> {
        let path = PathBuf::from(format!("{}.md", self.key));
        let file = OpenOptions::new().truncate(true).write(true).open(path)?;
        let mut buffer = BufWriter::new(file);
        buffer.write(self.content.as_bytes());
        Ok(())
    }

    fn get_key_from_path_or_new_key(path: &Path) -> String {
        path.to_str()
            .filter(|key| key.len() == 23)
            .filter(|key| key.ends_with(".md"))
            .filter(|key| {
                let valid_key_chars = key.chars()
                    .take_while(|char| matches!(char, 'A' ..= 'Z' | '0' ..= '9'))
                    .count();
                valid_key_chars == 20
            })
            .map(|key| key[..20].to_owned())
            .unwrap_or_else(|| Self::generate_new_key())
    }

    fn generate_new_key() -> String {
        thread_rng()
            .sample_iter(&Alphanumeric)
            .map(|char| char.to_ascii_uppercase() as char)
            .take(20)
            .collect()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Tag {
    name: Rc<str>,
}

impl Tag {
    pub fn parse_all_into(buffer: &mut Vec<Self>, input: &str) {
        let mut input = input;
        loop {
            let Some(tag_start) = input.find('#') else {
                return;
            };
            input = &input[tag_start..];
            let Some((tail, tag)) = Self::parse(input) else {
                input = &input[1..];
                continue;
            };
            buffer.push(tag);
            input = tail;
        }
    }

    pub fn parse(input: &str) -> Option<(&str, Self)> {
        let name = input.strip_prefix('#')?;
        let length = Self::tag_length(name);
        if length != 0 {
            return None;
        }
        let (name, input) = name.split_at(length);
        Some((
            input,
            Self {
                name: name.to_owned().into(),
            },
        ))
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
