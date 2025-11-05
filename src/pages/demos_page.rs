use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
struct Demo {
    title: String,
    description: String,
    category: DemoCategory,
    link: Option<String>,
}

#[derive(Clone, PartialEq)]
enum DemoCategory {
    Languages,
    Design,
    DataAnalytics,
}

impl DemoCategory {
    fn as_str(&self) -> &str {
        match self {
            DemoCategory::Languages => "Languages",
            DemoCategory::Design => "Design",
            DemoCategory::DataAnalytics => "Data Analytics",
        }
    }

    fn badge_class(&self) -> &str {
        match self {
            DemoCategory::Languages => "badge-primary",
            DemoCategory::Design => "badge-secondary",
            DemoCategory::DataAnalytics => "badge-accent",
        }
    }
}

#[component]
pub fn DemosPage() -> Element {
    // Sample demos - replace with your actual demos
    let demos = vec![
        Demo {
            title: "Rust Web Framework".to_string(),
            description: "A high-performance web framework built with Rust, featuring async/await and type safety.".to_string(),
            category: DemoCategory::Languages,
            link: None,
        },
        Demo {
            title: "Python Data Pipeline".to_string(),
            description: "ETL pipeline for processing large datasets using Python, Pandas, and Apache Spark.".to_string(),
            category: DemoCategory::Languages,
            link: None,
        },
        Demo {
            title: "UI Component Library".to_string(),
            description: "A modern component library with accessible, customizable components.".to_string(),
            category: DemoCategory::Design,
            link: None,
        },
        Demo {
            title: "Dashboard Template".to_string(),
            description: "Responsive dashboard template with dark mode support and data visualizations.".to_string(),
            category: DemoCategory::Design,
            link: None,
        },
        Demo {
            title: "News Analytics Dashboard".to_string(),
            description: "Real-time analytics dashboard for tracking news trends across African media.".to_string(),
            category: DemoCategory::DataAnalytics,
            link: None,
        },
        Demo {
            title: "Data Visualization Suite".to_string(),
            description: "Interactive data visualization tools for exploring complex datasets.".to_string(),
            category: DemoCategory::DataAnalytics,
            link: None,
        },
    ];

    let mut selected_category = use_signal(|| None::<DemoCategory>);

    let filtered_demos: Vec<Demo> = if let Some(ref category) = *selected_category.read() {
        demos.iter()
            .filter(|d| d.category == *category)
            .cloned()
            .collect()
    } else {
        demos.clone()
    };

    rsx! {
        main {
            class: "flex-1 overflow-y-auto p-8",
            div {
                class: "container mx-auto max-w-6xl",

            // Header
            div {
                class: "mb-8",
                h1 {
                    class: "text-5xl font-bold mb-4",
                    "Demos & Projects"
                }
                p {
                    class: "text-lg text-base-content opacity-70",
                    "Explore my work across different domains"
                }
            }

            // Category filters
            div {
                class: "flex gap-3 mb-8 flex-wrap",
                button {
                    class: if selected_category.read().is_none() { "btn btn-primary" } else { "btn btn-ghost" },
                    onclick: move |_| selected_category.set(None),
                    "All"
                }
                button {
                    class: if matches!(*selected_category.read(), Some(DemoCategory::Languages)) { "btn btn-primary" } else { "btn btn-ghost" },
                    onclick: move |_| selected_category.set(Some(DemoCategory::Languages)),
                    "Languages"
                }
                button {
                    class: if matches!(*selected_category.read(), Some(DemoCategory::Design)) { "btn btn-primary" } else { "btn btn-ghost" },
                    onclick: move |_| selected_category.set(Some(DemoCategory::Design)),
                    "Design"
                }
                button {
                    class: if matches!(*selected_category.read(), Some(DemoCategory::DataAnalytics)) { "btn btn-primary" } else { "btn btn-ghost" },
                    onclick: move |_| selected_category.set(Some(DemoCategory::DataAnalytics)),
                    "Data Analytics"
                }
            }

            // Demos grid
            div {
                class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6",
                for demo in filtered_demos {
                    DemoCard { demo: demo.clone() }
                }
            }
            }
        }
    }
}

#[component]
fn DemoCard(demo: Demo) -> Element {
    rsx! {
        article {
            class: "card card-lg bg-base-200 hover:shadow-xl transition-shadow",
            div {
                class: "card-body",
                div {
                    class: "flex justify-between items-start mb-3",
                    h3 {
                        class: "card-title text-xl",
                        "{demo.title}"
                    }
                    span {
                        class: "badge {demo.category.badge_class()}",
                        "{demo.category.as_str()}"
                    }
                }
                p {
                    class: "text-base-content opacity-70 mb-4 flex-grow",
                    "{demo.description}"
                }
                if let Some(link) = &demo.link {
                    div {
                        class: "card-actions justify-end",
                        a {
                            href: "{link}",
                            target: "_blank",
                            class: "btn btn-primary btn-sm",
                            "View Demo"
                        }
                    }
                }
            }
        }
    }
}
