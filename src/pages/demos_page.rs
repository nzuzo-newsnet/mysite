use dioxus::prelude::*;
use crate::Route;

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
            title: "Algorithm Visualizer".to_string(),
            description: "An interactive React-based tool visualizing sorting and pathfinding algorithms, integrated via iframe to demonstrate React expertise.".to_string(),
            category: DemoCategory::Design,
            link: Some("/demos/algovis".to_string()),
        },
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
            class: "flex-1 overflow-y-auto",
            div {
                class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12",

            // Header
            div {
                class: "mb-12 max-w-3xl",
                h1 {
                    class: "text-4xl font-extrabold mb-4 bg-clip-text text-transparent bg-gradient-to-r from-primary to-secondary",
                    "Demos & Projects"
                }
                p {
                    class: "text-lg text-base-content/70",
                    "Explore my work across different domains, from distributed systems to interactive UI experiments."
                }
            }

            // Category filters
            div {
                class: "flex gap-2 mb-10 overflow-x-auto pb-2 scrollbar-hide",
                button {
                    class: if selected_category.read().is_none() { 
                        "px-4 py-2 rounded-full bg-primary text-primary-content font-medium transition-colors"
                    } else { 
                        "px-4 py-2 rounded-full bg-base-200 text-base-content/70 hover:bg-base-300 font-medium transition-colors"
                    },
                    onclick: move |_| selected_category.set(None),
                    "All"
                }
                for category in [DemoCategory::Languages, DemoCategory::Design, DemoCategory::DataAnalytics] {
                    button {
                        class: if selected_category.read().as_ref() == Some(&category) { 
                            "px-4 py-2 rounded-full bg-primary text-primary-content font-medium transition-colors"
                        } else { 
                            "px-4 py-2 rounded-full bg-base-200 text-base-content/70 hover:bg-base-300 font-medium transition-colors"
                        },
                        onclick: {
                            let category = category.clone();
                            move |_| selected_category.set(Some(category.clone()))
                        },
                        "{category.as_str()}"
                    }
                }
            }

            // Demos grid
            div {
                class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8",
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
            class: "flex flex-col rounded-2xl border border-base-300 bg-base-100 overflow-hidden shadow-sm transition-all hover:shadow-xl hover:border-primary/30 group",
            
            // Card visual (placeholder)
            div {
                class: "h-48 bg-gradient-to-br from-base-200 to-base-300 relative flex items-center justify-center overflow-hidden",
                div {
                    class: "absolute inset-0 opacity-10 group-hover:scale-110 transition-transform duration-500",
                    svg { class: "w-full h-full", fill: "currentColor", view_box: "0 0 24 24", xmlns: "http://www.w3.org/2000/svg",
                        path { d: "M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5" }
                    }
                }
                span {
                    class: "badge {demo.category.badge_class()} absolute top-4 right-4 shadow-sm",
                    "{demo.category.as_str()}"
                }
            }

            div {
                class: "p-6 flex flex-col flex-1",
                h3 {
                    class: "text-xl font-bold mb-3 group-hover:text-primary transition-colors",
                    "{demo.title}"
                }
                p {
                    class: "text-base-content/60 text-sm mb-6 flex-grow",
                    "{demo.description}"
                }
                
                if let Some(link) = &demo.link {
                    div {
                        class: "mt-auto pt-4 border-t border-base-200",
                        if link == "/demos/algovis" {
                            Link {
                                to: Route::AlgoVis {},
                                class: "inline-flex items-center text-primary font-bold text-sm group/link",
                                "View Demo"
                                svg { class: "ml-1 w-4 h-4 group-hover/link:translate-x-1 transition-transform", fill: "none", stroke: "currentColor", view_box: "0 0 24 24", xmlns: "http://www.w3.org/2000/svg",
                                    path { stroke_linecap: "round", stroke_linejoin: "round", stroke_width: "2", d: "M9 5l7 7-7 7" }
                                }
                            }
                        } else {
                             a {
                                href: "{link}",
                                target: if link.starts_with('/') { "" } else { "_blank" },
                                class: "inline-flex items-center text-primary font-bold text-sm group/link",
                                "View Project"
                                svg { class: "ml-1 w-4 h-4 group-hover/link:translate-x-1 transition-transform", fill: "none", stroke: "currentColor", view_box: "0 0 24 24", xmlns: "http://www.w3.org/2000/svg",
                                    path { stroke_linecap: "round", stroke_linejoin: "round", stroke_width: "2", d: "M9 5l7 7-7 7" }
                                }
                            }
                        }
                    }
                } else {
                    div {
                        class: "mt-auto pt-4 border-t border-base-200 flex justify-between items-center",
                        span { class: "text-xs font-medium text-base-content/40", "Case Study Coming Soon" }
                        div { class: "w-2 h-2 rounded-full bg-base-300" }
                    }
                }
            }
        }
    }
}
