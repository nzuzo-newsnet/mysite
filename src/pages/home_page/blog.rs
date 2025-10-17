use dioxus::prelude::*;
use dioxus_markdown::Markdown;
use markdown::{to_html_with_options, CompileOptions, Options};

use crate::markdown_management::{fetch_article_content, list_files};
#[component]
pub fn Blogs() -> Element {
    let sources = use_resource(|| async move {
        let files = list_files("./articles/".to_string())
            .await
            .expect("Could not find articles");
        let mut out = Vec::new();

        for p in files {
            out.push(
                fetch_article_content(p.to_path_buf())
                    .await
                    .expect("Failed to get article"),
            );
        }
        out
    });
    rsx! {
        article {
            class: "card card-xl basis-6xl grow-5",
            header {

                h2 {
                    class: "text-4xl card-title",
                    "Latest Blog"
                }
            }
            section {
                class: "card-body",
                for s in sources.cloned().unwrap_or_default() {
                    BlogItem {
                        source: s,
                    }
                }
            }
        }
    }
}
#[component]
fn BlogItem(source: String) -> Element {
    rsx! {
        article {
            class: "card card-xl",
            div {
                class: "prose",
                Markdown {
                    content: source,
                }
            }
        }
    }
}
