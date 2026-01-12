use crate::markdown::{Markdown, PostMetadata};
use chrono::{DateTime, Utc};
use chrono_tz::US::Pacific;
use color_eyre::Result;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
    pub html: String,
    pub date: DateTime<Utc>,
}

impl Post {
    pub fn new(id: &str) -> Result<Self> {
        let markdown = Markdown::<PostMetadata>::from_file("posts", id, true)?;

        Ok(Self {
            id: id.to_string(),
            title: markdown.metadata().title.clone(),
            description: markdown.metadata().description.clone(),
            tags: markdown.metadata().tags.clone(),
            html: markdown.html().to_string(),
            date: markdown.metadata().date,
        })
    }

    pub fn list() -> Result<Vec<Post>> {
        let mut posts = Vec::new();

        for entry in std::fs::read_dir("posts")? {
            let path = entry?.path();

            if path.extension().and_then(|s| s.to_str()) != Some("md") {
                continue;
            }

            // if let Some(id) = path.file_stem().and_then(|s| s.to_str()) {
            if let Some(file_stem) = path.file_stem() {
                if let Some(id) = file_stem.to_str() {
                    let markdown = Markdown::<PostMetadata>::from_file("posts", id, false)?;

                    posts.push(Post {
                        id: id.to_string(),
                        title: markdown.metadata().title.clone(),
                        description: markdown.metadata().description.clone(),
                        tags: markdown.metadata().tags.clone(),
                        html: String::new(),
                        date: markdown.metadata().date,
                    });
                }
            }
        }
        posts.sort_by(Self::compare_posts_by_date);
        Ok(posts)
    }

    fn compare_posts_by_date(a: &Post, b: &Post) -> std::cmp::Ordering {
        b.date.cmp(&a.date)
    }

    #[allow(dead_code)]
    pub fn formatted_date(&self) -> String {
        self.date
            .with_timezone(&Pacific)
            .format("%a %b %d %Y %I:%M:%S %p %Z")
            .to_string()
    }
}
