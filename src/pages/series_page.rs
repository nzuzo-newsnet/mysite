use dioxus::prelude::*;
use crate::markdown_management::{fetch_all_series, SeriesData};

#[component]
pub fn SeriesPage() -> Element {
    // Fetch series data from server
    let series_data = use_resource(|| async move {
        fetch_all_series().await.ok()
    });

    rsx! {
        main {
            class: "flex-1 overflow-y-auto p-8",
            div {
                class: "container mx-auto max-w-7xl",

                // Header
                div {
                    class: "mb-8",
                    h1 {
                        class: "text-3xl font-bold mb-4",
                        "Article Series"
                    }
                    p {
                        class: "text-lg text-base-content opacity-70",
                        "Explore multi-part articles organized by topic and folder structure"
                    }
                }

                // Content
                match series_data.read().as_ref() {
                    Some(Some(series_list)) => {
                        if series_list.is_empty() {
                            rsx! {
                                div {
                                    class: "text-center py-12",
                                    p {
                                        class: "text-lg text-base-content opacity-70",
                                        "No article series found. Create folders in the articles directory to organize articles into series."
                                    }
                                }
                            }
                        } else {
                            rsx! {
                                // Series count
                                div {
                                    class: "mb-6",
                                    p {
                                        class: "text-sm text-base-content opacity-60",
                                        "{series_list.len()} series found"
                                    }
                                }

                                // Series grid
                                div {
                                    class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6",
                                    for series in series_list {
                                        SeriesCard { series: series.clone() }
                                    }
                                }
                            }
                        }
                    }
                    Some(None) => rsx! {
                        div {
                            class: "text-center py-12",
                            p {
                                class: "text-lg text-error",
                                "Failed to load series data"
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
fn SeriesCard(series: SeriesData) -> Element {
    let article_count = series.articles.len();

    // Get description from first article's summary if available
    let description = series.articles.first()
        .and_then(|article| article.toml_metadata.as_ref())
        .and_then(|metadata| metadata.summary.clone())
        .unwrap_or_else(|| format!("A series of {} articles", article_count));

    let article_label = if article_count == 1 { "article" } else { "articles" };

    rsx! {
        article {
            class: "card card-lg bg-base-200 hover:shadow-xl transition-all duration-300 border-2 border-transparent hover:border-primary",
            div {
                class: "card-body",

                // Header with series name
                div {
                    class: "mb-3",
                    h3 {
                        class: "card-title text-xl",
                        "{series.name}"
                    }
                    span {
                        class: "badge badge-primary mt-2",
                        "{article_count} {article_label}"
                    }
                }

                // Description
                p {
                    class: "text-base-content opacity-70 mb-4 flex-grow",
                    "{description}"
                }

                // Article links
                if !series.articles.is_empty() {
                    div {
                        class: "space-y-2 mb-4",
                        for (idx, article) in series.articles.iter().enumerate() {
                            Link {
                                to: format!("/article/{}", article.metadata.path.trim_end_matches(".md")),
                                class: "btn btn-sm btn-ghost w-full justify-start gap-2 hover:btn-primary",
                                span {
                                    class: "badge badge-outline badge-sm",
                                    "{idx + 1}"
                                }
                                span {
                                    class: "truncate text-left flex-1",
                                    "{article.metadata.title}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
