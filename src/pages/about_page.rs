use dioxus::prelude::*;

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
                }
            }
            }
        }
    }
}
