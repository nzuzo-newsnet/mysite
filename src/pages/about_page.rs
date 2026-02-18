use dioxus::prelude::*;
use dioxus_markdown::Markdown;

const ABOUT_ME_CONTENT: &str = include_str!("../../aboutme.md");

#[component]
pub fn AboutPage() -> Element {
    rsx! {
        main {
            class: "flex-1 overflow-y-auto",
            div {
                class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12",
                
                div {
                    class: "max-w-3xl mx-auto",
                    h1 {
                        class: "text-4xl font-extrabold mb-8 bg-clip-text text-transparent bg-gradient-to-r from-primary to-secondary",
                        "About Me"
                    }
                    
                    article {
                        class: "rounded-2xl border border-base-300 bg-base-100 p-8 shadow-sm",
                        section {
                            class: "prose prose-lg max-w-none prose-headings:text-primary",
                            Markdown {
                                content: ABOUT_ME_CONTENT.to_string()
                            }
                        }
                    }
                }
            }
        }
    }
}

