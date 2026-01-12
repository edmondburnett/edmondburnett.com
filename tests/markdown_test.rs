use edmondburnett_com::markdown::Markdown;

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use std::fs;

    #[derive(Deserialize, Debug, PartialEq)]
    struct TestMetadata {
        title: String,
        tags: Vec<String>,
    }

    // Helper to create a temporary markdown file
    fn create_test_file(dir: &str, id: &str, content: &str) {
        fs::create_dir_all(dir).unwrap();
        fs::write(format!("{}/{}.md", dir, id), content).unwrap();
    }

    // Helper to cleanup test files
    fn cleanup_test_dir(dir: &str) {
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn test_valid_markdown_with_metadata() {
        let dir = "test_posts";
        let id = "test_valid";
        let content = r#"---
title: Test Post
tags: [rust, testing]
---

# Hello World

This is a test post."#;

        create_test_file(dir, id, content);

        let result = Markdown::<TestMetadata>::from_file(dir, id, true);
        assert!(result.is_ok());

        let markdown = result.unwrap();
        assert_eq!(markdown.metadata().title, "Test Post");
        assert_eq!(markdown.metadata().tags, vec!["rust", "testing"]);
        assert!(markdown.html().contains("<h1>Hello World</h1>"));
        assert!(markdown.html().contains("<p>This is a test post.</p>"));

        cleanup_test_dir(dir);
    }

    #[test]
    fn test_markdown_without_metadata() {
        let dir = "test_posts";
        let id = "test_no_metadata";
        let content = "# Just Markdown\n\nNo frontmatter here.";

        create_test_file(dir, id, content);

        let result = Markdown::<TestMetadata>::from_file(dir, id, true);

        match result {
            Ok(_) => panic!("Expected error but got Ok"),
            Err(e) => {
                eprintln!("Error message: '{}'", e);
                eprintln!(
                    "Contains check: {}",
                    e.to_string().contains("No front matter/metadata found")
                );
            }
        }

        cleanup_test_dir(dir);
    }

    #[test]
    fn test_markdown_with_empty_content() {
        let dir = "test_posts";
        let id = "test_empty";
        let content = r#"---
title: Empty Post
tags: []
---"#;

        create_test_file(dir, id, content);

        let result = Markdown::<TestMetadata>::from_file(dir, id, true);
        assert!(result.is_ok());

        let markdown = result.unwrap();
        assert_eq!(markdown.metadata().title, "Empty Post");
        assert!(markdown.html().trim().is_empty());

        cleanup_test_dir(dir);
    }

    #[test]
    fn test_markdown_with_invalid_metadata() {
        let dir = "test_posts";
        let id = "test_invalid";
        let content = r#"---
title: Test
wrongfield: value
---

Content here."#;

        create_test_file(dir, id, content);

        let result = Markdown::<TestMetadata>::from_file(dir, id, true);
        // Should fail because 'tags' is required but missing
        assert!(result.is_err());

        cleanup_test_dir(dir);
    }

    #[test]
    fn test_file_not_found() {
        let result = Markdown::<TestMetadata>::from_file("nonexistent", "missing", true);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Failed to read"));
    }

    #[test]
    fn test_html_conversion() {
        let dir = "test_posts";
        let id = "test_html";
        let content = r#"---
title: HTML Test
tags: [markdown]
---

# Heading

**Bold** and *italic*

- List item 1
- List item 2"#;

        create_test_file(dir, id, content);

        let markdown = Markdown::<TestMetadata>::from_file(dir, id, true).unwrap();
        let html = markdown.html();

        assert!(html.contains("<h1>Heading</h1>"));
        assert!(html.contains("<strong>Bold</strong>"));
        assert!(html.contains("<em>italic</em>"));
        assert!(html.contains("<li>List item 1</li>"));

        cleanup_test_dir(dir);
    }

    #[test]
    fn test_raw_content_preserved() {
        let dir = "test_posts";
        let id = "test_raw";
        let content = r#"---
title: Raw Test
tags: []
---

Raw content here."#;

        create_test_file(dir, id, content);

        let markdown = Markdown::<TestMetadata>::from_file(dir, id, true).unwrap();
        assert_eq!(markdown.raw_content(), "Raw content here.");

        cleanup_test_dir(dir);
    }
}
