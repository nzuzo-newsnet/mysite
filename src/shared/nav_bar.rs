use dioxus::prelude::*;
use dioxus::document::eval;
use dioxus_free_icons::{icons::ld_icons::LdHome, Icon};

#[component]
pub fn NavBar() -> Element {
    let mut theme = use_signal(|| "light".to_string());

    // Update DOM when theme changes
    use_effect(move || {
        let current_theme = theme.read().clone();
        let _ = eval(&format!(
            r#"document.documentElement.setAttribute('data-theme', '{}');"#,
            current_theme
        ));
    });

    let available_themes = vec![
        ("light", "☀️ Light"),
        ("dark", "🌙 Dark"),
        ("cupcake", "🧁 Cupcake"),
        ("dracula", "🧛 Dracula"),
        ("cyberpunk", "🤖 Cyberpunk"),
        ("synthwave", "🌆 Synthwave"),
    ];

    rsx! {
        nav {
            class: "flex-shrink-0 w-full flex flex-col md:flex-row justify-between items-center p-3 md:p-5 gap-2 md:gap-4 bg-base-100 border-b border-base-300",

            // Logo/Name
            Link {
                to: "/",
                class: "text-2xl sm:text-3xl md:text-4xl lg:text-5xl font-bold hover:opacity-80 transition-opacity",
                "Nzuzo Magagula"
            }

            // Navigation Links
            div {
                class: "flex flex-wrap gap-2 md:gap-4 items-center justify-center",

                // Internal pages
                Link {
                    to: "/",
                    class: "btn btn-ghost flex items-center gap-2",
                    Icon {
                        height: 20,
                        width: 20,
                        fill: "currentColor",
                        icon: LdHome,
                    }
                    "Home"
                }

                Link {
                    to: "/about",
                    class: "btn btn-ghost",
                    "About"
                }

                Link {
                    to: "/demos",
                    class: "btn btn-ghost",
                    "Demos"
                }

                Link {
                    to: "/reading",
                    class: "btn btn-ghost",
                    "Reading"
                }

                Link {
                    to: "/series",
                    class: "btn btn-ghost",
                    "Series"
                }

                // Divider
                div {
                    class: "hidden md:block w-px h-6 bg-base-300"
                }

                // External GitHub links
                a {
                    href: "https://github.com/nzuzo-newsnet",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    class: "btn btn-ghost text-sm",
                    "nzuzo-newsnet"
                }

                a {
                    href: "https://github.com/newsnet-africa",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    class: "btn btn-ghost text-sm",
                    "newsnet-africa"
                }

                // Divider
                div {
                    class: "hidden md:block w-px h-6 bg-base-300"
                }

                // Theme Dropdown
                details {
                    class: "dropdown dropdown-end",
                    summary {
                        class: "btn btn-ghost gap-2",
                        svg {
                            class: "fill-current w-5 h-5",
                            xmlns: "http://www.w3.org/2000/svg",
                            view_box: "0 0 24 24",
                            path {
                                d: "M7,8H17V10H7Zm0,3h10v2H7Zm0,3h10v2H7Z"
                            }
                        }
                        span {
                            class: "hidden md:inline",
                            "Theme"
                        }
                    }
                    ul {
                        class: "dropdown-content menu bg-base-200 rounded-box z-[100] w-52 p-2 shadow-lg border border-base-300 absolute",
                        for (theme_id, theme_label) in available_themes.iter() {
                            li {
                                key: "{theme_id}",
                                a {
                                    class: if theme.read().as_str() == *theme_id { "active" } else { "" },
                                    onclick: {
                                        let theme_id = theme_id.to_string();
                                        move |_| {
                                            dioxus::logger::tracing::info!("Theme selected: {}", theme_id);
                                            theme.set(theme_id.clone());
                                        }
                                    },
                                    "{theme_label}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
