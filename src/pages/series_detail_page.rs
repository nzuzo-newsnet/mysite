use dioxus::prelude::*;
use crate::markdown_management::fetch_series_by_name;
use dioxus_markdown::Markdown;

#[component]
pub fn SeriesDetailPage(series_name: String) -> Element {
    // Fetch series data from server
    let series_data = use_resource(move || {
        let name = series_name.clone();
        async move { fetch_series_by_name(name).await.ok() }
    });

    rsx! {
        main {
            class: "flex-1 overflow-y-auto p-8",
            div {
                class: "container mx-auto max-w-5xl",

                // Content
                match series_data.read().as_ref() {
                    Some(Some(series)) => rsx! {
                        div {
                            class: "space-y-8",

                            // Back button
                            div {
                                class: "mb-4",
                                Link {
                                    to: "/series",
                                    class: "btn btn-ghost btn-sm gap-2",
                                    svg {
                                        xmlns: "http://www.w3.org/2000/svg",
                                        class: "h-4 w-4",
                                        fill: "none",
                                        view_box: "0 0 24 24",
                                        stroke: "currentColor",
                                        path {
                                            stroke_linecap: "round",
                                            stroke_linejoin: "round",
                                            stroke_width: "2",
                                            d: "M15 19l-7-7 7-7"
                                        }
                                    }
                                    "Back to Series"
                                }
                            }

                            // Header
                            div {
                                class: "mb-8",
                                h1 {
                                    class: "text-4xl font-bold mb-4",
                                    "{series.name}"
                                }
                                div {
                                    class: "flex gap-2 items-center",
                                    {
                                        let article_label = if series.total_articles == 1 { "article" } else { "articles" };
                                        rsx! {
                                            span {
                                                class: "badge badge-primary badge-lg",
                                                "{series.total_articles} {article_label}"
                                            }
                                        }
                                    }
                                }
                            }

                            // Long summary/description
                            if let Some(ref long_summary) = series.long_summary {
                                article {
                                    class: "card card-lg bg-base-200 mb-8",
                                    div {
                                        class: "card-body prose prose-lg max-w-none",
                                        Markdown {
                                            content: long_summary.clone()
                                        }
                                    }
                                }
                            }

                            // Articles list
                            div {
                                class: "space-y-4",
                                h2 {
                                    class: "text-2xl font-bold mb-4",
                                    "Articles in this series"
                                }

                                if series.articles.is_empty() {
                                    div {
                                        class: "text-center py-8",
                                        p {
                                            class: "text-lg text-base-content opacity-70",
                                            "No articles in this series yet."
                                        }
                                    }
                                } else {
                                    div {
                                        class: "space-y-3",
                                        for (idx, article) in series.articles.iter().enumerate() {
                                            ArticleCard {
                                                article: article.clone(),
                                                index: idx + 1
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    },
                    Some(None) => rsx! {
                        div {
                            class: "text-center py-12",
                            div {
                                class: "alert alert-error max-w-md mx-auto",
                                svg {
                                    xmlns: "http://www.w3.org/2000/svg",
                                    class: "stroke-current shrink-0 h-6 w-6",
                                    fill: "none",
                                    view_box: "0 0 24 24",
                                    path {
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        d: "M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"
                                    }
                                }
                                span { "Series not found" }
                            }
                            Link {
                                to: "/series",
                                class: "btn btn-primary mt-4",
                                "Back to Series"
                            }
                        }
                    },
                    None => rsx! {
                        div {
                            class: "text-center py-12",
                            span {
                                class: "loading loading-spinner loading-lg"
                            }
                            p {
                                class: "mt-4 text-base-content opacity-70",
                                "Loading series..."
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ArticleCard(article: crate::markdown_management::ArticleWithMetadata, index: usize) -> Element {
    rsx! {
        Link {
            to: format!("/article/{}", article.metadata.path.trim_end_matches(".md")),
            class: "card card-compact bg-base-100 border-2 border-base-300 hover:border-primary hover:shadow-lg transition-all duration-300",
            div {
                class: "card-body",
                div {
                    class: "flex items-start gap-4",

                    // Index badge
                    div {
                        class: "flex-shrink-0",
                        span {
                            class: "badge badge-primary badge-lg",
                            "{index}"
                        }
                    }

                    // Article info
                    div {
                        class: "flex-1 min-w-0",
                        h3 {
                            class: "font-bold text-lg mb-1",
                            "{article.metadata.title}"
                        }

                        // Metadata
                        if let Some(ref toml_meta) = article.toml_metadata {
                            div {
                                class: "flex flex-wrap gap-3 text-sm text-base-content opacity-70",

                                if let Some(ref date) = toml_meta.date {
                                    span {
                                        class: "flex items-center gap-1",
                                        "📅 {date}"
                                    }
                                }

                                if let Some(ref reading_time) = toml_meta.reading_time {
                                    span {
                                        class: "flex items-center gap-1",
                                        "⏱️ {reading_time}"
                                    }
                                }
                            }

                            // Summary
                            if let Some(ref summary) = toml_meta.summary {
                                p {
                                    class: "text-base-content opacity-70 mt-2 line-clamp-2",
                                    "{summary}"
                                }
                            }
                        }
                    }

                    // Arrow icon
                    div {
                        class: "flex-shrink-0",
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            class: "h-6 w-6",
                            fill: "none",
                            view_box: "0 0 24 24",
                            stroke: "currentColor",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                d: "M9 5l7 7-7 7"
                            }
                        }
                    }
                }
            }
        }
    }
}
