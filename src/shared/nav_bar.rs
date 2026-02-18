use dioxus::document::eval;
use dioxus::prelude::*;

#[component]
pub fn NavBar() -> Element {
    let mut theme = use_signal(|| "dark".to_string());
    let mut is_loaded = use_signal(|| false);
    let mut is_mobile_menu_open = use_signal(|| false);
    let current_route = use_route::<crate::Route>();

    // Load theme from localStorage on mount
    use_effect(move || {
        if !is_loaded() {
            spawn(async move {
                let load_script = r#"
                    try {
                        const savedTheme = localStorage.getItem('theme') || 'dark';
                        document.documentElement.setAttribute('data-theme', savedTheme);
                        dioxus.send(savedTheme);
                    } catch (e) {
                        dioxus.send('dark');
                    }
                "#;

                if let Ok(saved_theme) = eval(load_script).recv::<String>().await {
                    theme.set(saved_theme.clone());
                    is_loaded.set(true);
                }
            });
        }
    });

    // Apply theme changes
    use_effect(move || {
        if is_loaded() {
            let current_theme = theme.read().clone();
            let _ = eval(&format!(
                r#"
                try {{
                    localStorage.setItem('theme', '{}');
                    document.documentElement.setAttribute('data-theme', '{}');
                    
                    // Broadcast theme change to all iframes
                    const iframes = document.querySelectorAll('iframe');
                    iframes.forEach(iframe => {{
                        try {{
                            iframe.contentWindow.postMessage({{
                                type: 'THEME_CHANGE',
                                theme: '{}'
                            }}, '*');
                        }} catch (e) {{}}
                    }});
                }} catch (e) {{}}
                "#,
                current_theme, current_theme, current_theme
            ));
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

    let is_route_active = |route: &crate::Route| {
        match (&current_route, route) {
            (crate::Route::Home {}, crate::Route::Home {}) => true,
            (crate::Route::About {}, crate::Route::About {}) => true,
            (crate::Route::Demos {}, crate::Route::Demos {}) => true,
            (crate::Route::Reading {}, crate::Route::Reading {}) => true,
            (crate::Route::Series {}, crate::Route::Series {}) => true,
            (crate::Route::SeriesDetail { .. }, crate::Route::Series {}) => true,
            (crate::Route::Articles {}, crate::Route::Articles {}) => true,
            _ => false,
        }
    };

    let nav_links = vec![
        ("/", "Home", crate::Route::Home {}),
        ("/about", "About", crate::Route::About {}),
        ("/demos", "Demos", crate::Route::Demos {}),
        ("/reading", "Reading", crate::Route::Reading {}),
        ("/series", "Series", crate::Route::Series {}),
        ("/articles", "Articles", crate::Route::Articles {}),
    ];

    let current_theme_val = theme.read().clone();
    let is_dark_mode = current_theme_val == "dark" || current_theme_val == "night" || current_theme_val == "dracula" || current_theme_val == "black" || current_theme_val == "luxury";

    rsx! {
        nav {
            class: "sticky top-0 z-50 border-b backdrop-blur-md bg-base-100/80 border-base-300",
            div {
                class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                div {
                    class: "flex justify-between h-16 items-center",
                    
                    // Logo
                    Link {
                        to: "/",
                        class: "flex-shrink-0 flex items-center gap-3 group",
                        div {
                            class: "w-8 h-8 rounded-lg flex items-center justify-center font-bold text-xl bg-primary text-primary-content transition-transform group-hover:scale-110",
                            "N"
                        }
                        span {
                            class: "font-bold text-xl tracking-tight hidden sm:inline",
                            "Nzuzo Magagula"
                        }
                    }

                    // Desktop Menu
                    div {
                        class: "hidden md:flex items-center space-x-1",
                        for (path, label, route) in nav_links.iter() {
                            Link {
                                to: path.to_string(),
                                class: if is_route_active(route) {
                                    "px-3 py-2 rounded-md text-sm font-medium bg-primary/10 text-primary transition-colors"
                                } else {
                                    "px-3 py-2 rounded-md text-sm font-medium hover:bg-base-200 text-base-content/70 hover:text-base-content transition-colors"
                                },
                                "{label}"
                            }
                        }
                        
                        div { class: "h-6 w-px mx-2 bg-base-300" }
                        
                        // Theme Toggle (Quick switch between light/dark)
                        button {
                            class: "p-2 rounded-full hover:bg-base-200 transition-colors text-warning",
                            onclick: move |_| {
                                let next = if is_dark_mode { "light" } else { "dark" };
                                theme.set(next.to_string());
                            },
                            if is_dark_mode {
                                svg { class: "w-5 h-5", fill: "none", stroke: "currentColor", view_box: "0 0 24 24", xmlns: "http://www.w3.org/2000/svg",
                                    path { stroke_linecap: "round", stroke_linejoin: "round", stroke_width: "2", d: "M12 3v1m0 16v1m9-9h-1M4 12H3m15.364-6.364l-.707.707M6.343 17.657l-.707.707m12.728 0l-.707-.707M6.343 6.343l-.707-.707M12 5a7 7 0 100 14 7 7 0 000-14z" }
                                }
                            } else {
                                svg { class: "w-5 h-5 text-base-content", fill: "none", stroke: "currentColor", view_box: "0 0 24 24", xmlns: "http://www.w3.org/2000/svg",
                                    path { stroke_linecap: "round", stroke_linejoin: "round", stroke_width: "2", d: "M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z" }
                                }
                            }
                        }

                        // More Themes Dropdown
                        details {
                            class: "dropdown dropdown-end",
                            summary {
                                class: "p-2 rounded-full hover:bg-base-200 transition-colors list-none cursor-pointer",
                                svg { class: "w-5 h-5 opacity-70", fill: "none", stroke: "currentColor", view_box: "0 0 24 24", xmlns: "http://www.w3.org/2000/svg",
                                    path { stroke_linecap: "round", stroke_linejoin: "round", stroke_width: "2", d: "M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" }
                                    path { stroke_linecap: "round", stroke_linejoin: "round", stroke_width: "2", d: "M15 12a3 3 0 11-6 0 3 3 0 016 0z" }
                                }
                            }
                            ul {
                                class: "dropdown-content menu bg-base-200 rounded-box z-[100] w-52 p-2 shadow-xl border border-base-300 absolute max-h-96 overflow-y-auto mt-2",
                                for (theme_id, theme_label) in available_themes.iter() {
                                    li {
                                        key: "{theme_id}",
                                        a {
                                            class: if theme.read().as_str() == *theme_id { "active" } else { "" },
                                            onclick: {
                                                let theme_id = theme_id.to_string();
                                                move |_| {
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

                    // Mobile menu button
                    div {
                        class: "md:hidden flex items-center gap-2",
                        button {
                            class: "p-2 rounded-full hover:bg-base-200 transition-colors text-warning",
                            onclick: move |_| {
                                let next = if is_dark_mode { "light" } else { "dark" };
                                theme.set(next.to_string());
                            },
                            if is_dark_mode {
                                svg { class: "w-5 h-5", fill: "none", stroke: "currentColor", view_box: "0 0 24 24", xmlns: "http://www.w3.org/2000/svg",
                                    path { stroke_linecap: "round", stroke_linejoin: "round", stroke_width: "2", d: "M12 3v1m0 16v1m9-9h-1M4 12H3m15.364-6.364l-.707.707M6.343 17.657l-.707.707m12.728 0l-.707-.707M6.343 6.343l-.707-.707M12 5a7 7 0 100 14 7 7 0 000-14z" }
                                }
                            } else {
                                svg { class: "w-5 h-5 text-base-content", fill: "none", stroke: "currentColor", view_box: "0 0 24 24", xmlns: "http://www.w3.org/2000/svg",
                                    path { stroke_linecap: "round", stroke_linejoin: "round", stroke_width: "2", d: "M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z" }
                                }
                            }
                        }
                        button {
                            class: "p-2 rounded-md hover:bg-base-200",
                            onclick: move |_| is_mobile_menu_open.toggle(),
                            if is_mobile_menu_open() {
                                svg { class: "w-6 h-6", fill: "none", stroke: "currentColor", view_box: "0 0 24 24", xmlns: "http://www.w3.org/2000/svg",
                                    path { stroke_linecap: "round", stroke_linejoin: "round", stroke_width: "2", d: "M6 18L18 6M6 6l12 12" }
                                }
                            } else {
                                svg { class: "w-6 h-6", fill: "none", stroke: "currentColor", view_box: "0 0 24 24", xmlns: "http://www.w3.org/2000/svg",
                                    path { stroke_linecap: "round", stroke_linejoin: "round", stroke_width: "2", d: "M4 6h16M4 12h16M4 18h16" }
                                }
                            }
                        }
                    }
                }
            }

            // Mobile Menu Dropdown
            if is_mobile_menu_open() {
                div {
                    class: "md:hidden border-t border-base-300 bg-base-100",
                    div {
                        class: "px-2 pt-2 pb-3 space-y-1",
                        for (path, label, _) in nav_links.iter() {
                            Link {
                                to: path.to_string(),
                                class: "block px-3 py-2 rounded-md text-base font-medium text-base-content/70 hover:bg-base-200 hover:text-base-content",
                                onclick: move |_| is_mobile_menu_open.set(false),
                                "{label}"
                            }
                        }
                        
                        // Theme selector in mobile
                        div {
                            class: "px-3 py-2 border-t border-base-300 mt-2",
                            p { class: "text-xs font-bold uppercase tracking-wider text-base-content/50 mb-2", "Change Theme" }
                            div {
                                class: "grid grid-cols-2 gap-2",
                                for (theme_id, theme_label) in available_themes.iter().take(8) {
                                    button {
                                        class: if theme.read().as_str() == *theme_id {
                                            "btn btn-xs btn-primary"
                                        } else {
                                            "btn btn-xs btn-ghost border-base-300"
                                        },
                                        onclick: {
                                            let theme_id = theme_id.to_string();
                                            move |_| {
                                                theme.set(theme_id.clone());
                                                is_mobile_menu_open.set(false);
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
}

