use dioxus::prelude::*;
use dioxus_markdown::Markdown;

use crate::markdown_management::{fetch_article_with_metadata, ArticleTomlMetadata};

#[component]
pub fn ArticlePage(path: String) -> Element {
    let article_path = path.clone();
    let active_tab = use_signal(|| "article".to_string());

    // Fetch article with metadata from server based on the path
    let article_data = use_resource(move || {
        let path = article_path.clone();
        async move {
            fetch_article_with_metadata(path).await
        }
    });

    rsx! {
        main {
            class: "flex-1 overflow-y-auto p-8 pb-32",
            div {
                class: "container mx-auto max-w-4xl",
            article {
                class: "card card-xl",
                section {
                    class: "card-body",
                    {
                        match article_data.read().as_ref() {
                            Some(Ok(article)) => {
                                let metadata = article.toml_metadata.clone();
                                rsx! {
                                    div {
                                        class: "space-y-6",

                                        // Article metadata
                                        if let Some(ref meta) = article.toml_metadata {
                                            ArticleMetadata {
                                                metadata: meta.clone()
                                            }
                                        }

                                        // Tab content
                                        div {
                                            class: "min-h-[500px]",
                                            match active_tab.read().as_str() {
                                                "article" => rsx! {
                                                    div {
                                                        class: "prose prose-lg max-w-none",
                                                        Markdown {
                                                            content: article.content.clone(),
                                                        }
                                                    }

                                                    // Tags section
                                                    if let Some(ref meta) = article.toml_metadata {
                                                        if !meta.tags.is_empty() {
                                                            div {
                                                                class: "border-t border-base-300 pt-6 mt-8",
                                                                h3 {
                                                                    class: "text-lg font-semibold mb-3",
                                                                    "Tags"
                                                                }
                                                                div {
                                                                    class: "flex flex-wrap gap-2",
                                                                    for tag in &meta.tags {
                                                                        span {
                                                                            class: "badge badge-outline badge-lg",
                                                                            "#{tag}"
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                },
                                                "references" => rsx! {
                                                    div {
                                                        class: "p-8",
                                                        h2 {
                                                            class: "text-xl font-bold mb-4",
                                                            "References & Resources"
                                                        }
                                                        p {
                                                            class: "text-base-content opacity-70",
                                                            "References and resources will appear here once the advanced markdown parser is integrated."
                                                        }
                                                    }
                                                },
                                                "demo" => rsx! {
                                                    div {
                                                        class: "p-8",
                                                        h2 {
                                                            class: "text-xl font-bold mb-4",
                                                            "Interactive Demo"
                                                        }
                                                        p {
                                                            class: "text-base-content opacity-70",
                                                            "Interactive demos will appear here once configured in article metadata."
                                                        }
                                                    }
                                                },
                                                "related" => rsx! {
                                                    div {
                                                        class: "p-8",
                                                        h2 {
                                                            class: "text-xl font-bold mb-4",
                                                            "Related Articles"
                                                        }
                                                        p {
                                                            class: "text-base-content opacity-70",
                                                            "Related articles will be suggested here based on topics and series."
                                                        }
                                                    }
                                                },
                                                "quiz" => rsx! {
                                                    div {
                                                        class: "p-8",
                                                        h2 {
                                                            class: "text-xl font-bold mb-4",
                                                            "Check Your Knowledge"
                                                        }
                                                        p {
                                                            class: "text-base-content opacity-70",
                                                            "Knowledge checks and quizzes will appear here."
                                                        }
                                                    }
                                                },
                                                _ => rsx! {
                                                    div {
                                                        class: "p-8",
                                                        "Content not available"
                                                    }
                                                }
                                            }
                                        }
                                    }

                                    // Bottom Navigation Panel (Floating & Sticky)
                                    BottomNavPanel {
                                        active_tab: active_tab,
                                        metadata: metadata
                                    }
                                }
                            },
                            Some(Err(e)) => rsx! {
                                div {
                                    class: "alert alert-error",
                                    h3 { "Error Loading Article" }
                                    p { "{e}" }
                                }
                            },
                            None => rsx! {
                                ArticleSkeleton {}
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
fn ArticleMetadata(metadata: ArticleTomlMetadata) -> Element {
    rsx! {
        div {
            class: "bg-base-200 rounded-lg p-6 space-y-4",

            // Author and date
            div {
                class: "flex flex-wrap items-center gap-4 text-sm",
                if let Some(ref author) = metadata.author {
                    div {
                        class: "flex items-center gap-2",
                        span {
                            class: "font-semibold",
                            "By {author}"
                        }
                    }
                }
                if let Some(ref date) = metadata.date {
                    div {
                        class: "flex items-center gap-2",
                        span {
                            class: "badge badge-ghost",
                            "📅 {date}"
                        }
                    }
                }
                if let Some(ref reading_time) = metadata.reading_time {
                    div {
                        class: "flex items-center gap-2",
                        span {
                            class: "badge badge-ghost",
                            "⏱️ {reading_time}"
                        }
                    }
                }
            }

            // Summary
            if let Some(ref summary) = metadata.summary {
                div {
                    class: "text-base-content opacity-70 italic border-l-4 border-primary pl-4",
                    p { "{summary}" }
                }
            }

            // Topics and category
            div {
                class: "flex flex-wrap gap-2",
                if let Some(ref category) = metadata.category {
                    span {
                        class: "badge badge-primary badge-lg",
                        "{category}"
                    }
                }
                for topic in &metadata.topics {
                    span {
                        class: "badge badge-secondary badge-lg",
                        "{topic}"
                    }
                }
            }

            // Thumbnail
            if let Some(ref thumbnail) = metadata.thumbnail {
                div {
                    class: "mt-4",
                    img {
                        src: "{thumbnail}",
                        alt: "Article thumbnail",
                        class: "rounded-lg w-full max-h-96 object-cover"
                    }
                }
            }
        }
    }
}

#[component]
fn ArticleSkeleton() -> Element {
    rsx! {
        div {
            class: "animate-pulse space-y-6 p-8",

            // Metadata skeleton
            div {
                class: "bg-gray-200 rounded-lg p-6 space-y-4",
                div {
                    class: "flex gap-4",
                    div { class: "h-4 bg-gray-300 rounded w-32" }
                    div { class: "h-4 bg-gray-300 rounded w-24" }
                }
                div { class: "h-16 bg-gray-300 rounded" }
                div {
                    class: "flex gap-2",
                    div { class: "h-6 bg-gray-300 rounded w-20" }
                    div { class: "h-6 bg-gray-300 rounded w-24" }
                    div { class: "h-6 bg-gray-300 rounded w-20" }
                }
            }

            // Title skeleton
            div { class: "h-10 bg-gray-200 rounded w-3/4" }

            // Content skeleton
            div {
                class: "space-y-3",
                div { class: "h-4 bg-gray-200 rounded" }
                div { class: "h-4 bg-gray-200 rounded w-5/6" }
                div { class: "h-4 bg-gray-200 rounded w-4/6" }
            }
            div {
                class: "space-y-3 pt-4",
                div { class: "h-4 bg-gray-200 rounded" }
                div { class: "h-4 bg-gray-200 rounded w-5/6" }
            }
        }
    }
}

#[component]
fn BottomNavPanel(active_tab: Signal<String>, metadata: Option<ArticleTomlMetadata>) -> Element {
    rsx! {
        div {
            class: "dock dock-lg fixed bottom-0 left-0 right-0 z-50 bg-base-100 border-t border-base-300 shadow-lg",

            // Article Tab (Required)
            button {
                class: if active_tab.read().as_str() == "article" { "dock-active" } else { "" },
                onclick: move |_| active_tab.set("article".to_string()),
                svg {
                    xmlns: "http://www.w3.org/2000/svg",
                    class: "h-5 w-5",
                    fill: "none",
                    view_box: "0 0 24 24",
                    stroke: "currentColor",
                    path {
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        stroke_width: "2",
                        d: "M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
                    }
                }
                span {
                    class: "dock-label",
                    "Article"
                }
            }

            // Next/Prev Navigation (Required)
            div {
                class: "flex flex-col items-center justify-center cursor-default gap-1",
                div {
                    class: "flex gap-2",
                    if let Some(ref meta) = metadata {
                        if let Some(ref prev) = meta.prev_article {
                            Link {
                                to: format!("/article/{}", prev),
                                class: "btn btn-xs btn-ghost",
                                "← Prev"
                            }
                        }
                        if let Some(ref next) = meta.next_article {
                            Link {
                                to: format!("/article/{}", next),
                                class: "btn btn-xs btn-ghost",
                                "Next →"
                            }
                        }
                    } else {
                        span {
                            class: "text-xs opacity-50",
                            "No navigation"
                        }
                    }
                }
                span {
                    class: "dock-label",
                    "Navigate"
                }
            }

            // References Tab (Optional)
            button {
                class: if active_tab.read().as_str() == "references" { "dock-active" } else { "" },
                onclick: move |_| active_tab.set("references".to_string()),
                svg {
                    xmlns: "http://www.w3.org/2000/svg",
                    class: "h-5 w-5",
                    fill: "none",
                    view_box: "0 0 24 24",
                    stroke: "currentColor",
                    path {
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        stroke_width: "2",
                        d: "M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"
                    }
                }
                span {
                    class: "dock-label",
                    "References"
                }
            }

            // Demo Tab (Optional)
            button {
                class: if active_tab.read().as_str() == "demo" { "dock-active" } else { "" },
                onclick: move |_| active_tab.set("demo".to_string()),
                svg {
                    xmlns: "http://www.w3.org/2000/svg",
                    class: "h-5 w-5",
                    fill: "none",
                    view_box: "0 0 24 24",
                    stroke: "currentColor",
                    path {
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        stroke_width: "2",
                        d: "M14.752 11.168l-3.197-2.132A1 1 0 0010 9.87v4.263a1 1 0 001.555.832l3.197-2.132a1 1 0 000-1.664z"
                    }
                    path {
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        stroke_width: "2",
                        d: "M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                    }
                }
                span {
                    class: "dock-label",
                    "Demo"
                }
            }

            // Related Articles Tab (Optional)
            button {
                class: if active_tab.read().as_str() == "related" { "dock-active" } else { "" },
                onclick: move |_| active_tab.set("related".to_string()),
                svg {
                    xmlns: "http://www.w3.org/2000/svg",
                    class: "h-5 w-5",
                    fill: "none",
                    view_box: "0 0 24 24",
                    stroke: "currentColor",
                    path {
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        stroke_width: "2",
                        d: "M13 10V3L4 14h7v7l9-11h-7z"
                    }
                }
                span {
                    class: "dock-label",
                    "Related"
                }
            }

            // Quiz Tab (Optional)
            button {
                class: if active_tab.read().as_str() == "quiz" { "dock-active" } else { "" },
                onclick: move |_| active_tab.set("quiz".to_string()),
                svg {
                    xmlns: "http://www.w3.org/2000/svg",
                    class: "h-5 w-5",
                    fill: "none",
                    view_box: "0 0 24 24",
                    stroke: "currentColor",
                    path {
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        stroke_width: "2",
                        d: "M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-6 9l2 2 4-4"
                    }
                }
                span {
                    class: "dock-label",
                    "Quiz"
                }
            }
        }
    }
}
