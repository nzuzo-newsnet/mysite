use serde::{Deserialize, Serialize};

/// A reference/resource link for the references tab
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Reference {
    /// The title of the reference
    pub title: String,
    /// The URL of the reference
    pub url: String,
    /// Optional description of what this reference is about
    #[serde(default)]
    pub description: Option<String>,
}

/// Article series navigation for multi-series support
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArticleSeries {
    /// Name of the series (e.g., "data-engineering", "advanced-topics")
    pub name: String,
    /// Previous article in this series
    #[serde(default)]
    pub prev: Option<String>,
    /// Next article in this series
    #[serde(default)]
    pub next: Option<String>,
}

/// TOML metadata extracted from markdown front matter
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArticleTomlMetadata {
    #[serde(default)]
    pub date: Option<String>,
    #[serde(default)]
    pub author: Option<String>,
    #[serde(default)]
    pub summary: Option<String>,
    #[serde(default)]
    pub topics: Vec<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub thumbnail: Option<String>,
    #[serde(default)]
    pub reading_time: Option<String>,
    #[serde(default)]
    pub category: Option<String>,
    /// Primary series from folder structure (auto-detected)
    #[serde(default)]
    pub primary_series: Option<String>,
    /// Series information for grouped articles (can specify multiple)
    #[serde(default)]
    pub series: Vec<String>,
    /// Article series with navigation (supports multiple series)
    #[serde(default)]
    pub article_series: Vec<ArticleSeries>,
    /// Legacy: Previous article in sequence (deprecated, use article_series)
    #[serde(default)]
    pub prev_article: Option<String>,
    /// Legacy: Next article in sequence (deprecated, use article_series)
    #[serde(default)]
    pub next_article: Option<String>,
    /// References and resources for this article
    #[serde(default)]
    pub references: Vec<Reference>,
    /// Bottom nav controls
    #[serde(default = "default_true")]
    pub show_references: bool,
    #[serde(default)]
    pub show_demo: bool,
    #[serde(default)]
    pub show_related: bool,
    #[serde(default)]
    pub show_quiz: bool,
}

fn default_true() -> bool {
    true
}

/// Result of parsing markdown with front matter
#[derive(Clone, Debug, PartialEq)]
pub struct ParsedMarkdown {
    /// The parsed TOML metadata (if present)
    pub metadata: Option<ArticleTomlMetadata>,
    /// The markdown content without the front matter delimiters
    pub content: String,
}

/// Parse TOML metadata from markdown content
/// Extracts content between first ##### and next #####
///
/// # Format
/// ```text
/// #####
/// date = "2025-11-21"
/// author = "John Doe"
/// #####
///
/// # Article content starts here
/// ```
pub fn parse_markdown_with_metadata(content: &str) -> ParsedMarkdown {
    const DELIMITER: &str = "#####";

    // Try to find and parse TOML metadata
    let metadata = if let Some(first_delimiter_pos) = content.find(DELIMITER) {
        let after_first = &content[first_delimiter_pos + DELIMITER.len()..];

        if let Some(second_delimiter_pos) = after_first.find(DELIMITER) {
            let toml_content = after_first[..second_delimiter_pos].trim();

            // Parse TOML
            toml::from_str::<ArticleTomlMetadata>(toml_content).ok()
        } else {
            None
        }
    } else {
        None
    };

    // Extract content without metadata delimiters
    let content = extract_content_without_metadata(content);

    ParsedMarkdown { metadata, content }
}

/// Extract content without metadata delimiters
fn extract_content_without_metadata(content: &str) -> String {
    const DELIMITER: &str = "#####";

    // Find positions of both delimiters
    if let Some(first_pos) = content.find(DELIMITER) {
        let after_first = &content[first_pos + DELIMITER.len()..];
        if let Some(second_pos) = after_first.find(DELIMITER) {
            // Return content before first delimiter and after second delimiter
            let before = &content[..first_pos];
            let after = &after_first[second_pos + DELIMITER.len()..];
            return format!("{}{}", before.trim(), after.trim());
        }
    }

    // If no metadata found, return original content
    content.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_basic_metadata() {
        let markdown = r#"#####
date = "2025-11-21"
author = "John Doe"
summary = "A test article"
#####

# Test Article

Some content here.
"#;

        let parsed = parse_markdown_with_metadata(markdown);

        assert!(parsed.metadata.is_some());
        let metadata = parsed.metadata.unwrap();
        assert_eq!(metadata.date, Some("2025-11-21".to_string()));
        assert_eq!(metadata.author, Some("John Doe".to_string()));
        assert_eq!(metadata.summary, Some("A test article".to_string()));
        assert!(parsed.content.contains("# Test Article"));
        assert!(!parsed.content.contains("#####"));
    }

    #[test]
    fn test_parse_with_references() {
        let markdown = r#"#####
date = "2025-11-21"
author = "Jane Doe"

[[references]]
title = "Rust Book"
url = "https://doc.rust-lang.org/book/"
description = "The official Rust programming language book"

[[references]]
title = "Rust API Guidelines"
url = "https://rust-lang.github.io/api-guidelines/"
#####

# Article with References
"#;

        let parsed = parse_markdown_with_metadata(markdown);

        assert!(parsed.metadata.is_some());
        let metadata = parsed.metadata.unwrap();
        assert_eq!(metadata.references.len(), 2);
        assert_eq!(metadata.references[0].title, "Rust Book");
        assert_eq!(metadata.references[0].url, "https://doc.rust-lang.org/book/");
        assert_eq!(metadata.references[0].description, Some("The official Rust programming language book".to_string()));
        assert_eq!(metadata.references[1].title, "Rust API Guidelines");
    }

    #[test]
    fn test_parse_no_metadata() {
        let markdown = r#"# Simple Article

This article has no metadata.
"#;

        let parsed = parse_markdown_with_metadata(markdown);

        assert!(parsed.metadata.is_none());
        assert_eq!(parsed.content, markdown);
    }

    #[test]
    fn test_parse_article_series() {
        let markdown = r#"#####
[[article_series]]
name = "My Series"
prev = "article-1"
next = "article-3"
#####

# Part 2
"#;

        let parsed = parse_markdown_with_metadata(markdown);

        assert!(parsed.metadata.is_some());
        let metadata = parsed.metadata.unwrap();
        assert_eq!(metadata.article_series.len(), 1);
        assert_eq!(metadata.article_series[0].name, "My Series");
        assert_eq!(metadata.article_series[0].prev, Some("article-1".to_string()));
        assert_eq!(metadata.article_series[0].next, Some("article-3".to_string()));
    }
}
