use anyhow::Result;
use serde::{Deserialize, Serialize};

// Re-export types from github_cache module (web only)
#[cfg(feature = "web")]
pub use super::github_cache::{CachedGitHubData, GitHubCacheDefinition};

/// GitHub repository metadata
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "web", derive(bincode::Encode, bincode::Decode))]
pub struct GitHubRepo {
    pub name: String,
    pub description: Option<String>,
    pub html_url: String,
    pub homepage: Option<String>,
    pub language: Option<String>,
    pub stargazers_count: u32,
    pub forks_count: u32,
    pub open_issues_count: u32,
    pub created_at: String,
    pub updated_at: String,
    pub topics: Vec<String>,
}

/// GitHub user/org type
#[derive(Clone, Debug)]
pub enum GitHubAccountType {
    User,
    Organization,
}

/// Internal function to fetch repos without caching
async fn fetch_github_repos_uncached(
    account_type: &GitHubAccountType,
    account_name: &str,
) -> Result<Vec<GitHubRepo>> {
    let endpoint = match account_type {
        GitHubAccountType::User => format!("https://api.github.com/users/{}/repos", account_name),
        GitHubAccountType::Organization => {
            format!("https://api.github.com/orgs/{}/repos", account_name)
        }
    };

    // Add per_page parameter to get more repos
    let url = format!("{}?per_page=100&sort=updated", endpoint);

    let response = gloo_net::http::Request::get(&url)
        .header("Accept", "application/vnd.github.v3+json")
        .header("User-Agent", "Dioxus-Blog-App")
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to fetch GitHub repos: {}", e))?;

    if !response.ok() {
        return Err(anyhow::anyhow!(
            "GitHub API error: {} - {}",
            response.status(),
            response.status_text()
        ));
    }

    let repos: Vec<GitHubRepo> = response
        .json()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to parse GitHub response: {}", e))?;

    Ok(repos)
}

// ============================================================================
// WASM/Client-side implementation with IndexedDB
// ============================================================================

#[cfg(all(feature = "web", target_arch = "wasm32"))]
/// Fetch all repositories for a GitHub account with IndexedDB caching (client-side)
/// Uses stale-while-revalidate pattern: returns cached data if available, fetches fresh data in background
pub async fn fetch_github_repos(
    account_type: GitHubAccountType,
    account_name: String,
) -> Result<Vec<GitHubRepo>> {
    use netabase_store::databases::indexeddb_store::IndexedDBStore;
    use wasm_bindgen::prelude::*;

    // Cache key based on account type and name
    let cache_key = format!("github_repos_{}_{}",
        match account_type {
            GitHubAccountType::User => "user",
            GitHubAccountType::Organization => "org",
        },
        account_name
    );

    // Cache TTL: 30 minutes (in milliseconds)
    const CACHE_TTL_MS: f64 = 30.0 * 60.0 * 1000.0;

    // Get current timestamp
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = Date, js_name = now)]
        fn date_now() -> f64;
    }
    let now = date_now();

    // Open IndexedDB store
    let store = IndexedDBStore::<GitHubCacheDefinition>::new("blogger_github_cache")
        .await
        .map_err(|e| anyhow::anyhow!("Failed to open IndexedDB: {}", e))?;

    let tree = store.open_tree::<CachedGitHubData>();

    // Try to get cached data from IndexedDB
    use super::github_cache::CachedGitHubDataPrimaryKey;
    let cached_data = tree.get(CachedGitHubDataPrimaryKey(cache_key.clone()))
        .await
        .map_err(|e| anyhow::anyhow!("Failed to read from IndexedDB: {}", e))?;

    if let Some(cached) = cached_data {
        let age = now - cached.cached_at;

        // If cache is still fresh (< 30 minutes), return immediately
        if age < CACHE_TTL_MS {
            dioxus::logger::tracing::info!("Returning fresh cached GitHub repos (age: {:.1}s)", age / 1000.0);
            return Ok(cached.repos);
        }

        // Cache is stale but exists - return stale data immediately
        // and spawn background refresh (fire-and-forget)
        dioxus::logger::tracing::info!("Returning stale cached GitHub repos (age: {:.1}s), refreshing in background", age / 1000.0);

        let cache_key_clone = cache_key.clone();
        let account_type_clone = account_type.clone();
        let account_name_clone = account_name.clone();

        wasm_bindgen_futures::spawn_local(async move {
            if let Ok(fresh_repos) = fetch_github_repos_uncached(&account_type_clone, &account_name_clone).await {
                // Open store again in background task
                if let Ok(store) = IndexedDBStore::<GitHubCacheDefinition>::new("blogger_github_cache").await {
                    let tree = store.open_tree::<CachedGitHubData>();
                    let new_cache = CachedGitHubData {
                        cache_key: cache_key_clone.clone(),
                        repos: fresh_repos,
                        cached_at: date_now(),
                    };
                    let _ = tree.put(new_cache).await;
                    dioxus::logger::tracing::info!("Updated stale GitHub cache in background");
                }
            }
        });

        // Return stale data immediately (stale-while-revalidate)
        return Ok(cached.repos);
    }

    // No cache exists - fetch fresh data
    dioxus::logger::tracing::info!("No cached GitHub repos found, fetching fresh data");
    let repos = fetch_github_repos_uncached(&account_type, &account_name).await?;

    // Cache the fresh data in IndexedDB
    let cache_data = CachedGitHubData {
        cache_key: cache_key.clone(),
        repos: repos.clone(),
        cached_at: now,
    };
    tree.put(cache_data).await
        .map_err(|e| anyhow::anyhow!("Failed to write to IndexedDB: {}", e))?;

    dioxus::logger::tracing::info!("Cached fresh GitHub repos to IndexedDB");

    Ok(repos)
}

// ============================================================================
// Server-side implementation with cached crate
// ============================================================================

#[cfg(all(not(target_arch = "wasm32"), feature = "server"))]
/// Fetch all repositories for a GitHub account with server-side caching
/// Uses the `cached` crate with a 30-minute TTL
pub async fn fetch_github_repos(
    account_type: GitHubAccountType,
    account_name: String,
) -> Result<Vec<GitHubRepo>> {
    use cached::proc_macro::cached;

    // Inner cached function
    #[cached(
        time = 1800,  // 30 minutes in seconds
        result = true,
        sync_writes = true,
        key = "String",
        convert = r#"{ format!("{}_{}",
            match account_type {
                GitHubAccountType::User => "user",
                GitHubAccountType::Organization => "org",
            },
            account_name
        ) }"#
    )]
    async fn fetch_cached(
        account_type: GitHubAccountType,
        account_name: String,
    ) -> Result<Vec<GitHubRepo>> {
        dioxus::logger::tracing::info!("Fetching GitHub repos from API (server-side cache miss)");
        fetch_github_repos_uncached(&account_type, &account_name).await
    }

    fetch_cached(account_type, account_name).await
}

// ============================================================================
// Fallback implementation (native without server feature)
// ============================================================================

#[cfg(all(not(target_arch = "wasm32"), not(feature = "server")))]
/// Fetch all repositories for a GitHub account without caching (fallback)
pub async fn fetch_github_repos(
    account_type: GitHubAccountType,
    account_name: String,
) -> Result<Vec<GitHubRepo>> {
    dioxus::logger::tracing::info!("Fetching GitHub repos (no caching - fallback implementation)");
    fetch_github_repos_uncached(&account_type, &account_name).await
}

/// Fetch the README content for a specific repository
pub async fn fetch_repo_readme(owner: String, repo: String) -> Result<String> {
    let url = format!(
        "https://api.github.com/repos/{}/{}/readme",
        owner, repo
    );

    let response = gloo_net::http::Request::get(&url)
        .header("Accept", "application/vnd.github.v3.raw")
        .header("User-Agent", "Dioxus-Blog-App")
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to fetch README: {}", e))?;

    if !response.ok() {
        return Err(anyhow::anyhow!(
            "GitHub API error: {} - {}",
            response.status(),
            response.status_text()
        ));
    }

    let content = response
        .text()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to read README content: {}", e))?;

    Ok(content)
}
