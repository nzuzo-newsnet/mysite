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

/// Recursively collect all markdown files from a directory (synchronous)
#[cfg(feature = "server")]
fn collect_markdown_files_sync(dir: &std::path::Path) -> Result<Vec<std::path::PathBuf>, std::io::Error> {
    use std::fs;

    let mut markdown_files = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let metadata = fs::metadata(&path)?;

        if metadata.is_dir() {
            // Recursively scan subdirectories
            let mut sub_files = collect_markdown_files_sync(&path)?;
            markdown_files.append(&mut sub_files);
        } else if path.extension().and_then(|s| s.to_str()) == Some("md") {
            markdown_files.push(path);
        }
    }

    Ok(markdown_files)
}

/// Extract series name from folder path
#[cfg(feature = "server")]
fn extract_series_from_path(path: &std::path::Path, base_dir: &str) -> Option<String> {
    // Get the parent directory relative to articles/
    let parent = path.parent()?;
    let parent_str = parent.to_str()?;

    // Remove the base "articles/" prefix
    if let Some(relative_path) = parent_str.strip_prefix(base_dir) {
        let relative_path = relative_path.trim_start_matches('/');
        if !relative_path.is_empty() {
            // Convert path to series name (e.g., "rust/basics" -> "rust/basics")
            return Some(relative_path.to_string());
        }
    }

    None
}

/// List all available article files (server-side)
#[server]
#[cached::proc_macro::cached(time = 300, result = true, sync_writes = true)]
pub async fn list_files() -> Result<Vec<ArticleMetadata>, ServerFnError> {
    use tokio::fs;
    use std::path::{Path, PathBuf};
    use futures::future::join_all;

    let articles_dir = "articles";
    let base_path = Path::new(articles_dir);

    // Recursively collect all markdown files (using sync version to avoid async recursion complexity)
    let file_paths = collect_markdown_files_sync(base_path)
        .map_err(|e| ServerFnError::new(format!("Failed to read articles directory: {}", e)))?;

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

        // Get relative path from articles directory
        let relative_path = path
            .strip_prefix(articles_dir)
            .unwrap_or(&path)
            .to_str()
            .unwrap_or(&file_name)
            .to_string();

        ArticleMetadata {
            name: file_name.clone(),
            path: relative_path,
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
    use std::path::Path;

    // Sanitize the path to prevent directory traversal
    let safe_path = path.replace("..", "");
    let file_path = format!("articles/{}", safe_path);

    let raw_content = fs::read_to_string(&file_path)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to read article: {}", e)))?;

    // Parse TOML metadata
    let mut toml_metadata = parse_toml_metadata(&raw_content);

    // Extract primary series from folder structure
    let path_buf = Path::new(&file_path);
    let primary_series = extract_series_from_path(path_buf, "articles");

    // Set primary_series in metadata if detected from folder
    if let Some(ref mut metadata) = toml_metadata {
        if primary_series.is_some() {
            metadata.primary_series = primary_series;
        }
    }

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

/// Series summary metadata from TOML frontmatter
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SeriesSummaryMetadata {
    pub short_summary: Option<String>,
}

/// Series data with articles
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SeriesData {
    pub name: String,
    pub articles: Vec<ArticleWithMetadata>,
    pub total_articles: usize,
    pub short_summary: Option<String>,
    pub long_summary: Option<String>,
}

/// Fetch all series with their articles
#[server]
#[cached::proc_macro::cached(time = 300, result = true, sync_writes = true)]
pub async fn fetch_all_series() -> Result<Vec<SeriesData>, ServerFnError> {
    use std::collections::HashMap;
    use futures::future::join_all;

    dioxus::logger::tracing::info!("fetch_all_series: Starting");

    // Fetch articles list (cached)
    let articles = list_files().await?;

    // Fetch all articles with metadata in parallel
    let futures = articles.iter().map(|article| {
        let path = article.path.clone();
        async move { fetch_article_with_metadata(path).await }
    });

    let results: Vec<Result<ArticleWithMetadata, ServerFnError>> = join_all(futures).await;

    // Collect successful results
    let articles_with_metadata: Vec<ArticleWithMetadata> = results
        .into_iter()
        .filter_map(|r| r.ok())
        .collect();

    // Group articles by series
    let mut series_map: HashMap<String, Vec<ArticleWithMetadata>> = HashMap::new();

    for article in articles_with_metadata {
        if let Some(ref metadata) = article.toml_metadata {
            // Add to primary series from folder structure
            if let Some(ref primary_series) = metadata.primary_series {
                series_map
                    .entry(primary_series.clone())
                    .or_insert_with(Vec::new)
                    .push(article.clone());
            }

            // Add to additional series from metadata
            for series_name in &metadata.series {
                series_map
                    .entry(series_name.clone())
                    .or_insert_with(Vec::new)
                    .push(article.clone());
            }
        }
    }

    // Convert to SeriesData with summaries
    let futures = series_map.into_iter().map(|(name, mut articles)| async move {
        // Sort articles by name within each series
        articles.sort_by(|a, b| a.metadata.name.cmp(&b.metadata.name));

        let total_articles = articles.len();

        // Try to read summary.md from the series folder
        let summary_path = format!("articles/{}/summary.md", name);
        let (short_summary, long_summary) = match tokio::fs::read_to_string(&summary_path).await {
            Ok(content) => {
                // Parse TOML frontmatter and markdown content
                if content.starts_with("#####") {
                    let parts: Vec<&str> = content.splitn(3, "#####").collect();
                    if parts.len() >= 3 {
                        let toml_str = parts[1].trim();
                        let markdown_content = parts[2].trim();

                        // Parse TOML metadata
                        let metadata: Result<SeriesSummaryMetadata, _> = toml::from_str(toml_str);
                        let short = metadata.ok().and_then(|m| m.short_summary);

                        (short, Some(markdown_content.to_string()))
                    } else {
                        (None, Some(content))
                    }
                } else {
                    (None, Some(content))
                }
            },
            Err(_) => (None, None),
        };

        SeriesData {
            name,
            articles,
            total_articles,
            short_summary,
            long_summary,
        }
    });

    let mut series_list: Vec<SeriesData> = join_all(futures).await;

    // Sort series by name
    series_list.sort_by(|a, b| a.name.cmp(&b.name));

    dioxus::logger::tracing::info!("fetch_all_series: Found {} series", series_list.len());

    Ok(series_list)
}

/// Fetch a single series by name
#[server]
#[cached::proc_macro::cached(time = 300, result = true, sync_writes = true)]
pub async fn fetch_series_by_name(series_name: String) -> Result<SeriesData, ServerFnError> {
    use futures::future::join_all;

    dioxus::logger::tracing::info!("fetch_series_by_name: Fetching series '{}'", series_name);

    // Fetch articles list (cached)
    let articles = list_files().await?;

    // Fetch all articles with metadata in parallel
    let futures = articles.iter().map(|article| {
        let path = article.path.clone();
        async move { fetch_article_with_metadata(path).await }
    });

    let results: Vec<Result<ArticleWithMetadata, ServerFnError>> = join_all(futures).await;

    // Collect articles that belong to this series
    let mut series_articles: Vec<ArticleWithMetadata> = results
        .into_iter()
        .filter_map(|r| r.ok())
        .filter(|article| {
            if let Some(ref metadata) = article.toml_metadata {
                // Check if article belongs to this series
                metadata.primary_series.as_ref() == Some(&series_name) ||
                metadata.series.contains(&series_name)
            } else {
                false
            }
        })
        .collect();

    if series_articles.is_empty() {
        return Err(ServerFnError::new("Series not found"));
    }

    // Sort articles by name
    series_articles.sort_by(|a, b| a.metadata.name.cmp(&b.metadata.name));

    let total_articles = series_articles.len();

    // Read summary.md from the series folder
    let summary_path = format!("articles/{}/summary.md", series_name);
    let (short_summary, long_summary) = match tokio::fs::read_to_string(&summary_path).await {
        Ok(content) => {
            // Parse TOML frontmatter and markdown content
            if content.starts_with("#####") {
                let parts: Vec<&str> = content.splitn(3, "#####").collect();
                if parts.len() >= 3 {
                    let toml_str = parts[1].trim();
                    let markdown_content = parts[2].trim();

                    // Parse TOML metadata
                    let metadata: Result<SeriesSummaryMetadata, _> = toml::from_str(toml_str);
                    let short = metadata.ok().and_then(|m| m.short_summary);

                    (short, Some(markdown_content.to_string()))
                } else {
                    (None, Some(content))
                }
            } else {
                (None, Some(content))
            }
        },
        Err(_) => (None, None),
    };

    Ok(SeriesData {
        name: series_name,
        articles: series_articles,
        total_articles,
        short_summary,
        long_summary,
    })
}

/// Fetch about me content
#[server]
#[cached::proc_macro::cached(time = 300, result = true, sync_writes = true)]
pub async fn fetch_about_me() -> Result<String, ServerFnError> {
    match tokio::fs::read_to_string("aboutme.md").await {
        Ok(content) => Ok(content),
        Err(_) => Err(ServerFnError::new("About me file not found")),
    }
}
