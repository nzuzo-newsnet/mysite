use std::path::PathBuf;

use dioxus::prelude::*;
use dioxus_markdown::Markdown;

use crate::markdown_management::fetch_article_content;

#[component]
pub fn ProjectPage(source: String) -> Element {
    rsx! {
        slot {
            class: "prose",
            Markdown {
                content: source,
            }
        }
    }
}
