use edmondburnett_com::post::Post;

fn get_post_ids() -> Vec<String> {
    std::fs::read_dir("posts")
        .expect("posts directory exists")
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if path.extension()?.to_str()? == "md" {
                path.file_stem()?.to_str().map(String::from)
            } else {
                None
            }
        })
        .collect()
}

#[test]
fn all_posts_are_valid() {
    for id in get_post_ids() {
        Post::new(&id).unwrap_or_else(|e| panic!("Post '{}' failed validation: {}", id, e));
    }
}
