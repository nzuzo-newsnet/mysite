use dioxus::prelude::*;
use dioxus_markdown::Markdown;
use markdown::{to_html_with_options, CompileOptions, Options};

use crate::markdown_management::extract_readmes_for_account;
#[component]
pub fn Projects() -> Element {
    let mut sources = use_resource(|| async move {
        extract_readmes_for_account("orgs".to_string(), "newsnet-africa".to_string())
            .await
            .expect("Failed to extract ReadMEs")
    });
    rsx! {
        article {
            class: "card card-md",
            header {

                h2 {
                    class: "text-4xl card-title",
                    "Here's what I'm working on"
                }
            }
            section {
                class: "card-body",
                for (name, content) in sources.cloned().unwrap_or_default() {
                    ProjectItem {
                        source: content
                    }
                }
                button {
                    class: "btn",
                    onclick: move |_| sources.restart(),
                    "Click"
                }
            }
        }
    }
}
#[component]
fn ProjectItem(source: String) -> Element {
    rsx! {
        article {
            class: "card card-md",
            slot {
                class: "prose",
                Markdown {
                    content: source,
                }
            }
        }
    }
}
