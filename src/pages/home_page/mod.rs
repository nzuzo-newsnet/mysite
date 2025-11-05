pub mod blog;
pub mod projects;
use crate::pages::home_page::{blog::Blogs, projects::Projects};
use dioxus::prelude::*;
#[component]
pub fn HomePage() -> Element {
    rsx! {
        main {
            class: "flex-1 w-full overflow-hidden p-2 sm:p-4 md:p-8",
            div {
                class: "flex flex-col lg:flex-row gap-2 sm:gap-4 md:gap-8 h-full",
                div {
                    class: "flex-[3] h-full overflow-hidden",
                    Blogs {}
                }
                div {
                    class: "flex-[1] h-full overflow-hidden",
                    Projects {}
                }
            }
        }
    }
}
