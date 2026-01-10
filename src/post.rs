// use comrak::{Options, markdown_to_html};
use std::path::PathBuf;

pub fn load_file() {
    let posts_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("posts");
    let post_data = match std::fs::read_to_string(posts_path.join("test-post.md")) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Failed to load file {}", e);
            String::new()
        }
    };
    println!("{}", post_data);
}
