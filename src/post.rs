use crate::markdown::{FrontMatter, get_html, get_metadata, parse_file, read_file};
use color_eyre::Result;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Post {
    metadata: FrontMatter,
    html: String,
}

#[allow(dead_code)]
impl Post {
    pub fn new(id: &str) -> Result<Self> {
        let file = read_file("posts", id)?;
        let parsed = parse_file(&file)?;
        let metadata = get_metadata(&parsed)?;
        let html = get_html(&parsed);

        Ok(Self { metadata, html })
    }
}
