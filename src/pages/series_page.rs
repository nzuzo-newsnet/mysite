use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
struct Series {
    name: String,
    description: String,
    category: SeriesCategory,
    total_parts: u32,
    articles: Vec<String>,
}

#[derive(Clone, PartialEq)]
enum SeriesCategory {
    Projects,
    Tutorials,
    Concepts,
    Research,
}

impl SeriesCategory {
    fn as_str(&self) -> &str {
        match self {
            SeriesCategory::Projects => "Projects",
            SeriesCategory::Tutorials => "Tutorials",
            SeriesCategory::Concepts => "Concepts",
            SeriesCategory::Research => "Research",
        }
    }

    fn badge_class(&self) -> &str {
        match self {
            SeriesCategory::Projects => "badge-primary",
            SeriesCategory::Tutorials => "badge-secondary",
            SeriesCategory::Concepts => "badge-accent",
            SeriesCategory::Research => "badge-info",
        }
    }

    fn icon(&self) -> &str {
        match self {
            SeriesCategory::Projects => "🚀",
            SeriesCategory::Tutorials => "📚",
            SeriesCategory::Concepts => "💡",
            SeriesCategory::Research => "🔬",
        }
    }
}

#[component]
pub fn SeriesPage() -> Element {
    // Sample series - replace with actual data from your articles
    let series_list = vec![
        Series {
            name: "Building a News Aggregator".to_string(),
            description: "A comprehensive guide to building a scalable news aggregation platform with Rust and modern web technologies.".to_string(),
            category: SeriesCategory::Projects,
            total_parts: 5,
            articles: vec![
                "news-aggregator-part-1".to_string(),
                "news-aggregator-part-2".to_string(),
            ],
        },
        Series {
            name: "Rust for Python Developers".to_string(),
            description: "Learn Rust coming from a Python background, with practical examples and comparisons.".to_string(),
            category: SeriesCategory::Tutorials,
            total_parts: 8,
            articles: vec![
                "rust-python-part-1".to_string(),
                "rust-python-part-2".to_string(),
            ],
        },
        Series {
            name: "Data Pipeline Architecture".to_string(),
            description: "Design and implementation patterns for robust data processing pipelines.".to_string(),
            category: SeriesCategory::Projects,
            total_parts: 4,
            articles: vec![
                "data-pipeline-part-1".to_string(),
            ],
        },
        Series {
            name: "Machine Learning Fundamentals".to_string(),
            description: "Core concepts and mathematical foundations of machine learning algorithms.".to_string(),
            category: SeriesCategory::Concepts,
            total_parts: 12,
            articles: vec![
                "ml-fundamentals-part-1".to_string(),
                "ml-fundamentals-part-2".to_string(),
                "ml-fundamentals-part-3".to_string(),
            ],
        },
        Series {
            name: "Web Performance Optimization".to_string(),
            description: "Techniques and strategies for building fast, efficient web applications.".to_string(),
            category: SeriesCategory::Tutorials,
            total_parts: 6,
            articles: vec![
                "web-perf-part-1".to_string(),
            ],
        },
    ];

    let mut selected_category = use_signal(|| None::<SeriesCategory>);

    let filtered_series: Vec<Series> = if let Some(ref category) = *selected_category.read() {
        series_list.iter()
            .filter(|s| s.category == *category)
            .cloned()
            .collect()
    } else {
        series_list.clone()
    };

    rsx! {
        main {
            class: "flex-1 overflow-y-auto p-8",
            div {
                class: "container mx-auto max-w-7xl",

            // Header
            div {
                class: "mb-8",
                h1 {
                    class: "text-5xl font-bold mb-4",
                    "Article Series"
                }
                p {
                    class: "text-lg text-base-content opacity-70",
                    "Explore multi-part articles organized by topic and category"
                }
            }

            // Category filters
            div {
                class: "flex gap-3 mb-8 flex-wrap",
                button {
                    class: if selected_category.read().is_none() { "btn btn-primary" } else { "btn btn-ghost" },
                    onclick: move |_| selected_category.set(None),
                    "All Series"
                }
                button {
                    class: if matches!(*selected_category.read(), Some(SeriesCategory::Projects)) { "btn btn-primary" } else { "btn btn-ghost" },
                    onclick: move |_| selected_category.set(Some(SeriesCategory::Projects)),
                    "{SeriesCategory::Projects.icon()} Projects"
                }
                button {
                    class: if matches!(*selected_category.read(), Some(SeriesCategory::Tutorials)) { "btn btn-primary" } else { "btn btn-ghost" },
                    onclick: move |_| selected_category.set(Some(SeriesCategory::Tutorials)),
                    "{SeriesCategory::Tutorials.icon()} Tutorials"
                }
                button {
                    class: if matches!(*selected_category.read(), Some(SeriesCategory::Concepts)) { "btn btn-primary" } else { "btn btn-ghost" },
                    onclick: move |_| selected_category.set(Some(SeriesCategory::Concepts)),
                    "{SeriesCategory::Concepts.icon()} Concepts"
                }
                button {
                    class: if matches!(*selected_category.read(), Some(SeriesCategory::Research)) { "btn btn-primary" } else { "btn btn-ghost" },
                    onclick: move |_| selected_category.set(Some(SeriesCategory::Research)),
                    "{SeriesCategory::Research.icon()} Research"
                }
            }

            // Series grid
            div {
                class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6",
                for series in filtered_series {
                    SeriesCard { series: series.clone() }
                }
            }
            }
        }
    }
}

#[component]
fn SeriesCard(series: Series) -> Element {
    let progress = (series.articles.len() as f32 / series.total_parts as f32 * 100.0) as u32;
    let is_complete = series.articles.len() as u32 == series.total_parts;

    rsx! {
        article {
            class: "card card-lg bg-base-200 hover:shadow-xl transition-all duration-300 border-2 border-transparent hover:border-primary",
            div {
                class: "card-body",

                // Header with category and icon
                div {
                    class: "flex justify-between items-start mb-3",
                    div {
                        class: "flex items-center gap-2",
                        span {
                            class: "text-2xl",
                            "{series.category.icon()}"
                        }
                        h3 {
                            class: "card-title text-xl",
                            "{series.name}"
                        }
                    }
                    span {
                        class: "badge {series.category.badge_class()}",
                        "{series.category.as_str()}"
                    }
                }

                // Description
                p {
                    class: "text-base-content opacity-70 mb-4 flex-grow",
                    "{series.description}"
                }

                // Progress bar
                div {
                    class: "mb-4",
                    div {
                        class: "flex justify-between text-sm mb-2",
                        span {
                            class: "text-base-content opacity-60",
                            "{series.articles.len()} of {series.total_parts} parts"
                        }
                        span {
                            class: if is_complete { "text-success font-semibold" } else { "text-base-content opacity-60" },
                            if is_complete {
                                "Complete ✓"
                            } else {
                                "{progress}%"
                            }
                        }
                    }
                    progress {
                        class: if is_complete { "progress progress-success w-full" } else { "progress progress-primary w-full" },
                        value: "{progress}",
                        max: "100",
                    }
                }

                // Article links
                if !series.articles.is_empty() {
                    div {
                        class: "space-y-2",
                        for (idx, article) in series.articles.iter().enumerate() {
                            Link {
                                to: format!("/article/{}", article),
                                class: "btn btn-sm btn-ghost w-full justify-start gap-2",
                                span {
                                    class: "badge badge-outline badge-sm",
                                    "Part {idx + 1}"
                                }
                                span {
                                    class: "truncate",
                                    "{article}"
                                }
                            }
                        }
                    }
                }

                // View Series button
                div {
                    class: "card-actions justify-end mt-4",
                    button {
                        class: "btn btn-primary btn-sm",
                        "View Series →"
                    }
                }
            }
        }
    }
}
