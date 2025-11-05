use dioxus::prelude::*;
use dioxus_markdown::Markdown;

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
