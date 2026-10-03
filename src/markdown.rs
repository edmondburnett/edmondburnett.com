use chrono::{DateTime, Utc};
use color_eyre::eyre::Context;
use color_eyre::{Result, eyre};
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
    pub obsolete: Option<bool>,
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
        let mut options = Options::default();
        options.render.figure_with_caption = true;

        let arena = Arena::new();
        let root = parse_document(&arena, &parsed.content, &options);

        let mut html = String::new();
        format_html(root, &options, &mut html).wrap_err("Failed to format HTML")?;

        let html = Self::add_code_labels(&html)?;
        let html = Self::add_image_srcsets(&html)?;
        let html = Self::add_image_dimensions(&html)?;
        let html = Self::add_image_links(&html)?;

        Ok(html)
    }

    /// Adds a `srcset` to local images that have high-density variants on disk,
    /// e.g. `foo.png` gets `foo@2x.png 2x` if `static/.../foo@2x.png` exists.
    fn add_image_srcsets(html: &str) -> Result<String> {
        let re = regex::Regex::new(r#"<img src="(/static/[^"]+)""#).wrap_err("Failed to compile regex")?;

        Ok(re
            .replace_all(html, |caps: &regex::Captures| {
                let src = &caps[1];
                let candidates: Vec<String> = Self::existing_density_variants(src)
                    .into_iter()
                    .map(|(variant, density)| format!("{} {}", variant, density))
                    .collect();

                if candidates.is_empty() {
                    return caps[0].to_string();
                }

                format!(r#"<img src="{}" srcset="{} 1x, {}""#, src, src, candidates.join(", "))
            })
            .to_string())
    }

    /// Adds `width`/`height` to local images, read from the file on disk, so the
    /// browser can reserve space before the image loads and the layout doesn't jump.
    fn add_image_dimensions(html: &str) -> Result<String> {
        let re = regex::Regex::new(r#"<img src="(/static/[^"]+)""#).wrap_err("Failed to compile regex")?;

        Ok(re
            .replace_all(html, |caps: &regex::Captures| {
                let path = PathBuf::from(caps[1].trim_start_matches('/'));
                match imagesize::size(&path) {
                    Ok(dim) => format!(r#"{} width="{}" height="{}""#, &caps[0], dim.width, dim.height),
                    Err(err) => {
                        tracing::warn!(path = %path.display(), error = %err, "Couldn't read image dimensions");
                        caps[0].to_string()
                    }
                }
            })
            .to_string())
    }

    /// Wraps images in a link to the full-size file (the highest-density variant
    /// on disk, if any), so readers can click through. Images that are already
    /// inside a Markdown link are left alone. The `<figure>`/`<figcaption>` stay
    /// outside the link so only the image itself is clickable.
    fn add_image_links(html: &str) -> Result<String> {
        let re = regex::Regex::new(r#"(<a [^>]*>(?:<figure>)?)?<img src="([^"]+)"[^>]*/>"#)
            .wrap_err("Failed to compile regex")?;

        Ok(re
            .replace_all(html, |caps: &regex::Captures| {
                if caps.get(1).is_some() {
                    return caps[0].to_string();
                }

                let src = &caps[2];
                let full_size = Self::existing_density_variants(src)
                    .pop()
                    .map(|(variant, _)| variant)
                    .unwrap_or_else(|| src.to_string());

                format!(r#"<a class="image-link" href="{}">{}</a>"#, full_size, &caps[0])
            })
            .to_string())
    }

    /// Density variants of a local `/static/` image that exist on disk, lowest density first.
    fn existing_density_variants(src: &str) -> Vec<(String, &'static str)> {
        if !src.starts_with("/static/") {
            return Vec::new();
        }

        ["2x", "3x"]
            .into_iter()
            .filter_map(|density| {
                let variant = Self::density_variant(src, density)?;
                let path = PathBuf::from(variant.trim_start_matches('/'));
                path.is_file().then_some((variant, density))
            })
            .collect()
    }

    /// `/static/a/foo.png` + `2x` -> `/static/a/foo@2x.png`
    fn density_variant(src: &str, density: &str) -> Option<String> {
        let (stem, ext) = src.rsplit_once('.')?;
        if ext.contains('/') {
            return None;
        }
        Some(format!("{}@{}.{}", stem, density, ext))
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
