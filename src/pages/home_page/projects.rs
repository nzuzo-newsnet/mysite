use dioxus::prelude::*;
use dioxus_markdown::Markdown;
use markdown::{to_html_with_options, CompileOptions, Options};
#[component]
pub fn Projects() -> Element {
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
                for _ in 0..4 {
                    ProjectItem {

                    }
                }
            }
        }
    }
}
#[component]
fn ProjectItem() -> Element {
    let source = r#"# Netabase
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org) [![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)



**Netabase** is a distributed, peer-to-peer database system built on top of [sled](https://github.com/spacejam/sled) with optional [libp2p](https://libp2p.io/) integration.
It provides a type-safe, macro-driven approach to defining database schemas and models with support for primary keys, secondary keys, and relational queries.

The system operates in two modes:
- **Local Mode**: High-performance embedded database for single-node applications
- **Distributed Mode**: P2P networked database with automatic synchronization (requires `libp2p` feature)"#;
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
