use comrak::{Options, markdown_to_html};
use gray_matter::engine::YAML;
use gray_matter::{Matter, ParsedEntity};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
struct FrontMatter {
    id: Option<String>, // TODO: actually comes from filename?
    title: String,
    description: Option<String>,
    tags: Vec<String>,
}

pub fn convert_markdown() {
    let post_data = load_file("test-post");

    let matter = Matter::<YAML>::new();
    let parsed: ParsedEntity = match matter.parse(&post_data) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Failed to parse {}", e);
            return;
        }
    };

    let options = Options::default();

    let metadata: FrontMatter = match parsed.data {
        Some(pod) => match pod.deserialize::<FrontMatter>() {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Failed to deserialize front matter: {}", e);
                return;
            }
        },
        None => {
            eprintln!("No front matter found in file.");
            return;
        }
    };
    println!(
        "Metadata: {:?}, {:?}, {:?}",
        metadata.title, metadata.description, metadata.tags
    );

    let html = markdown_to_html(&parsed.content, &options);
    println!("{}", html);
}

fn load_file(id: &str) -> String {
    let posts_path = get_posts_path();
    let post_data = match std::fs::read_to_string(posts_path.join(format!("{}.md", id))) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Failed to load file {}", e);
            String::new()
        }
    };
    post_data
}

fn get_posts_path() -> PathBuf {
    // TODO: might be different in production
    PathBuf::from("posts")
}
