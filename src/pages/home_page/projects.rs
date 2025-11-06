use crate::markdown_management::github::{fetch_github_repos, GitHubAccountType, GitHubRepo};
use dioxus::prelude::*;

#[component]
pub fn Projects() -> Element {
    // Fetch GitHub repos for your account
    // Change "your-username" to your actual GitHub username
    let mut repos = use_resource(|| async move {
        dioxus::logger::tracing::info!("Starting to fetch GitHub repos");
        let result = fetch_github_repos(
            GitHubAccountType::User,
            "newsnet-africa".to_string(), // Replace with your GitHub username
        )
        .await
        .unwrap_or_default();
        dioxus::logger::tracing::info!("Fetched GitHub repos");
        result
    });

    rsx! {
        article {
            class: "card card-md max-width-[30%] h-full flex flex-col",
            header {
                class: "card-header bg-base-100 sticky top-0 z-10",
                h2 {
                    class: "text-lg sm:text-xl md:text-2xl card-title",
                    "Here's what I'm working on"
                }
            }
            section {
                class: "card-body overflow-y-auto flex-1",
                match repos.value().as_ref() {
                    Some(repo_list) => rsx! {
                        if repo_list.is_empty() {
                            div {
                                class: "text-center p-4 text-base-content opacity-50",
                                "No repositories found"
                            }
                        } else {
                            for repo in repo_list.iter() {
                                ProjectItem {
                                    repo: repo.clone()
                                }
                            }
                        }
                    },
                    None => rsx! {
                        ProjectsSkeleton {}
                    }
                }
            }
            footer {
                class: "card-footer bg-base-100 sticky bottom-0",
                button {
                    class: "btn btn-primary w-full",
                    onclick: move |_| repos.restart(),
                    "Refresh"
                }
            }
        }
    }
}

#[component]
fn ProjectItem(repo: GitHubRepo) -> Element {
    rsx! {
        article {
            class: "card card-sm bg-base-200 mb-4",
            div {
                class: "card-body",
                h3 {
                    class: "card-title text-lg",
                    a {
                        href: "{repo.html_url}",
                        target: "_blank",
                        class: "link link-primary",
                        "{repo.name}"
                    }
                }
                if let Some(description) = &repo.description {
                    p {
                        class: "text-sm text-base-content opacity-70 mb-2",
                        "{description}"
                    }
                }
                div {
                    class: "flex flex-wrap gap-2 text-xs",
                    if let Some(language) = &repo.language {
                        span {
                            class: "badge badge-primary badge-sm",
                            "{language}"
                        }
                    }
                    span {
                        class: "badge badge-ghost badge-sm",
                        "⭐ {repo.stargazers_count}"
                    }
                    span {
                        class: "badge badge-ghost badge-sm",
                        "🔀 {repo.forks_count}"
                    }
                }
                if !repo.topics.is_empty() {
                    div {
                        class: "flex flex-wrap gap-1 mt-2",
                        for topic in &repo.topics {
                            span {
                                class: "badge badge-outline badge-xs",
                                "{topic}"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ProjectsSkeleton() -> Element {
    rsx! {
        div {
            class: "space-y-4",
            // Render 3 skeleton project cards
            for _ in 0..3 {
                div {
                    class: "animate-pulse card card-sm bg-base-200 mb-4",
                    div {
                        class: "card-body",
                        // Title skeleton
                        div {
                            class: "h-6 bg-gray-300 rounded w-2/3 mb-2",
                        }
                        // Description skeleton
                        div {
                            class: "space-y-2",
                            div { class: "h-3 bg-gray-300 rounded" }
                            div { class: "h-3 bg-gray-300 rounded w-4/5" }
                        }
                        // Badges skeleton
                        div {
                            class: "flex gap-2 mt-3",
                            div { class: "h-5 bg-gray-300 rounded w-16" }
                            div { class: "h-5 bg-gray-300 rounded w-12" }
                            div { class: "h-5 bg-gray-300 rounded w-12" }
                        }
                    }
                }
            }
        }
    }
}
