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
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        div {
            class: "h-dvh flex flex-col overflow-hidden",
            NavBar {}
            pages::home_page::HomePage {}
        }
    }
}

#[component]
fn Article(segments: Vec<String>) -> Element {
    // Join segments to form the path (e.g., ["data-engineering", "01-pipeline-basics"] -> "data-engineering/01-pipeline-basics")
    let path = segments.join("/");

    // Log the route segments
    logger::tracing::info!("Article route called with segments: {:?}, path: {}", segments, path);

    let full_path = format!("{}.md", path);

    rsx! {
        div {
            class: "h-dvh flex flex-col overflow-hidden",
            NavBar {}
            // Use key to force component recreation when path changes
            // The key must be a unique value that changes with the path
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
