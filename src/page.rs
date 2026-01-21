use crate::markdown::{Markdown, PageMetadata};
use chrono::{DateTime, Utc};
use chrono_tz::US::Pacific;
use color_eyre::Result;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Page {
    pub id: String,
    pub title: String,
    pub html: String,
    pub updated: DateTime<Utc>,
}

impl Page {
    pub fn new(id: &str) -> Result<Self> {
        let markdown = Markdown::<PageMetadata>::from_file("pages", id, true)?;

        Ok(Self {
            id: id.to_string(),
            title: markdown.metadata().title.clone(),
            html: markdown.html().to_string(),
            updated: markdown.metadata().updated,
        })
    }

    #[allow(dead_code)]
    pub fn updated(&self) -> String {
        self.updated
            .with_timezone(&Pacific)
            // .format("%a %b %d %Y %I:%M:%S %p %Z")
            .format("%B %d, %Y")
            .to_string()
    }
}
