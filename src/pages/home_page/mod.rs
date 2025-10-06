pub mod blog;
pub mod projects;
use crate::pages::home_page::{blog::Blogs, projects::Projects};
use dioxus::prelude::*;
#[component]
pub fn HomePage() -> Element {
    rsx! {
        main {
            class: "max-w-dvw max-h-dvh p-20",
            div {
                class: "flex flex-row gap-50",
                Projects {

                }
                Blogs {

                }
            }
        }
    }
}
