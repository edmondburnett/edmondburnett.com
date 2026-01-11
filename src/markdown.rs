use color_eyre::{Result, eyre};
use comrak::{Options, markdown_to_html};
use gray_matter::engine::YAML;
use gray_matter::{Matter, ParsedEntity};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct FrontMatter {
    id: Option<String>, // TODO: actually comes from filename?
    title: String,
    description: Option<String>,
    tags: Vec<String>,
}

pub fn parse_file(file: &String) -> Result<ParsedEntity> {
    let matter = Matter::<YAML>::new();
    matter.parse(file).map_err(Into::into)
}

pub fn get_metadata(parsed: &ParsedEntity) -> Result<FrontMatter> {
    let metadata: FrontMatter = match &parsed.data {
        Some(pod) => match pod.deserialize::<FrontMatter>() {
            Ok(data) => data,
            Err(e) => return Err(e.into()),
        },
        None => {
            eprintln!("No front matter found in file.");
            return Err(eyre::eyre!("No front matter found in file."));
        }
    };
    Ok(metadata)
}

pub fn get_html(parsed: &ParsedEntity) -> String {
    markdown_to_html(&parsed.content, &Options::default())
}

pub fn read_file(dir: &str, id: &str) -> Result<String> {
    let posts_path = get_path(dir);
    let file = std::fs::read_to_string(posts_path.join(format!("{}.md", id)))?;
    Ok(file)
}

fn get_path(dir: &str) -> PathBuf {
    PathBuf::from(dir)
}
