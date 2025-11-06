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
    let article_label = if article_count == 1 { "article" } else { "articles" };

    rsx! {
        Link {
            to: format!("/series/{}", series.name),
            class: "card card-lg bg-base-200 hover:shadow-xl transition-all duration-300 border-2 border-transparent hover:border-primary cursor-pointer",
            article {
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

                // Short Summary
                div {
                    class: "text-base-content opacity-70 mb-4 flex-grow",
                    if let Some(ref short_summary) = series.short_summary {
                        p {
                            "{short_summary}"
                        }
                    } else {
                        p {
                            "A series of {article_count} articles exploring various topics"
                        }
                    }
                }

                // View series button
                div {
                    class: "card-actions justify-end",
                    button {
                        class: "btn btn-primary btn-sm gap-2",
                        "View Series"
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
                                d: "M9 5l7 7-7 7"
                            }
                        }
                    }
                }
            }
        }
    }
}
