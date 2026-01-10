use comrak::{Options, markdown_to_html};
use std::path::PathBuf;

pub fn convert_markdown() {
    let post_data = load_file("test-post");
    let mut options = Options::default();
    options.extension.front_matter_delimiter = Some("---".to_string());
    let html = markdown_to_html(&post_data, &options);
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
