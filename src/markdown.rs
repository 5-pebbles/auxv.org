use std::{
    cmp::min,
    fs::{read_dir, read_to_string},
    path::Path,
};

use crate::error::StartError;

const SHORT_LEN: usize = 50;

#[derive(Clone)]
pub struct MarkdownPage {
    pub title: &'static str,
    pub short: &'static str,
    pub text: &'static str,
    pub path: &'static str,
}

pub fn load_markdown_recursive(
    mut pages: Vec<MarkdownPage>,
    directory: &Path,
) -> Result<Vec<MarkdownPage>, StartError> {
    for entry in read_dir(directory)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            pages = load_markdown_recursive(pages, &path)?;
            continue;
        }

        let Some(content) = load_markdown_file(&path)? else {
            continue;
        };

        let title = Box::leak(generate_markdown_title(&path)?.into_boxed_str());
        let short = "This part of the site does not work yet...";
        let text = Box::leak(content.into_boxed_str());
        let path = Box::leak(
            path.into_os_string()
                .into_string()
                .unwrap()
                .into_boxed_str(),
        );
        let markdown_page = MarkdownPage {
            title,
            short,
            text,
            path,
        };
        pages.push(markdown_page);
    }
    Ok(pages)
}

fn load_markdown_file(path: &Path) -> Result<Option<String>, StartError> {
    if !path.extension().map_or(false, |ext| ext == "md") {
        return Ok(None);
    }

    Ok(Some(read_to_string(path)?))
}

fn generate_markdown_title(path: &Path) -> Result<String, StartError> {
    let filename = path.file_stem().unwrap().to_str().unwrap();

    let words: Vec<String> = filename
        .split(|c| c == '-' || c == '_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first_char) => {
                    first_char.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase()
                }
                None => String::new(),
            }
        })
        .collect();

    Ok(words.join(" "))
}
