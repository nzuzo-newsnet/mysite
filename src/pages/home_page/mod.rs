pub mod blog;
pub mod projects;
use crate::pages::home_page::{blog::Blogs, projects::Projects};
use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq)]
enum MobileTab {
    Articles,
    Projects,
}

#[component]
pub fn HomePage() -> Element {
    let mut active_tab = use_signal(|| MobileTab::Articles);

    rsx! {
        main {
            class: "flex-1 w-full overflow-hidden",

            // Desktop layout (md and up) - No tabs, side by side
            div {
                class: "hidden md:flex md:flex-row h-full w-full",

                // Articles on the left
                div {
                    class: "flex-1 overflow-y-auto p-8",
                    Blogs {}
                }

                // Projects on the right
                div {
                    class: "w-80 lg:w-96 border-l border-base-300 bg-base-100 overflow-y-auto",
                    Projects {}
                }
            }

            // Mobile layout (below md) - Tabs + content
            div {
                class: "flex md:hidden flex-col h-full w-full",

                // Content area
                div {
                    class: "flex-1 overflow-y-auto p-2 sm:p-4 pb-20",
                    if active_tab() == MobileTab::Articles {
                        Blogs {}
                    } else {
                        Projects {}
                    }
                }

                // Bottom tab bar
                div {
                    class: "fixed bottom-0 left-0 right-0 z-50 bg-base-100 border-t border-base-300 flex justify-around items-center p-2",
                button {
                    class: if active_tab() == MobileTab::Articles {
                        "btn btn-ghost btn-active bg-primary text-primary-content flex flex-col gap-1 flex-1"
                    } else {
                        "btn btn-ghost flex flex-col gap-1 flex-1"
                    },
                    onclick: move |_| active_tab.set(MobileTab::Articles),
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        class: "h-5 w-5",
                        fill: "none",
                        view_box: "0 0 24 24",
                        stroke: "currentColor",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "2",
                            d: "M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"
                        }
                    }
                    span {
                        class: "text-xs",
                        "Articles"
                    }
                }
                button {
                    class: if active_tab() == MobileTab::Projects {
                        "btn btn-ghost btn-active bg-primary text-primary-content flex flex-col gap-1 flex-1"
                    } else {
                        "btn btn-ghost flex flex-col gap-1 flex-1"
                    },
                    onclick: move |_| active_tab.set(MobileTab::Projects),
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        class: "h-5 w-5",
                        fill: "none",
                        view_box: "0 0 24 24",
                        stroke: "currentColor",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "2",
                            d: "M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
                        }
                    }
                    span {
                        class: "text-xs",
                        "Projects"
                    }
                }
                }
            }
        }
    }
}
