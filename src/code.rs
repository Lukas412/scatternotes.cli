use std::fs;
use std::path::PathBuf;
use std::process::Command;

use eyre::eyre;
use nom::Err;

use crate::config::Config;
use crate::note::Note;

const MD_CODE: &str = "```";

pub struct Code<'a> {
    note: &'a Note,
    content: &'a str,
    language: &'a str,
}

impl<'a> Code<'a> {
    pub fn search(note: &'a Note) -> Option<Self> {
        search_next(note, note.content()).map(|(_, code)| code)
    }

    pub fn search_language(note: &'a Note, language: &str) -> Option<Self> {
        let mut input = note.content();
        let language = unify_language(language);
        loop {
            let (remaining, code) = search_next(note, input)?;
            if code.language == language {
                return Some(code);
            }
            input = remaining;
        }
    }

    pub fn save(&self, config: &Config) -> eyre::Result<()> {
        let path = self.path(config);
        fs::write(path, self.content);
        Ok(())
    }

    pub fn run(&self, config: &Config) -> eyre::Result<String> {
        let path = self.path(config);
        let output = match self.language {
            "python" => Command::new("python3").args([path]).output()?,
            language => return Err(eyre!("no language executor defined for {}", language)),
        };
        Ok(String::from_utf8(output.stdout)?)
    }

    pub fn path(&self, config: &Config) -> PathBuf {
        let name = self.note.name();
        let extension = match self.language {
            "javascript" => "js",
            "python" => "py",
            "rust" => "rs",
            "plantuml" => "puml",
            "csharp" => "cs",
            extension => extension,
        };
        config.code(name).with_extension(extension)
    }
}

impl<'a> Code<'a> {
    fn new(note: &'a Note, content: &'a str, language: &'a str) -> Self {
        let language = unify_language(language);
        Self {
            note,
            content,
            language,
        }
    }
}

pub fn search_next<'a>(note: &'a Note, input: &'a str) -> Option<(&'a str, Code<'a>)> {
    let (_, input) = input.split_once(MD_CODE)?;
    let (content, input) = input.split_once(MD_CODE)?;
    let content = content.trim();
    let input = input.trim_start();
    let (language, content) = content.split_once(char::is_whitespace)?;
    Some((input, Code::new(note, content, language)))
}

fn unify_language(name: &str) -> &str {
    let name = name.trim();
    match name {
        "py" => "python",
        "c#" => "csharp",
        "js" => "javascript",
        "rs" => "rust",
        "puml" => "plantuml",
        _ => name,
    }
}
