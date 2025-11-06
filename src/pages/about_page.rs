use dioxus::prelude::*;
use dioxus_markdown::Markdown;
use crate::markdown_management::fetch_about_me;

#[component]
pub fn AboutPage() -> Element {
    // Fetch about me content from server
    let about_data = use_resource(|| async move {
        fetch_about_me().await.ok()
    });

    rsx! {
        main {
            class: "flex-1 overflow-y-auto p-8",
            div {
                class: "container mx-auto max-w-4xl",
                article {
                    class: "card card-xl",
                    section {
                        class: "card-body prose max-w-none",

                        match about_data.read().as_ref() {
                            Some(Some(content)) => rsx! {
                                Markdown {
                                    content: content.clone()
                                }
                            },
                            Some(None) => rsx! {
                                // Fallback content if aboutme.md not found
                                h1 { "About Me" }
                                div {
                                    class: "space-y-6",
                                    section {
                                        h2 { "Hi, I'm Nzuzo Magagula" }
                                        p {
                                            "I'm a software engineer passionate about building impactful solutions. "
                                            "My work focuses on creating tools and systems that make a difference."
                                        }
                                    }
                                    section {
                                        h2 { "What I Do" }
                                        p {
                                            "I work on a variety of projects ranging from web applications to data analytics tools. "
                                            "I'm particularly interested in building systems that help people access and understand information better."
                                        }
                                    }
                                    section {
                                        h2 { "Current Projects" }
                                        ul {
                                            li {
                                                strong { "NewsNet Africa: " }
                                                "A platform for aggregating and analyzing African news content."
                                            }
                                            li {
                                                strong { "Various open-source contributions: " }
                                                "Working on tools and libraries that help developers build better software."
                                            }
                                        }
                                    }
                                    section {
                                        h2 { "Get in Touch" }
                                        p {
                                            "Feel free to reach out if you'd like to collaborate or just chat about technology!"
                                        }
                                    }
                                }
                            },
                            None => rsx! {
                                div {
                                    class: "text-center py-12",
                                    span {
                                        class: "loading loading-spinner loading-lg"
                                    }
                                    p {
                                        class: "mt-4 text-base-content opacity-70",
                                        "Loading..."
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
