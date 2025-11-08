use dioxus::prelude::*;
use dioxus_markdown::Markdown;

use crate::markdown_management::{ArticleTomlMetadata, fetch_article_with_metadata};

#[component]
pub fn ArticlePage(path: String) -> Element {
    // Reset active tab to "article" when component mounts
    let active_tab = use_signal(|| "article".to_string());

    // Log when component mounts/remounts with new path
    dioxus::logger::tracing::info!("ArticlePage mounted/remounted with path: {}", path);

    // Fetch article with metadata from server based on the path
    // Since we're using a key in the parent, this will re-run when the path changes
    let article_data = use_resource(move || {
        let current_path = path.clone();
        dioxus::logger::tracing::info!("use_resource triggered for path: {}", current_path);
        async move {
            dioxus::logger::tracing::info!("Fetching article: {}", current_path);
            let result = fetch_article_with_metadata(current_path.clone()).await;
            match &result {
                Ok(_) => dioxus::logger::tracing::info!("Successfully fetched article: {}", current_path),
                Err(e) => dioxus::logger::tracing::error!("Failed to fetch article {}: {:?}", current_path, e),
            }
            result
        }
    });

    rsx! {
        main {
            class: "flex-1 overflow-hidden flex flex-col md:flex-row",

            // Main content area
            div {
                class: "flex-1 overflow-y-auto p-4 md:p-8 pb-32 md:pb-8",
                div {
                    class: "container mx-auto max-w-4xl",
                article {
                    class: "card card-xl",
                    section {
                        class: "card-body",
                    {
                        match article_data.read().as_ref() {
                            Some(Ok(article)) => {
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

                                                    // Next/Previous Navigation Cards
                                                    if let Some(ref meta) = article.toml_metadata {
                                                        NavigationCards {
                                                            metadata: meta.clone()
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
                                                                            class: "badge h-fit badge-outline badge-lg",
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

            // Right sidebar for md+ screens with tabs
            {
                if let Some(Ok(article)) = article_data.read().as_ref() {
                    rsx! {
                        div {
                            class: "hidden md:block md:w-80 lg:w-96 border-l border-base-300 bg-base-100 overflow-y-auto",
                            RightSidebar {
                                active_tab: active_tab,
                                metadata: article.toml_metadata.clone()
                            }
                        }
                    }
                } else {
                    rsx! {}
                }
            }

            // Bottom Navigation Panel for mobile only
            {
                if let Some(Ok(article)) = article_data.read().as_ref() {
                    rsx! {
                        div {
                            class: "md:hidden",
                            BottomNavPanel {
                                active_tab: active_tab,
                                metadata: article.toml_metadata.clone()
                            }
                        }
                    }
                } else {
                    rsx! {}
                }
            }
        }
    }
}

#[component]
fn RightSidebar(active_tab: Signal<String>, metadata: Option<ArticleTomlMetadata>) -> Element {
    rsx! {
        div {
            class: "flex flex-col h-full",

            // Navigation section at top
            if let Some(ref meta) = metadata {
                div {
                    class: "p-4 border-b border-base-300",
                    h3 {
                        class: "text-sm font-semibold mb-3 opacity-70",
                        "Navigate"
                    }

                    // Series info
                    if !meta.article_series.is_empty() {
                        {
                            let series = &meta.article_series[0];
                            rsx! {
                                div {
                                    class: "mb-3",
                                    span {
                                        class: "badge badge-sm badge-primary",
                                        "{series.name}"
                                    }
                                }
                            }
                        }
                    }

                    // Prev/Next buttons
                    div {
                        class: "flex gap-2",
                        if !meta.article_series.is_empty() {
                            {
                                let series = &meta.article_series[0];
                                rsx! {
                                    if let Some(ref prev) = series.prev {
                                        Link {
                                            to: format!("/article/{}", prev),
                                            class: "btn btn-sm btn-outline flex-1",
                                            "← Prev"
                                        }
                                    }
                                    if let Some(ref next) = series.next {
                                        Link {
                                            to: format!("/article/{}", next),
                                            class: "btn btn-sm btn-primary flex-1",
                                            "Next →"
                                        }
                                    }
                                }
                            }
                        } else {
                            if let Some(ref prev) = meta.prev_article {
                                Link {
                                    to: format!("/article/{}", prev),
                                    class: "btn btn-sm btn-outline flex-1",
                                    "← Prev"
                                }
                            }
                            if let Some(ref next) = meta.next_article {
                                Link {
                                    to: format!("/article/{}", next),
                                    class: "btn btn-sm btn-primary flex-1",
                                    "Next →"
                                }
                            }
                        }
                    }
                }
            }

            // Tabs section
            div {
                class: "flex-1 p-4",
                h3 {
                    class: "text-sm font-semibold mb-3 opacity-70",
                    "Sections"
                }

                div {
                    class: "flex flex-col gap-2",

                    // Article tab
                    button {
                        class: if active_tab.read().as_str() == "article" {
                            "btn btn-sm btn-primary justify-start"
                        } else {
                            "btn btn-sm btn-ghost justify-start"
                        },
                        onclick: move |_| active_tab.set("article".to_string()),
                        "📄 Article"
                    }

                    // References tab
                    if metadata.as_ref().map(|m| m.show_references).unwrap_or(true) {
                        button {
                            class: if active_tab.read().as_str() == "references" {
                                "btn btn-sm btn-primary justify-start"
                            } else {
                                "btn btn-sm btn-ghost justify-start"
                            },
                            onclick: move |_| active_tab.set("references".to_string()),
                            "📚 References"
                        }
                    }

                    // Demo tab
                    if metadata.as_ref().map(|m| m.show_demo).unwrap_or(false) {
                        button {
                            class: if active_tab.read().as_str() == "demo" {
                                "btn btn-sm btn-primary justify-start"
                            } else {
                                "btn btn-sm btn-ghost justify-start"
                            },
                            onclick: move |_| active_tab.set("demo".to_string()),
                            "▶️ Demo"
                        }
                    }

                    // Related tab
                    if metadata.as_ref().map(|m| m.show_related).unwrap_or(false) {
                        button {
                            class: if active_tab.read().as_str() == "related" {
                                "btn btn-sm btn-primary justify-start"
                            } else {
                                "btn btn-sm btn-ghost justify-start"
                            },
                            onclick: move |_| active_tab.set("related".to_string()),
                            "⚡ Related"
                        }
                    }

                    // Quiz tab
                    if metadata.as_ref().map(|m| m.show_quiz).unwrap_or(false) {
                        button {
                            class: if active_tab.read().as_str() == "quiz" {
                                "btn btn-sm btn-primary justify-start"
                            } else {
                                "btn btn-sm btn-ghost justify-start"
                            },
                            onclick: move |_| active_tab.set("quiz".to_string()),
                            "✓ Quiz"
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

                // Series name badge if using new structure
                if let Some(ref meta) = metadata {
                    if !meta.article_series.is_empty() {
                        span {
                            class: "badge badge-xs badge-outline opacity-60",
                            "{meta.article_series[0].name}"
                        }
                    }
                }

                div {
                    class: "flex gap-2",
                    if let Some(ref meta) = metadata {
                        // Use new article_series structure
                        if !meta.article_series.is_empty() {
                            {
                                let series = &meta.article_series[0];
                                rsx! {
                                    if let Some(ref prev) = series.prev {
                                        Link {
                                            to: format!("/article/{}", prev),
                                            class: "btn btn-xs btn-ghost",
                                            "← Prev"
                                        }
                                    }
                                    if let Some(ref next) = series.next {
                                        Link {
                                            to: format!("/article/{}", next),
                                            class: "btn btn-xs btn-ghost",
                                            "Next →"
                                        }
                                    }
                                }
                            }
                        } else {
                            // Fall back to legacy fields
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
                        }
                    } else {
                        span {
                            class: "text-xs opacity-50",
                            "No navigation"
                        }
                    }
                }
            }

            // References Tab (Conditional)
            if metadata.as_ref().map(|m| m.show_references).unwrap_or(true) {
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
            }

            // Demo Tab (Conditional)
            if metadata.as_ref().map(|m| m.show_demo).unwrap_or(false) {
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
            }

            // Related Articles Tab (Conditional)
            if metadata.as_ref().map(|m| m.show_related).unwrap_or(false) {
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
            }

            // Quiz Tab (Conditional)
            if metadata.as_ref().map(|m| m.show_quiz).unwrap_or(false) {
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
}

#[component]
fn NavigationCards(metadata: ArticleTomlMetadata) -> Element {
    // Determine prev and next from article_series or legacy fields
    let (prev_path, next_path, series_name) = if !metadata.article_series.is_empty() {
        let series = &metadata.article_series[0];
        (
            series.prev.clone(),
            series.next.clone(),
            Some(series.name.clone()),
        )
    } else {
        (
            metadata.prev_article.clone(),
            metadata.next_article.clone(),
            metadata.primary_series.clone(),
        )
    };

    // Only render if there's at least one navigation link
    if prev_path.is_none() && next_path.is_none() {
        return rsx! {};
    }

    rsx! {
        div {
            class: "border-t border-base-300 pt-8 mt-8",

            // Series name badge if available
            if let Some(ref series) = series_name {
                div {
                    class: "mb-4",
                    span {
                        class: "badge badge-primary badge-lg",
                        "{series}"
                    }
                }
            }

            div {
                class: "grid grid-cols-1 md:grid-cols-2 gap-4",

                // Previous article card
                if let Some(ref prev) = prev_path {
                    Link {
                        to: format!("/article/{}", prev),
                        class: "card card-sm bg-base-200 hover:bg-base-300 transition-colors",
                        div {
                            class: "card-body",
                            div {
                                class: "flex items-center gap-2 text-sm opacity-70 mb-2",
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
                                span { "Previous" }
                            }
                            h3 {
                                class: "card-title text-base",
                                {prev.split('/').last().unwrap_or(prev).replace("-", " ")}
                            }
                        }
                    }
                } else {
                    // Empty placeholder for grid alignment
                    div {}
                }

                // Next article card
                if let Some(ref next) = next_path {
                    Link {
                        to: format!("/article/{}", next),
                        class: "card card-sm bg-base-200 hover:bg-base-300 transition-colors",
                        div {
                            class: "card-body",
                            div {
                                class: "flex items-center justify-end gap-2 text-sm opacity-70 mb-2",
                                span { "Next" }
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
                            h3 {
                                class: "card-title text-base text-right",
                                {next.split('/').last().unwrap_or(next).replace("-", " ")}
                            }
                        }
                    }
                }
            }
        }
    }
}
