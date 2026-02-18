pub mod blog;
pub mod projects;
use crate::pages::home_page::{blog::Blogs, projects::Projects};
use dioxus::prelude::*;

#[component]
pub fn HomePage() -> Element {
    rsx! {
        main {
            class: "flex-1 w-full overflow-y-auto",
            
            div {
                class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8",
                
                // Hero Section
                Hero {}

                // Main Content + Sidebar
                div {
                    class: "flex flex-col lg:flex-row gap-8",
                    
                    // Main Content Area (Blogs)
                    div {
                        class: "flex-1 min-w-0",
                        Blogs {}
                    }

                    // Sidebar Area (Projects)
                    div {
                        class: "lg:w-80 flex-shrink-0",
                        div {
                            class: "lg:sticky lg:top-24",
                            Projects {}
                        }
                    }
                }
            }

            // Footer
            footer {
                class: "border-t py-8 mt-12 bg-base-100 border-base-300 text-base-content/60",
                div {
                    class: "max-w-7xl mx-auto px-4 flex flex-col md:flex-row justify-between items-center gap-4",
                    div {
                        class: "text-sm",
                        "© 2026 Nzuzo Magagula. Built with Rust & Dioxus."
                    }
                    div {
                        class: "flex gap-6 text-sm font-medium",
                        a { href: "#", class: "hover:text-primary", "RSS" }
                        a { href: "https://github.com/nzuzo-newsnet", class: "hover:text-primary", "Github" }
                        a { href: "#", class: "hover:text-primary", "Source" }
                    }
                }
            }
        }
    }
}

#[component]
fn Hero() -> Element {
    rsx! {
        div {
            class: "mb-12 py-8 md:py-12 border-b border-base-300 min-h-[60dvh] flex items-center",
            div {
                class: "flex flex-col md:flex-row items-center justify-between w-full gap-8 md:gap-12",
                
                // Text Content
                div {
                    class: "flex-1 max-w-2xl",
                    h1 {
                        class: "text-4xl md:text-6xl font-extrabold mb-6 bg-clip-text text-transparent bg-gradient-to-r from-primary to-secondary",
                        "Building high-performance distributed systems in Rust."
                    }
                    p {
                        class: "text-lg md:text-2xl mb-8 text-base-content/80 font-medium leading-relaxed",
                        "I'm an engineer focused on database internals, consensus algorithms, and the future of internet infrastructure."
                    }
                    div {
                        class: "flex flex-wrap gap-4",
                        Link {
                            to: "/articles",
                            class: "inline-flex items-center px-6 py-3 border border-transparent text-base font-bold rounded-xl shadow-lg text-primary-content bg-primary hover:bg-primary-focus transition-all hover:scale-105",
                            "View Articles"
                        }
                        a {
                            href: "https://github.com/nzuzo-newsnet",
                            class: "inline-flex items-center px-6 py-3 border-2 border-base-300 text-base font-bold rounded-xl text-base-content hover:bg-base-200 transition-all hover:scale-105",
                            svg { class: "mr-2 h-5 w-5", fill: "currentColor", view_box: "0 0 24 24", xmlns: "http://www.w3.org/2000/svg",
                                path { d: "M12 .297c-6.63 0-12 5.373-12 12 0 5.303 3.438 9.8 8.205 11.385.6.113.82-.258.82-.577 0-.285-.01-1.04-.015-2.04-3.338.724-4.042-1.61-4.042-1.61C4.422 18.07 3.633 17.7 3.633 17.7c-1.087-.744.084-.729.084-.729 1.205.084 1.838 1.236 1.838 1.236 1.07 1.835 2.809 1.305 3.495.998.108-.776.417-1.305.76-1.605-2.665-.3-5.466-1.332-5.466-5.93 0-1.31.465-2.38 1.235-3.22-.135-.303-.54-1.523.105-3.176 0 0 1.005-.322 3.3 1.23.96-.267 1.98-.399 3-.405 1.02.006 2.04.138 3 .405 2.28-1.552 3.285-1.23 3.285-1.23.645 1.653.24 2.873.12 3.176.765.84 1.23 1.91 1.23 3.22 0 4.61-2.805 5.625-5.475 5.92.42.36.81 1.096.81 2.22 0 1.606-.015 2.896-.015 3.286 0 .315.21.69.825.57C20.565 22.092 24 17.592 24 12.297c0-6.627-5.373-12-12-12" }
                            }
                            "Github"
                        }
                    }
                }

                // Square Image Anchor (Linked to About)
                Link {
                    to: "/about",
                    class: "w-56 h-56 md:w-80 md:h-80 flex-shrink-0 relative group",
                    div {
                        class: "absolute inset-0 bg-primary/30 rounded-3xl blur-2xl group-hover:blur-3xl transition-all duration-500 opacity-0 group-hover:opacity-100",
                    }
                    div {
                        class: "relative w-full h-full rounded-3xl overflow-hidden border-4 border-base-300 shadow-2xl transition-transform duration-500 group-hover:scale-105",
                        img {
                            src: "/main_image.jpg",
                            class: "w-full h-full object-cover transition-transform duration-700 group-hover:scale-110",
                            alt: "Nzuzo Magagula"
                        }
                    }
                }
            }
        }
    }
}



