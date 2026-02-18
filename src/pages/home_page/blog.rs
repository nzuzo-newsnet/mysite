use dioxus::prelude::*;

use crate::markdown_management::{
    ArticleWithMetadata, fetch_home_page_data_with_metadata,
};

#[component]
pub fn Blogs() -> Element {
    // Fetch articles with metadata
    let home_data = use_resource(|| async move {
        fetch_home_page_data_with_metadata().await.ok()
    });

    rsx! {
        div {
            class: "space-y-12",
            
            // Featured Post Section
            div {
                h2 {
                    class: "text-xl font-bold mb-6 flex items-center gap-2",
                    span { class: "w-2 h-6 bg-primary rounded-full" }
                    "Latest Deep Dive"
                }
                
                {
                    match home_data.read().as_ref() {
                        Some(Some(data)) => {
                            if let Some(article) = &data.first_article {
                                rsx! {
                                    FeaturedArticle {
                                        article: article.clone()
                                    }
                                }
                            } else {
                                rsx! {
                                    div { class: "text-center py-12 bg-base-200 rounded-xl", "No articles found." }
                                }
                            }
                        },
                        _ => rsx! { BlogSkeleton {} }
                    }
                }
            }

            // Recent Writing Section
            div {
                h2 {
                    class: "text-xl font-bold mb-6 flex items-center gap-2",
                    span { class: "w-2 h-6 bg-secondary rounded-full" }
                    "Recent Writing"
                }
                
                div {
                    class: "grid gap-6 md:grid-cols-2",
                    {
                        match home_data.read().as_ref() {
                            Some(Some(data)) => {
                                let recent_articles: Vec<ArticleWithMetadata> = data.recent_articles
                                    .iter()
                                    .skip(1)
                                    .cloned()
                                    .collect();

                                rsx! {
                                    for article in recent_articles.iter().take(4) {
                                        ArticleCard {
                                            article: article.clone()
                                        }
                                    }
                                    
                                    // See all button
                                    Link {
                                        to: "/articles",
                                        class: "md:col-span-2 py-4 rounded-xl border border-dashed border-base-300 flex items-center justify-center text-primary font-medium hover:bg-base-200 transition-colors",
                                        "View All Articles →"
                                    }
                                }
                            },
                            _ => rsx! {
                                for _ in 0..4 {
                                    div { class: "h-48 bg-base-200 animate-pulse rounded-xl" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn FeaturedArticle(article: ArticleWithMetadata) -> Element {
    let thumbnail = article.toml_metadata.as_ref().and_then(|m| m.thumbnail.clone());
    let date = article.toml_metadata.as_ref().and_then(|m| m.date.clone()).unwrap_or_default();
    let category = article.toml_metadata.as_ref().and_then(|m| m.category.clone()).unwrap_or_else(|| "Deep Dive".to_string());
    let summary = article.toml_metadata.as_ref().and_then(|m| m.summary.clone()).unwrap_or_else(|| {
        article.content.chars().take(200).collect::<String>() + "..."
    });

    rsx! {
        div {
            class: "rounded-xl overflow-hidden shadow-lg transition-all hover:shadow-xl border border-base-300 bg-base-100",
            div {
                class: "md:flex",
                // Visual / Thumbnail
                div {
                    class: "md:w-2/5 h-48 md:h-auto bg-gradient-to-br from-primary/20 to-secondary/20 relative overflow-hidden flex items-center justify-center",
                    if let Some(src) = thumbnail {
                        img {
                            src: "{src}",
                            class: "absolute inset-0 w-full h-full object-cover opacity-80"
                        }
                    } else {
                        svg { class: "w-20 h-20 opacity-20 text-primary", fill: "none", stroke: "currentColor", view_box: "0 0 24 24", xmlns: "http://www.w3.org/2000/svg",
                            path { stroke_linecap: "round", stroke_linejoin: "round", stroke_width: "1.5", d: "M19.428 15.428a2 2 0 00-1.022-.547l-2.387-.477a6 6 0 00-3.86.517l-.67.335a2 2 0 01-1.286.172l-1.63-.408a2 2 0 01-1.327-1.185l-1.012-2.53a2 2 0 00-1.022-.547l-2.387-.477a6 6 0 00-3.86.517l-.67.335a2 2 0 01-1.286.172l-1.63-.408a2 2 0 01-1.327-1.185l-1.012-2.53a2 2 0 00-1.022-.547l-2.387-.477a6 6 0 00-3.86.517l-.67.335a2 2 0 01-1.286.172l-1.63-.408a2 2 0 01-1.327-1.185l-1.012-2.53a2 2 0 00-1.022-.547l-2.387-.477a6 6 0 00-3.86.517l-.67.335a2 2 0 01-1.286.172l-1.63-.408a2 2 0 01-1.327-1.185l-1.012-2.53z" }
                        }
                    }
                }
                // Content
                div {
                    class: "p-6 md:p-8 md:w-3/5 flex flex-col justify-center",
                    div {
                        class: "flex items-center gap-3 text-xs font-semibold uppercase tracking-wider mb-2",
                        span { class: "text-primary", "{category}" }
                        span { class: "text-base-content/50 flex items-center gap-1", 
                            svg { class: "w-3 h-3", fill: "none", stroke: "currentColor", view_box: "0 0 24 24", xmlns: "http://www.w3.org/2000/svg",
                                path { stroke_linecap: "round", stroke_linejoin: "round", stroke_width: "2", d: "M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" }
                            }
                            "{date}" 
                        }
                    }
                    Link {
                        to: format!("/article/{}", article.metadata.path.trim_end_matches(".md")),
                        class: "text-2xl font-bold mb-3 hover:text-primary transition-colors cursor-pointer block",
                        "{article.metadata.title}"
                    }
                    p {
                        class: "mb-4 line-clamp-3 text-base-content/70",
                        "{summary}"
                    }
                    Link {
                        to: format!("/article/{}", article.metadata.path.trim_end_matches(".md")),
                        class: "inline-flex items-center text-primary hover:text-primary-focus font-medium text-sm group",
                        "Read Article"
                        svg { class: "ml-1 w-4 h-4 group-hover:translate-x-1 transition-transform", fill: "none", stroke: "currentColor", view_box: "0 0 24 24", xmlns: "http://www.w3.org/2000/svg",
                            path { stroke_linecap: "round", stroke_linejoin: "round", stroke_width: "2", d: "M9 5l7 7-7 7" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ArticleCard(article: ArticleWithMetadata) -> Element {
    let date = article.toml_metadata.as_ref().and_then(|m| m.date.clone()).unwrap_or_default();
    let category = article.toml_metadata.as_ref().and_then(|m| m.category.clone()).unwrap_or_else(|| "Article".to_string());
    let read_time = article.toml_metadata.as_ref().and_then(|m| m.reading_time.clone()).unwrap_or_else(|| "5 min read".to_string());
    let summary = article.toml_metadata.as_ref().and_then(|m| m.summary.clone()).unwrap_or_else(|| {
        article.content.chars().take(150).collect::<String>() + "..."
    });

    rsx! {
        Link {
            to: format!("/article/{}", article.metadata.path.trim_end_matches(".md")),
            class: "rounded-xl p-5 border border-base-300 bg-base-100 transition-all hover:-translate-y-1 hover:shadow-lg hover:border-primary/50 group cursor-pointer block",
            div {
                class: "flex items-center justify-between mb-3",
                div {
                    class: "flex items-center gap-2 text-xs",
                    span { class: "px-2 py-1 rounded-md font-medium bg-base-200 text-base-content/70", "{category}" }
                    span { class: "text-base-content/40", "{read_time}" }
                }
                span { class: "text-xs text-base-content/40", "{date}" }
            }
            h3 {
                class: "text-lg font-bold mb-2 group-hover:text-primary transition-colors line-clamp-2",
                "{article.metadata.title}"
            }
            p {
                class: "text-sm line-clamp-3 text-base-content/60",
                "{summary}"
            }
        }
    }
}

#[component]
fn BlogSkeleton() -> Element {
    rsx! {
        div {
            class: "animate-pulse rounded-xl border border-base-300 bg-base-100 h-64 flex flex-col md:flex-row",
            div { class: "md:w-2/5 bg-base-200 h-48 md:h-auto" }
            div {
                class: "p-6 md:p-8 md:w-3/5 space-y-4",
                div { class: "h-4 bg-base-200 rounded w-1/4" }
                div { class: "h-8 bg-base-200 rounded w-3/4" }
                div { class: "space-y-2",
                    div { class: "h-4 bg-base-200 rounded" }
                    div { class: "h-4 bg-base-200 rounded w-5/6" }
                }
            }
        }
    }
}

