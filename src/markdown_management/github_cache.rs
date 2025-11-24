use netabase_store::netabase_definition_module;

#[netabase_definition_module(GitHubCacheDefinition, GitHubCacheKeys)]
pub mod github_cache_schema {
    use netabase_store::{netabase, NetabaseModel};

    /// Cached GitHub data structure stored in IndexedDB
    /// Uses the parent GitHubRepo type with bincode encoding
    #[derive(NetabaseModel, Clone, Debug, bincode::Encode, bincode::Decode, serde::Serialize, serde::Deserialize)]
    #[netabase(GitHubCacheDefinition)]
    pub struct CachedGitHubData {
        #[primary_key]
        pub cache_key: String,
        pub repos: Vec<super::super::github::GitHubRepo>,
        pub cached_at: f64, // JavaScript timestamp
    }
}

// Re-export the generated types
pub use github_cache_schema::*;
