use dioxus::prelude::*;
use dioxus_markdown::Markdown;

use crate::markdown_management::{
    fetch_home_page_data_with_metadata, ArticleTomlMetadata, ArticleWithMetadata,
};

#[component]
pub fn Blogs() -> Element {
    // Fetch articles with metadata
    let home_data = use_resource(|| async move {
        let start = chrono::Local::now();
        dioxus::logger::tracing::info!(
            "Starting to fetch home page data with metadata at: {start}"
        );
        let result = fetch_home_page_data_with_metadata().await;
        let end = chrono::Local::now();
        dioxus::logger::tracing::info!("Fetched home page data at: {end}");
        result.ok()
    });

    rsx! {
        article {
            class: "card card-xl h-full flex flex-col",
            // Latest blog post (top half - 50vh)
            div {
                class: "flex-1 max-h-[50vh] min-h-[50vh] overflow-hidden border-b border-base-300",
                header {
                    class: "card-header bg-base-100 sticky top-0 z-10",
                    h2 {
                        class: "text-xl sm:text-2xl md:text-3xl card-title",
                        "Latest Blog"
                    }
                }
                section {
                    class: "card-body overflow-y-auto h-[calc(50vh-5rem)]",
                    {
                        match home_data.read().as_ref() {
                            Some(Some(data)) => {
                                if let Some(article) = &data.first_article {
                                    rsx! {
                                        FullArticlePreview {
                                            article: article.clone()
                                        }
                                    }
                                } else {
                                    rsx! {
                                        div {
                                            class: "flex justify-center items-center p-8",
                                            "No articles available"
                                        }
                                    }
                                }
                            },
                            _ => rsx! {
                                BlogSkeleton {}
                            }
                        }
                    }
                }
            }

            // Recent articles (bottom half - scrollable)
            div {
                class: "flex-1 max-h-[50vh] min-h-[50vh] overflow-hidden",
                header {
                    class: "card-header bg-base-100 sticky top-0 z-10",
                    h2 {
                        class: "text-lg sm:text-xl md:text-2xl card-title",
                        "Recent Articles"
                    }
                }
                section {
                    class: "card-body overflow-y-auto h-[calc(50vh-5rem)]",
                    {
                        match home_data.read().as_ref() {
                            Some(Some(data)) => {
                                // Skip the first article (already shown above)
                                let recent_articles: Vec<ArticleWithMetadata> = data.recent_articles
                                    .iter()
                                    .skip(1)
                                    .take(5)
                                    .cloned()
                                    .collect();

                                rsx! {
                                    div {
                                        class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                                        for article in recent_articles {
                                            ArticleSummaryCard {
                                                article: article.clone()
                                            }
                                        }
                                    }
                                }
                            },
                            _ => rsx! {
                                BlogSkeleton {}
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn FullArticlePreview(article: ArticleWithMetadata) -> Element {
    // Extract first few paragraphs for preview (limit to ~300 chars)
    let preview_content = article.content.chars().take(300).collect::<String>() + "...";

    rsx! {
        div {
            class: "space-y-4 flex md:flex-row h-full gap-3 items-center",

            // Thumbnail if available
            if let Some(ref metadata) = article.toml_metadata {
                if let Some(ref thumbnail) = metadata.thumbnail {
                    div {
                        class: "w-full h-32 overflow-hidden rounded-lg",
                        img {
                            src: "{thumbnail}",
                            alt: "Article thumbnail",
                            class: "w-full h-full object-cover"
                        }
                    }
                }
            }

            // Metadata
            if let Some(ref metadata) = article.toml_metadata {
                ArticleMetadataDisplay {
                    metadata: metadata.clone(),
                    is_full: true
                }
            }
            div {
                // Title as link
                Link {
                    to: format!("/article/{}", article.metadata.path.trim_end_matches(".md")),
                    class: "link link-primary",
                    h1 {
                        class: "text-2xl font-bold hover:opacity-80",
                        "{article.metadata.title}"
                    }
                }

                // Summary or content preview
                if let Some(ref metadata) = article.toml_metadata {
                    if let Some(ref summary) = metadata.summary {
                        p {
                            class: "text-lg text-base-content opacity-80",
                            "{summary}"
                        }
                    } else {
                        p {
                            class: "text-base-content opacity-80",
                            "{preview_content}"
                        }
                    }
                } else {
                    p {
                        class: "text-base-content opacity-80",
                        "{preview_content}"
                    }
                }

                // Read more link
                Link {
                    to: format!("/article/{}", article.metadata.path.trim_end_matches(".md")),
                    class: "btn btn-primary",
                    "Read Full Article →"
                }
            }
        }
    }
}

#[component]
fn ArticleSummaryCard(article: ArticleWithMetadata) -> Element {
    rsx! {
        Link {
            to: format!("/article/{}", article.metadata.path.trim_end_matches(".md")),
            class: "card card-sm image-full bg-base-200 shadow-sm transition-all hover:scale-[1.02]",

            // Thumbnail if available
            if let Some(ref metadata) = article.toml_metadata {
                if let Some(ref thumbnail) = metadata.thumbnail {
                    figure {
                        class: "h-24 overflow-hidden",
                        img {
                            src: "{thumbnail}",
                            alt: "Article thumbnail",
                            class: "w-full h-full object-cover"
                        }
                    }
                }
            }

            div {
                class: "card-body",

                // Title
                h3 {
                    class: "card-title text-xl mb-2",
                    "{article.metadata.title}"
                }

                // Metadata
                if let Some(ref metadata) = article.toml_metadata {
                    ArticleMetadataDisplay {
                        metadata: metadata.clone(),
                        is_full: false
                    }

                    // Summary
                    if let Some(ref summary) = metadata.summary {
                        p {
                            class: "text-base-content opacity-70 mt-3 line-clamp-3",
                            "{summary}"
                        }
                    }
                }

                // Read more
                div {
                    class: "card-actions justify-end mt-4",
                    span {
                        class: "link link-primary text-sm",
                        "Read more →"
                    }
                }
            }
        }
    }
}

#[component]
fn ArticleMetadataDisplay(metadata: ArticleTomlMetadata, is_full: bool) -> Element {
    rsx! {
        div {
            class: if is_full { "flex flex-wrap gap-3 items-center text-sm" } else { "flex flex-wrap gap-2 items-center text-xs" },

            // Date
            if let Some(ref date) = metadata.date {
                span {
                    class: "badge badge-ghost",
                    "📅 {date}"
                }
            }

            // Reading time
            if let Some(ref reading_time) = metadata.reading_time {
                span {
                    class: "badge badge-ghost",
                    "⏱️ {reading_time}"
                }
            }

            // Category
            if let Some(ref category) = metadata.category {
                span {
                    class: "badge badge-primary",
                    "{category}"
                }
            }

            // Topics/Tags (show first few)
            if is_full {
                for topic in metadata.topics.iter().take(3) {
                    span {
                        class: "badge badge-secondary",
                        "{topic}"
                    }
                }
            }
        }
    }
}

#[component]
fn BlogItem(source: String) -> Element {
    rsx! {
        article {
            class: "card",
            div {
                class: "prose max-w-none",
                Markdown {
                    content: source,
                }
            }
        }
    }
}

#[component]
fn BlogSkeleton() -> Element {
    rsx! {
        div {
            class: "animate-pulse p-8 space-y-4",
            // Title skeleton
            div {
                class: "h-8 bg-gray-300 rounded w-3/4",
            }
            // Paragraph skeletons
            div {
                class: "space-y-3",
                div { class: "h-4 bg-gray-300 rounded" }
                div { class: "h-4 bg-gray-300 rounded w-5/6" }
                div { class: "h-4 bg-gray-300 rounded w-4/6" }
            }
            div {
                class: "space-y-3 pt-4",
                div { class: "h-4 bg-gray-300 rounded" }
                div { class: "h-4 bg-gray-300 rounded w-5/6" }
            }
        }
    }
}
