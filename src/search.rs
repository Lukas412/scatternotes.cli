use std::path::PathBuf;

use eyre::{Context, ContextCompat};

use crate::{config::Config, note_files, read_note_tags};

pub fn search_note_tags(
    config: &Config,
    query_tags: Vec<String>,
) -> eyre::Result<Vec<(PathBuf, Vec<String>)>> {
    let files = note_files(&config)
        .wrap_err("could net read notes directory!")
        .with_context(|| config.path().display())?;

    let mut results = Vec::new();
    let mut found_tags = Vec::new();
    for file in files {
        let mut note_tags = match read_note_tags(&file) {
            Ok(tags) => tags,
            Err(err) => {
                println!("ERROR {} \"{}\"", file.display(), err);
                return Err();
            }
        };

        let mut file_tags_match = true;

        for query_tag in query_tags.iter() {
            let tag_position = note_tags
                .iter()
                .position(|note_tag| note_tag.contains(query_tag.as_str()));

            if let Some(index) = tag_position {
                if let Some(tag) = note_tags.swap_remove_back(index) {
                    found_tags.push(tag);
                }
                continue;
            }

            if found_tags
                .iter()
                .any(|found_tag| found_tag.contains(query_tag.as_str()))
            {
                continue;
            }

            file_tags_match = false;
            break;
        }
        for tag in found_tags.drain(0..).rev() {
            note_tags.push_front(tag);
        }

        if !file_tags_match {
            continue;
        }

        results.push((file, found_tags));
    }
    Ok(results)
}
