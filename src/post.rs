use crate::markdown::{Markdown, PostMetadata};
use color_eyre::Result;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
    pub html: String,
}

impl Post {
    pub fn new(id: &str) -> Result<Self> {
        let markdown = Markdown::<PostMetadata>::from_file("posts", id)?;

        Ok(Self {
            id: id.to_string(),
            title: markdown.metadata().title.clone(),
            description: markdown.metadata().description.clone(),
            tags: markdown.metadata().tags.clone(),
            html: markdown.html().to_string(),
        })
    }
}
