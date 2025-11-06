use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
struct ReadingItem {
    title: String,
    author: String,
    description: String,
    status: ReadingStatus,
    link: Option<String>,
}

#[derive(Clone, PartialEq)]
enum ReadingStatus {
    CurrentlyReading,
    Read,
    WantToRead,
}

impl ReadingStatus {
    fn as_str(&self) -> &str {
        match self {
            ReadingStatus::CurrentlyReading => "Currently Reading",
            ReadingStatus::Read => "Read",
            ReadingStatus::WantToRead => "Want to Read",
        }
    }

    fn badge_class(&self) -> &str {
        match self {
            ReadingStatus::CurrentlyReading => "badge-primary",
            ReadingStatus::Read => "badge-success",
            ReadingStatus::WantToRead => "badge-ghost",
        }
    }
}

#[component]
pub fn ReadingPage() -> Element {
    // Sample reading list - replace with your actual reading list
    let reading_list = vec![
        ReadingItem {
            title: "The Rust Programming Language".to_string(),
            author: "Steve Klabnik, Carol Nichols".to_string(),
            description: "The official Rust book, covering everything from basics to advanced concepts.".to_string(),
            status: ReadingStatus::Read,
            link: Some("https://doc.rust-lang.org/book/".to_string()),
        },
        ReadingItem {
            title: "Designing Data-Intensive Applications".to_string(),
            author: "Martin Kleppmann".to_string(),
            description: "Essential reading for understanding the architecture of modern data systems.".to_string(),
            status: ReadingStatus::CurrentlyReading,
            link: None,
        },
        ReadingItem {
            title: "Clean Architecture".to_string(),
            author: "Robert C. Martin".to_string(),
            description: "A guide to software structure and design principles.".to_string(),
            status: ReadingStatus::Read,
            link: None,
        },
        ReadingItem {
            title: "The Art of Doing Science and Engineering".to_string(),
            author: "Richard Hamming".to_string(),
            description: "Insights on learning and doing great work in technical fields.".to_string(),
            status: ReadingStatus::WantToRead,
            link: None,
        },
        ReadingItem {
            title: "Release It!".to_string(),
            author: "Michael T. Nygard".to_string(),
            description: "Design and deploy production-ready software.".to_string(),
            status: ReadingStatus::CurrentlyReading,
            link: None,
        },
    ];

    let mut selected_status = use_signal(|| None::<ReadingStatus>);

    let filtered_items: Vec<ReadingItem> = if let Some(ref status) = *selected_status.read() {
        reading_list.iter()
            .filter(|item| item.status == *status)
            .cloned()
            .collect()
    } else {
        reading_list.clone()
    };

    rsx! {
        main {
            class: "flex-1 overflow-y-auto p-8",
            div {
                class: "container mx-auto max-w-5xl",

            // Header
            div {
                class: "mb-8",
                h1 {
                    class: "text-3xl font-bold mb-4",
                    "Reading List"
                }
                p {
                    class: "text-lg text-base-content opacity-70",
                    "Books, papers, and articles I'm reading or have read"
                }
            }

            // Status filters
            div {
                class: "flex gap-3 mb-8 flex-wrap",
                button {
                    class: if selected_status.read().is_none() { "btn btn-primary" } else { "btn btn-ghost" },
                    onclick: move |_| selected_status.set(None),
                    "All"
                }
                button {
                    class: if matches!(*selected_status.read(), Some(ReadingStatus::CurrentlyReading)) { "btn btn-primary" } else { "btn btn-ghost" },
                    onclick: move |_| selected_status.set(Some(ReadingStatus::CurrentlyReading)),
                    "Currently Reading"
                }
                button {
                    class: if matches!(*selected_status.read(), Some(ReadingStatus::Read)) { "btn btn-primary" } else { "btn btn-ghost" },
                    onclick: move |_| selected_status.set(Some(ReadingStatus::Read)),
                    "Read"
                }
                button {
                    class: if matches!(*selected_status.read(), Some(ReadingStatus::WantToRead)) { "btn btn-primary" } else { "btn btn-ghost" },
                    onclick: move |_| selected_status.set(Some(ReadingStatus::WantToRead)),
                    "Want to Read"
                }
            }

            // Reading list
            div {
                class: "space-y-4",
                for item in filtered_items {
                    ReadingCard { item: item.clone() }
                }
            }
            }
        }
    }
}

#[component]
fn ReadingCard(item: ReadingItem) -> Element {
    rsx! {
        article {
            class: "card card-lg bg-base-200 hover:shadow-lg transition-shadow",
            div {
                class: "card-body",
                div {
                    class: "flex justify-between items-start mb-2",
                    div {
                        h3 {
                            class: "card-title text-xl mb-1",
                            if let Some(link) = &item.link {
                                a {
                                    href: "{link}",
                                    target: "_blank",
                                    class: "link link-primary",
                                    "{item.title}"
                                }
                            } else {
                                "{item.title}"
                            }
                        }
                        p {
                            class: "text-sm text-base-content opacity-60",
                            "by {item.author}"
                        }
                    }
                    span {
                        class: "badge {item.status.badge_class()}",
                        "{item.status.as_str()}"
                    }
                }
                p {
                    class: "text-base-content opacity-70",
                    "{item.description}"
                }
            }
        }
    }
}
