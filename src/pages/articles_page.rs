use dioxus::prelude::*;
use crate::markdown_management::{fetch_standalone_articles, ArticleWithMetadata};

const ARTICLES_PER_PAGE: usize = 12;

#[component]
pub fn ArticlesPage() -> Element {
    let mut current_page = use_signal(|| 1usize);

    // Fetch paginated articles from server
    let articles_data = use_resource(move || async move {
        let page = current_page.read().clone();
        fetch_standalone_articles(page, ARTICLES_PER_PAGE).await.ok()
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
                        "Standalone Articles"
                    }
                    p {
                        class: "text-lg text-base-content opacity-70",
                        "Browse articles that are not part of any series"
                    }
                }

                // Content
                match articles_data.read().as_ref() {
                    Some(Some(paginated_data)) => {
                        if paginated_data.articles.is_empty() {
                            rsx! {
                                div {
                                    class: "text-center py-12",
                                    p {
                                        class: "text-lg text-base-content opacity-70",
                                        if paginated_data.page > 1 {
                                            "No articles found on this page"
                                        } else {
                                            "No standalone articles found"
                                        }
                                    }
                                }
                            }
                        } else {
                            rsx! {
                                // Article count
                                div {
                                    class: "mb-6",
                                    p {
                                        class: "text-sm text-base-content opacity-60",
                                        "{paginated_data.total_count} standalone articles found"
                                    }
                                }

                                // Articles grid
                                div {
                                    class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 mb-8",
                                    for article in &paginated_data.articles {
                                        ArticleCard { article: article.clone() }
                                    }
                                }

                                // Pagination controls
                                if paginated_data.total_pages > 1 {
                                    Pagination {
                                        current_page: paginated_data.page,
                                        total_pages: paginated_data.total_pages,
                                        on_page_change: move |page| {
                                            current_page.set(page);
                                        }
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
                                "Failed to load articles"
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
                                "Loading articles..."
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ArticleCard(article: ArticleWithMetadata) -> Element {
    let thumbnail = article.toml_metadata.as_ref()
        .and_then(|m| m.thumbnail.clone());

    rsx! {
        Link {
            to: format!("/article/{}", article.metadata.path.trim_end_matches(".md")),
            class: "card bg-base-100 shadow-md hover:shadow-xl transition-all duration-300 border-2 border-transparent hover:border-primary cursor-pointer h-full flex flex-col",

            // Thumbnail if available
            if let Some(thumb_url) = thumbnail {
                figure {
                    class: "aspect-video overflow-hidden",
                    img {
                        src: "{thumb_url}",
                        alt: "{article.metadata.title}",
                        class: "w-full h-full object-cover"
                    }
                }
            }

            div {
                class: "card-body flex-1 flex flex-col",

                // Title
                h3 {
                    class: "card-title text-lg mb-2",
                    "{article.metadata.title}"
                }

                // Metadata
                if let Some(ref toml_meta) = article.toml_metadata {
                    div {
                        class: "flex flex-wrap gap-2 text-xs mb-3",

                        if let Some(ref date) = toml_meta.date {
                            span {
                                class: "badge badge-sm badge-outline",
                                "📅 {date}"
                            }
                        }

                        if let Some(ref reading_time) = toml_meta.reading_time {
                            span {
                                class: "badge badge-sm badge-outline",
                                "⏱️ {reading_time}"
                            }
                        }

                        if let Some(ref category) = toml_meta.category {
                            span {
                                class: "badge badge-sm badge-primary",
                                "{category}"
                            }
                        }
                    }

                    // Topics
                    if !toml_meta.topics.is_empty() {
                        div {
                            class: "flex flex-wrap gap-1 mb-3",
                            for topic in &toml_meta.topics {
                                span {
                                    class: "badge badge-xs badge-ghost",
                                    "{topic}"
                                }
                            }
                        }
                    }

                    // Summary
                    if let Some(ref summary) = toml_meta.summary {
                        p {
                            class: "text-sm text-base-content opacity-70 line-clamp-3 flex-1",
                            "{summary}"
                        }
                    }
                }

                // Read more button
                div {
                    class: "card-actions justify-end mt-auto pt-4",
                    button {
                        class: "btn btn-sm btn-primary gap-2",
                        "Read Article"
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

#[component]
fn Pagination(
    current_page: usize,
    total_pages: usize,
    on_page_change: EventHandler<usize>
) -> Element {
    // Calculate page range to show
    let start_page = if current_page <= 3 {
        1
    } else {
        current_page - 2
    };

    let end_page = (start_page + 4).min(total_pages);

    rsx! {
        div {
            class: "flex justify-center items-center gap-2 py-8",

            // Previous button
            button {
                class: "btn btn-sm",
                disabled: current_page == 1,
                onclick: move |_| {
                    if current_page > 1 {
                        on_page_change.call(current_page - 1);
                    }
                },
                "«"
            }

            // First page
            if start_page > 1 {
                button {
                    class: "btn btn-sm",
                    onclick: move |_| on_page_change.call(1),
                    "1"
                }
                if start_page > 2 {
                    span { class: "px-2", "..." }
                }
            }

            // Page numbers
            for page in start_page..=end_page {
                button {
                    class: if page == current_page {
                        "btn btn-sm btn-primary"
                    } else {
                        "btn btn-sm"
                    },
                    onclick: move |_| on_page_change.call(page),
                    "{page}"
                }
            }

            // Last page
            if end_page < total_pages {
                if end_page < total_pages - 1 {
                    span { class: "px-2", "..." }
                }
                button {
                    class: "btn btn-sm",
                    onclick: move |_| on_page_change.call(total_pages),
                    "{total_pages}"
                }
            }

            // Next button
            button {
                class: "btn btn-sm",
                disabled: current_page == total_pages,
                onclick: move |_| {
                    if current_page < total_pages {
                        on_page_change.call(current_page + 1);
                    }
                },
                "»"
            }
        }
    }
}
