use color_eyre::{Result, eyre};
use comrak::{Options, markdown_to_html};
use gray_matter::engine::YAML;
use gray_matter::{Matter, ParsedEntity};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct PostMetadata {
    pub id: Option<String>,
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct TutorialMetadata {
    // example of another metadata type
    // usage: let markdown = Markdown::<TutorialMetadata>::from_file("tutorials", id)?;
    pub difficulty: String,
    pub duration: u32,
}

#[allow(dead_code)]
pub struct Markdown<T> {
    metadata: T,
    html: String,
    raw_content: String,
}

impl<T: DeserializeOwned> Markdown<T> {
    pub fn from_file(dir: &str, id: &str) -> Result<Self> {
        let file = Self::read_file(dir, id)?;
        let parsed = Self::parse_file(&file)?;
        let metadata = Self::extract_metadata(&parsed)?;
        let html = Self::convert_to_html(&parsed);

        Ok(Self {
            metadata,
            html,
            raw_content: parsed.content,
        })
    }

    fn read_file(dir: &str, id: &str) -> Result<String> {
        let posts_path = Self::get_path(dir);
        let file = std::fs::read_to_string(posts_path.join(format!("{}.md", id)))?;
        Ok(file)
    }

    fn parse_file(file: &String) -> Result<ParsedEntity> {
        let matter = Matter::<YAML>::new();
        matter.parse(file).map_err(Into::into)
    }

    fn extract_metadata(parsed: &ParsedEntity) -> Result<T> {
        parsed
            .data
            .as_ref()
            .ok_or_else(|| eyre::eyre!("No front matter/metadata found in file."))?
            .deserialize()
            .map_err(Into::into)
    }

    fn convert_to_html(parsed: &ParsedEntity) -> String {
        markdown_to_html(&parsed.content, &Options::default())
    }

    fn get_path(dir: &str) -> PathBuf {
        PathBuf::from(dir)
    }

    pub fn metadata(&self) -> &T {
        &self.metadata
    }

    pub fn html(&self) -> &str {
        &self.html
    }
}
