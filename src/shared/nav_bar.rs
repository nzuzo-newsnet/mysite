use dioxus::document::eval;
use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::ld_icons::LdHome};

#[component]
pub fn NavBar() -> Element {
    let mut theme = use_signal(|| "dark".to_string());
    let mut is_loaded = use_signal(|| false);
    let current_route = use_route::<crate::Route>();

    // Load theme from localStorage on mount only
    use_effect(move || {
        if !is_loaded() {
            spawn(async move {
                let load_script = r#"
                    try {
                        const savedTheme = localStorage.getItem('theme') || 'dark';
                        document.documentElement.setAttribute('data-theme', savedTheme);
                        dioxus.send(savedTheme);
                    } catch (e) {
                        console.error('Failed to load theme:', e);
                        dioxus.send('dark');
                    }
                "#;

                if let Ok(saved_theme) = eval(load_script).recv::<String>().await {
                    theme.set(saved_theme.clone());
                    is_loaded.set(true);
                    dioxus::logger::tracing::info!("Loaded theme: {}", saved_theme);
                }
            });
        }
    });

    // Apply theme changes (but not on initial load)
    use_effect(move || {
        if is_loaded() {
            let current_theme = theme.read().clone();

            let _ = eval(&format!(
                r#"
                try {{
                    localStorage.setItem('theme', '{}');
                    document.documentElement.setAttribute('data-theme', '{}');
                }} catch (e) {{
                    console.error('Failed to save theme:', e);
                }}
                "#,
                current_theme, current_theme
            ));

            dioxus::logger::tracing::info!("Theme changed to: {}", current_theme);
        }
    });

    let available_themes = vec![
        ("light", "☀️ Light"),
        ("dark", "🌙 Dark"),
        ("cupcake", "🧁 Cupcake"),
        ("bumblebee", "🐝 Bumblebee"),
        ("emerald", "💚 Emerald"),
        ("corporate", "💼 Corporate"),
        ("synthwave", "🌆 Synthwave"),
        ("retro", "📺 Retro"),
        ("cyberpunk", "🤖 Cyberpunk"),
        ("valentine", "💝 Valentine"),
        ("halloween", "🎃 Halloween"),
        ("garden", "🌻 Garden"),
        ("forest", "🌲 Forest"),
        ("aqua", "🌊 Aqua"),
        ("lofi", "🎵 Lo-Fi"),
        ("pastel", "🎨 Pastel"),
        ("fantasy", "🦄 Fantasy"),
        ("wireframe", "📐 Wireframe"),
        ("black", "⬜ Black"),
        ("luxury", "💎 Luxury"),
        ("dracula", "🧛 Dracula"),
        ("cmyk", "🖨️ CMYK"),
        ("autumn", "🍂 Autumn"),
        ("business", "📊 Business"),
        ("acid", "🧪 Acid"),
        ("lemonade", "🍋 Lemonade"),
        ("night", "🌃 Night"),
        ("coffee", "☕ Coffee"),
        ("winter", "❄️ Winter"),
        ("dim", "🔅 Dim"),
        ("nord", "🏔️ Nord"),
        ("sunset", "🌅 Sunset"),
    ];

    // Helper function to check if route is active
    let is_route_active = |route: &crate::Route| {
        match (&current_route, route) {
            (crate::Route::Home {}, crate::Route::Home {}) => true,
            (crate::Route::About {}, crate::Route::About {}) => true,
            (crate::Route::Demos {}, crate::Route::Demos {}) => true,
            (crate::Route::Reading {}, crate::Route::Reading {}) => true,
            (crate::Route::Series {}, crate::Route::Series {}) => true,
            // SeriesDetail should also highlight the Series nav item
            (crate::Route::SeriesDetail { .. }, crate::Route::Series {}) => true,
            (crate::Route::Articles {}, crate::Route::Articles {}) => true,
            _ => false,
        }
    };

    rsx! {
        nav {
            class: "flex-shrink-0 w-full flex flex-col md:flex-row justify-between items-center p-3 md:p-5 gap-2 md:gap-4 bg-base-100 border-b border-base-300",

            // Logo/Name
            Link {
                to: "/",
                class: "text-xl sm:text-2xl md:text-3xl font-bold hover:opacity-80 transition-opacity",
                "Nzuzo Magagula"
            }

            // Navigation Links
            div {
                class: "flex flex-wrap gap-2 md:gap-4 items-center justify-center",

                // Internal pages
                Link {
                    to: "/",
                    class: if is_route_active(&crate::Route::Home {}) {
                        "btn btn-ghost btn-active bg-primary text-primary-content flex items-center gap-2"
                    } else {
                        "btn btn-ghost flex items-center gap-2"
                    },
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
                    class: if is_route_active(&crate::Route::About {}) {
                        "btn btn-ghost btn-active bg-primary text-primary-content"
                    } else {
                        "btn btn-ghost"
                    },
                    "About"
                }

                Link {
                    to: "/demos",
                    class: if is_route_active(&crate::Route::Demos {}) {
                        "btn btn-ghost btn-active bg-primary text-primary-content"
                    } else {
                        "btn btn-ghost"
                    },
                    "Demos"
                }

                Link {
                    to: "/reading",
                    class: if is_route_active(&crate::Route::Reading {}) {
                        "btn btn-ghost btn-active bg-primary text-primary-content"
                    } else {
                        "btn btn-ghost"
                    },
                    "Reading"
                }

                Link {
                    to: "/series",
                    class: if is_route_active(&crate::Route::Series {}) {
                        "btn btn-ghost btn-active bg-primary text-primary-content"
                    } else {
                        "btn btn-ghost"
                    },
                    "Series"
                }

                Link {
                    to: "/articles",
                    class: if is_route_active(&crate::Route::Articles {}) {
                        "btn btn-ghost btn-active bg-primary text-primary-content"
                    } else {
                        "btn btn-ghost"
                    },
                    "Articles"
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
                        class: "dropdown-content menu bg-base-200 rounded-box z-[100] w-52 p-2 shadow-lg border border-base-300 absolute max-h-96 overflow-y-auto",
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
