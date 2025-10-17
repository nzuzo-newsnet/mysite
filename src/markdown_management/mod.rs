use dioxus::{fullstack::reqwest, prelude::*};
use std::path::PathBuf;

#[cfg(feature = "server")]
use tokio::fs::{read_dir, DirEntry};

#[server]
pub async fn fetch_article_content(file_name: PathBuf) -> Result<String, ServerFnError> {
    match tokio::fs::read_to_string(file_name).await {
        Ok(s) => Ok(s),
        Err(e) => Err(ServerFnError::ServerError {
            message: format!("Internal error finding article: {e:?}"),
            code: 404,
            details: None,
        }),
    }
}

#[server]
pub async fn list_files(dir: String) -> Result<Vec<PathBuf>, ServerFnError> {
    let mut rd = read_dir(dir)
        .await
        .map_err(|e| ServerFnError::ServerError {
            message: format!("Internal error reading directory: {e:?}"),
            code: 500,
            details: None,
        })?;

    let mut files: Vec<PathBuf> = Vec::new();
    while let Some(entry) = rd
        .next_entry()
        .await
        .map_err(|e| ServerFnError::ServerError {
            message: format!("Internal error reading directory entry: {e:?}"),
            code: 500,
            details: None,
        })?
    {
        let ft = entry
            .file_type()
            .await
            .map_err(|e| ServerFnError::ServerError {
                message: format!("Internal error reading file type: {e:?}"),
                code: 500,
                details: None,
            })?;
        if ft.is_file() {
            files.push(entry.path());
        }
    }

    Ok(files)
}

#[server]
pub async fn get_repo_readme(user: String, repo: String) -> Result<String, ServerFnError> {
    let url = format!("https://raw.githubusercontent.com/{user}/{repo}/main/README.md");
    Ok(reqwest::get(url)
        .await
        .expect("Failed to get repo")
        .text()
        .await
        .expect("Failed to read repo"))
}

use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use reqwest::header::{ACCEPT, USER_AGENT};

#[derive(Debug, serde::Deserialize)]
struct Repo {
    full_name: String,
}

#[derive(Debug, serde::Deserialize)]
struct ReadmeResponse {
    content: Option<String>,
    encoding: Option<String>,
    download_url: Option<String>,
}

#[server]
pub async fn extract_readmes_for_account(
    account_type: String, // "users" or "orgs"
    username: String,
) -> Result<Vec<(String, String)>, ServerFnError> {
    let repos_url = format!("https://api.github.com/{}/{}/repos", account_type, username);
    let client = reqwest::Client::new();

    let repos_resp = client
        .get(&repos_url)
        .header(USER_AGENT, "extract-readmes-for-account")
        .header(ACCEPT, "application/vnd.github.v3+json")
        .send()
        .await
        .expect("Repo fetch fail");

    if !repos_resp.status().is_success() {
        return Err(ServerFnError::Request(
            dioxus_fullstack::RequestError::Connect("Could not get repo".to_string()),
        ));
    }

    let repos: Vec<Repo> = repos_resp.json().await.expect("Cold not find repo");
    let mut results = Vec::new();

    for repo in repos {
        let readme_endpoint = format!("https://api.github.com/repos/{}/readme", repo.full_name);
        let resp = match client
            .get(&readme_endpoint)
            .header(USER_AGENT, "extract-readmes-for-account")
            .header(ACCEPT, "application/vnd.github.v3+json")
            .send()
            .await
        {
            Ok(r) => r,
            Err(_) => continue,
        };

        if resp.status().as_u16() == 404 || !resp.status().is_success() {
            continue;
        }

        let rr: ReadmeResponse = match resp.json().await {
            Ok(j) => j,
            Err(_) => continue,
        };

        if let (Some(enc), Some(content)) = (rr.encoding.as_deref(), rr.content.as_deref()) {
            if enc.eq_ignore_ascii_case("base64") {
                match STANDARD.decode(content.replace('\n', "")) {
                    Ok(bytes) => {
                        let text = String::from_utf8_lossy(&bytes).to_string();
                        results.push((repo.full_name.clone(), text));
                        continue;
                    }
                    Err(_) => continue,
                }
            } else {
                continue;
            }
        }

        if let Some(url) = rr.download_url {
            if let Ok(r2) = client.get(&url).send().await {
                if r2.status().is_success() {
                    if let Ok(txt) = r2.text().await {
                        results.push((repo.full_name.clone(), txt));
                    }
                }
            }
        }
    }

    Ok(results)
}
