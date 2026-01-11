use color_eyre::Result;

use crate::post;

pub fn validate_posts() -> Result<()> {
    use post::Post;
    tracing::info!("Validating posts...");

    let mut valid = 0;
    let mut invalid = 0;

    for entry in std::fs::read_dir("posts")? {
        let path = entry?.path();

        if path.extension().and_then(|s| s.to_str()) != Some("md") {
            continue;
        }

        if let Some(id) = path.file_stem().and_then(|s| s.to_str()) {
            match Post::new(id) {
                Ok(_) => {
                    tracing::info!(post_id = %id, "✓ Post valid");
                    valid += 1;
                }
                Err(e) => {
                    tracing::error!(post_id = %id, error = %e, "✗ Post validation failed");
                    invalid += 1;
                }
            }
        } else {
            tracing::warn!(path = ?path, "✗ Invalid filename");
            invalid += 1;
        }
    }

    tracing::info!(valid, invalid, "Validation complete");

    Ok(())
}
