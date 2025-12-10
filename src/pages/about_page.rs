use dioxus::prelude::*;
use dioxus_markdown::Markdown;

const ABOUT_ME_CONTENT: &str = include_str!("../../aboutme.md");

#[component]
pub fn AboutPage() -> Element {
    rsx! {
        main {
            class: "flex-1 overflow-y-auto p-8",
            div {
                class: "container mx-auto max-w-4xl",
                article {
                    class: "card card-xl",
                    section {
                        class: "card-body prose max-w-none",
                        Markdown {
                            content: ABOUT_ME_CONTENT.to_string()
                        }
                    }
                }
            }
        }
    }
}
