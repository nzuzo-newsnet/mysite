use crate::shared::nav_bar::NavBar;
use dioxus::{
    document::eval,
    logger::{self, tracing::Level},
    prelude::*,
};

pub mod markdown_management;
pub mod pages;
pub mod shared;

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/article/:..segments")]
    Article { segments: Vec<String> },
    #[route("/about")]
    About {},
    #[route("/demos")]
    Demos {},
    #[route("/reading")]
    Reading {},
    #[route("/series")]
    Series {},
    #[route("/series/:name")]
    SeriesDetail { name: String },
    #[route("/articles")]
    Articles {},
    #[route("/demos/algovis")]
    AlgoVis {},
}

fn main() {
    // Initialize article watcher on server startup
    #[cfg(feature = "server")]
    {
        if let Err(e) = markdown_management::start_article_watcher() {
            logger::tracing::error!("Failed to start article watcher: {}", e);
        }
    }

    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let _ = logger::init(Level::INFO);

    // Set initial theme
    use_effect(move || {
        eval(r#"
            if (!document.documentElement.hasAttribute('data-theme')) {
                document.documentElement.setAttribute('data-theme', 'light');
            }
        "#);
    });

    rsx! {
        document::Link {
            rel: "stylesheet",
            href: TAILWIND_CSS,
        }
        document::Meta {
            name: "viewport",
            content: "width=device-width, initial-scale=1.0, maximum-scale=5.0",
        }
        
        // Background for Glassmorphism
        div {
            class: "fixed inset-0 z-[-1] pointer-events-none overflow-hidden bg-base-300",
            img {
                src: "/main_image.jpg",
                class: "w-full h-full object-cover blur-[100px] opacity-50 scale-110",
                alt: "Background"
            }
        }

        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        div {
            class: "min-h-dvh flex flex-col",
            NavBar {}
            pages::home_page::HomePage {}
        }
    }
}

#[component]
fn Article(segments: Vec<String>) -> Element {
    // Join segments to form the path (e.g., ["data-engineering", "01-pipeline-basics"] -> "data-engineering/01-pipeline-basics")
    let path = segments.join("/");

    // Log the route segments - this should print EVERY time the route changes
    logger::tracing::info!("Article component rendered with segments: {:?}, path: {}", segments, path);

    let full_path = format!("{}.md", path);

    // Log the key being used
    logger::tracing::info!("Using key for ArticlePage: {}", full_path);

    rsx! {
        div {
            class: "h-dvh flex flex-col overflow-hidden",
            NavBar {}
            // Use key to force component remount on path change
            // This ensures all hooks re-initialize with the new path
            pages::article_page::ArticlePage {
                key: full_path.clone(),
                path: full_path
            }
        }
    }
}

#[component]
fn About() -> Element {
    rsx! {
        div {
            class: "h-dvh flex flex-col overflow-hidden",
            NavBar {}
            pages::about_page::AboutPage {}
        }
    }
}

#[component]
fn Demos() -> Element {
    rsx! {
        div {
            class: "h-dvh flex flex-col overflow-hidden",
            NavBar {}
            pages::demos_page::DemosPage {}
        }
    }
}

#[component]
fn Reading() -> Element {
    rsx! {
        div {
            class: "h-dvh flex flex-col overflow-hidden",
            NavBar {}
            pages::reading_page::ReadingPage {}
        }
    }
}

#[component]
fn Series() -> Element {
    rsx! {
        div {
            class: "h-dvh flex flex-col overflow-hidden",
            NavBar {}
            pages::series_page::SeriesPage {}
        }
    }
}

#[component]
fn SeriesDetail(name: String) -> Element {
    rsx! {
        div {
            class: "h-dvh flex flex-col overflow-hidden",
            NavBar {}
            pages::series_detail_page::SeriesDetailPage {
                series_name: name
            }
        }
    }
}

#[component]
fn Articles() -> Element {
    rsx! {
        div {
            class: "h-dvh flex flex-col overflow-hidden",
            NavBar {}
            pages::articles_page::ArticlesPage {}
        }
    }
}

#[component]
fn AlgoVis() -> Element {
    rsx! {
        div {
            class: "h-dvh flex flex-col overflow-hidden",
            NavBar {}
            pages::algo_vis_page::AlgoVisPage {}
        }
    }
}
