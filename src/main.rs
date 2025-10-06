use crate::{pages::home_page::HomePage, shared::nav_bar::NavBar};
use dioxus::prelude::*;
use dioxus_primitives::navbar::Navbar;
pub mod markdown_management;
pub mod pages;
pub mod shared;
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    HomePage {},
}
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
fn main() {
    dioxus::launch(App);
}
#[component]
fn App() -> Element {
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: TAILWIND_CSS,
        }
        Router::<Route> {

        }
        NavBar {

        }
        HomePage {

        }
    }
}
