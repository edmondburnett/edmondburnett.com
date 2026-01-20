use chrono::{DateTime, Utc};
use color_eyre::eyre::Context;
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
    pub date: DateTime<Utc>,
    pub updated: Option<DateTime<Utc>>,
    #[serde(default)]
    pub draft: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PageMetadata {
    pub id: Option<String>,
    pub title: String,
    pub updated: DateTime<Utc>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct TutorialMetadata {
    // example of another metadata type
    pub difficulty: String,
    pub duration: u32,
}

#[derive(Debug)]
pub struct Markdown<T> {
    metadata: T,
    html: String,
    raw_content: String,
}

#[allow(dead_code)]
impl<T: DeserializeOwned> Markdown<T> {
    pub fn from_file(dir: &str, id: &str, full: bool) -> Result<Self> {
        let id = id;
        let file = Self::read_file(dir, id).wrap_err(format!("Failed to read {}.md", id))?;
        let parsed = Self::parse_file(&file).wrap_err(format!("Failed to markdown"))?;
        let metadata = Self::extract_metadata(&parsed).wrap_err(format!("Can't extract metadata from {}.md", id))?;
        let mut raw_content = String::new();
        let mut html = String::new();

        if full {
            html = Self::convert_to_html(&parsed);
            raw_content = parsed.content;

            if html.trim().is_empty() {
                tracing::warn!(file = %format!("{}.md", id), "No HTML content found in file");
            }
        }

        Ok(Self {
            metadata,
            html,
            raw_content,
        })
    }

    fn read_file(dir: &str, id: &str) -> Result<String> {
        let path = Self::get_path(dir);
        let file = std::fs::read_to_string(path.join(format!("{}.md", id)))?;
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
            .ok_or_else(|| eyre::eyre!("No front matter/metadata found."))?
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

    pub fn raw_content(&self) -> &str {
        &self.raw_content
    }
}
