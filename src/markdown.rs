use chrono::{DateTime, Utc};
use color_eyre::eyre::Context;
use color_eyre::{Result, eyre};
use comrak::nodes::NodeValue;
use comrak::{Arena, Options, format_html, markdown_to_html, parse_document};
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
    pub updated: DateTime<Utc>,
    #[serde(default)]
    pub draft: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PageMetadata {
    pub id: Option<String>,
    pub title: String,
    pub updated: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(dead_code)]
pub struct ProjectMetadata {
    pub id: String,
    pub name: String,
    pub description: String,
    pub tags: Vec<String>,
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
            html = Self::convert_to_html(&parsed)?;
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

    fn convert_to_html_simple(parsed: &ParsedEntity) -> String {
        markdown_to_html(&parsed.content, &Options::default())
    }

    fn convert_to_html(parsed: &ParsedEntity) -> Result<String> {
        let arena = Arena::new();
        let root = parse_document(&arena, &parsed.content, &Options::default());

        let mut html = String::new();
        format_html(root, &Options::default(), &mut html).wrap_err("Failed to format HTML")?;

        let html = Self::add_code_labels(&html)?;

        Ok(html)
    }

    fn add_code_labels(html: &str) -> Result<String> {
        let re = regex::Regex::new(r#"<pre><code class="language-(\w+)"#).wrap_err("Failed to compile regex")?;

        Ok(re
            .replace_all(html, |caps: &regex::Captures| {
                let lang = &caps[1];
                format!(
                    r#"<pre><div class="code-label">{}</div><code class="language-{}"#,
                    lang, lang
                )
            })
            .to_string())
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
