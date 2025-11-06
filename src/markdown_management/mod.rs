pub mod local;
pub mod github;

#[cfg(feature = "web")]
pub mod github_cache;

pub use local::{
    list_files,
    fetch_article_content,
    fetch_article_with_metadata,
    fetch_home_page_data,
    fetch_home_page_data_with_metadata,
    fetch_all_series,
    HomePageData,
    HomePageDataWithMetadata,
    ArticleMetadata,
    ArticleTomlMetadata,
    ArticleWithMetadata,
    SeriesInfo,
    SeriesData,
};
pub use github::{fetch_github_repos, GitHubRepo};
