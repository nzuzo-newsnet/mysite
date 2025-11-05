use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

/// Metadata for an article (basic file info)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArticleMetadata {
    pub name: String,
    pub path: String,
    pub title: String,
}

/// Series/group information for organizing related articles
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SeriesInfo {
    /// Name of the series (e.g., "Machine Learning Basics")
    pub name: String,
    /// Part number in the series
    pub part: u32,
    /// Total parts in the series (optional)
    #[serde(default)]
    pub total_parts: Option<u32>,
    /// Parent series (for nested series, max 2 levels)
    #[serde(default)]
    pub parent: Option<Box<SeriesInfo>>,
}

/// TOML metadata extracted from markdown files
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
    /// Series information for grouped articles
    #[serde(default)]
    pub series: Option<SeriesInfo>,
    /// Previous article in sequence
    #[serde(default)]
    pub prev_article: Option<String>,
    /// Next article in sequence
    #[serde(default)]
    pub next_article: Option<String>,
}

/// Combined article data with content and metadata
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArticleWithMetadata {
    pub metadata: ArticleMetadata,
    pub toml_metadata: Option<ArticleTomlMetadata>,
    pub content: String,
}

/// Parse TOML metadata from markdown content
/// Extracts content between first ##### and next #####
#[cfg(feature = "server")]
fn parse_toml_metadata(content: &str) -> Option<ArticleTomlMetadata> {
    const DELIMITER: &str = "#####";

    // Find the first delimiter
    let first_delimiter_pos = content.find(DELIMITER)?;
    let after_first = &content[first_delimiter_pos + DELIMITER.len()..];

    // Find the second delimiter
    let second_delimiter_pos = after_first.find(DELIMITER)?;
    let toml_content = &after_first[..second_delimiter_pos].trim();

    // Parse TOML
    toml::from_str::<ArticleTomlMetadata>(toml_content).ok()
}

/// Extract content without metadata delimiters
#[cfg(feature = "server")]
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

/// Extract title from markdown file efficiently (reads only first ~500 bytes)
#[cfg(feature = "server")]
async fn extract_title_from_file(path: &std::path::Path) -> Result<String, std::io::Error> {
    use tokio::fs::File;
    use tokio::io::{AsyncBufReadExt, BufReader};

    let file = File::open(path).await?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    // Read first 10 lines to find title (usually in first few lines)
    let mut line_count = 0;
    while let Some(line) = lines.next_line().await? {
        line_count += 1;
        if line.trim().starts_with("# ") {
            return Ok(line.trim_start_matches("# ").trim().to_string());
        }
        // Stop after 10 lines to avoid reading too much
        if line_count >= 10 {
            break;
        }
    }

    Err(std::io::Error::new(std::io::ErrorKind::NotFound, "No title found"))
}

/// List all available article files (server-side)
#[server]
#[cached::proc_macro::cached(time = 300, result = true, sync_writes = true)]
pub async fn list_files() -> Result<Vec<ArticleMetadata>, ServerFnError> {
    use tokio::fs;
    use std::path::PathBuf;
    use futures::future::join_all;

    let articles_dir = "articles";

    let mut entries: tokio::fs::ReadDir = fs::read_dir(articles_dir)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to read articles directory: {}", e)))?;

    // Collect all markdown file paths first
    let mut file_paths = Vec::new();
    while let Some(entry) = entries
        .next_entry()
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to read directory entry: {}", e)))?
    {
        let path: PathBuf = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("md") {
            file_paths.push(path);
        }
    }

    // Process all files in parallel
    let futures = file_paths.into_iter().map(|path| async move {
        let file_name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();

        // Extract title efficiently (only reads first few lines)
        let title = extract_title_from_file(&path)
            .await
            .unwrap_or_else(|_| file_name.clone());

        ArticleMetadata {
            name: file_name.clone(),
            path: path
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or(&file_name)
                .to_string(),
            title,
        }
    });

    let mut articles = join_all(futures).await;

    // Sort by name
    articles.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(articles)
}

/// Fetch article content from the filesystem (server-side)
#[server]
#[cached::proc_macro::cached(time = 3600, result = true, sync_writes = true, key = "String", convert = r#"{ path.clone() }"#)]
pub async fn fetch_article_content(path: String) -> Result<String, ServerFnError> {
    use tokio::fs;

    // Sanitize the path to prevent directory traversal
    let safe_path = path.replace("..", "");
    let file_path = format!("articles/{}", safe_path);

    let content = fs::read_to_string(&file_path)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to read article: {}", e)))?;

    Ok(content)
}

/// Fetch article with full metadata and processed content
#[server]
#[cached::proc_macro::cached(time = 3600, result = true, sync_writes = true, key = "String", convert = r#"{ path.clone() }"#)]
pub async fn fetch_article_with_metadata(path: String) -> Result<ArticleWithMetadata, ServerFnError> {
    use tokio::fs;

    // Sanitize the path to prevent directory traversal
    let safe_path = path.replace("..", "");
    let file_path = format!("articles/{}", safe_path);

    let raw_content = fs::read_to_string(&file_path)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to read article: {}", e)))?;

    // Parse TOML metadata
    let toml_metadata = parse_toml_metadata(&raw_content);

    // Extract content without metadata delimiters
    let content = extract_content_without_metadata(&raw_content);

    // Extract title from content
    let title = content
        .lines()
        .find(|line| line.trim().starts_with("# "))
        .map(|line| line.trim_start_matches("# ").trim().to_string())
        .unwrap_or_else(|| safe_path.clone());

    let name = safe_path
        .trim_end_matches(".md")
        .to_string();

    Ok(ArticleWithMetadata {
        metadata: ArticleMetadata {
            name,
            path: safe_path,
            title,
        },
        toml_metadata,
        content,
    })
}

/// Fetch article content by name (without extension)
#[server]
#[cached::proc_macro::cached(time = 3600, result = true, sync_writes = true, key = "String", convert = r#"{ name.clone() }"#)]
pub async fn fetch_article_by_name(name: String) -> Result<String, ServerFnError> {
    fetch_article_content(format!("{}.md", name)).await
}

/// Combined data for home page (list + first article)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HomePageData {
    pub articles: Vec<ArticleMetadata>,
    pub first_article_content: Option<String>,
}

/// Combined data for home page with metadata
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HomePageDataWithMetadata {
    pub first_article: Option<ArticleWithMetadata>,
    pub recent_articles: Vec<ArticleWithMetadata>,
}

/// Fetch home page data (articles list + first article content) in a single call
#[server]
#[cached::proc_macro::cached(time = 300, result = true, sync_writes = true)]
pub async fn fetch_home_page_data() -> Result<HomePageData, ServerFnError> {
    // Fetch articles list (cached)
    let articles = list_files().await?;

    // Fetch first article content if available (cached)
    let first_article_content = if let Some(first) = articles.first() {
        fetch_article_content(first.path.clone()).await.ok()
    } else {
        None
    };

    Ok(HomePageData {
        articles,
        first_article_content,
    })
}

/// Fetch home page data with metadata for all articles
#[server]
#[cached::proc_macro::cached(time = 300, result = true, sync_writes = true)]
pub async fn fetch_home_page_data_with_metadata() -> Result<HomePageDataWithMetadata, ServerFnError> {
    use futures::future::join_all;

    dioxus::logger::tracing::info!("fetch_home_page_data_with_metadata: Starting");
    let start = std::time::Instant::now();

    // Fetch articles list (cached)
    let list_start = std::time::Instant::now();
    let articles = list_files().await?;
    dioxus::logger::tracing::info!("fetch_home_page_data_with_metadata: Listed {} files in {:?}", articles.len(), list_start.elapsed());

    // Fetch all articles with metadata in parallel
    let fetch_start = std::time::Instant::now();
    let futures = articles.iter().map(|article| {
        let path = article.path.clone();
        async move { fetch_article_with_metadata(path).await }
    });

    let results: Vec<Result<ArticleWithMetadata, ServerFnError>> = join_all(futures).await;
    dioxus::logger::tracing::info!("fetch_home_page_data_with_metadata: Fetched all articles in {:?}", fetch_start.elapsed());

    // Collect successful results
    let mut articles_with_metadata: Vec<ArticleWithMetadata> = results
        .into_iter()
        .filter_map(|r| r.ok())
        .collect();

    // Sort by date (most recent first) if date is available in metadata
    articles_with_metadata.sort_by(|a, b| {
        match (&a.toml_metadata, &b.toml_metadata) {
            (Some(meta_a), Some(meta_b)) => {
                // Sort by date descending (most recent first)
                meta_b.date.cmp(&meta_a.date)
            }
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => a.metadata.name.cmp(&b.metadata.name),
        }
    });

    let first_article = articles_with_metadata.first().cloned();
    let recent_articles = articles_with_metadata;

    dioxus::logger::tracing::info!("fetch_home_page_data_with_metadata: Completed in {:?}", start.elapsed());

    Ok(HomePageDataWithMetadata {
        first_article,
        recent_articles,
    })
}
