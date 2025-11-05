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
    #[route("/article/:name")]
    Article { name: String },
    #[route("/about")]
    About {},
    #[route("/demos")]
    Demos {},
    #[route("/reading")]
    Reading {},
    #[route("/series")]
    Series {},
}

fn main() {
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
fn Article(name: String) -> Element {
    rsx! {
        div {
            class: "h-dvh flex flex-col overflow-hidden",
            NavBar {}
            pages::article_page::ArticlePage {
                path: format!("{}.md", name)
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
