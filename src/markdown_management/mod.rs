pub mod local;
pub mod github;

#[cfg(feature = "web")]
pub mod github_cache;

#[cfg(feature = "server")]
pub mod watcher;

pub use local::{
    list_files,
    fetch_article_content,
    fetch_article_with_metadata,
    fetch_home_page_data,
    fetch_home_page_data_with_metadata,
    fetch_all_series,
    fetch_series_by_name,
    fetch_standalone_articles,
    fetch_about_me,
    HomePageData,
    HomePageDataWithMetadata,
    ArticleMetadata,
    ArticleWithMetadata,
    SeriesInfo,
    SeriesData,
    PaginatedArticles,
};

// Re-export types from advanced_markdown_parser
pub use advanced_markdown_parser::{ArticleTomlMetadata, ArticleSeries, Reference};
pub use github::{fetch_github_repos, GitHubRepo};

#[cfg(feature = "server")]
pub use watcher::start_article_watcher;
