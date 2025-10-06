use dioxus::prelude::*;
use dioxus_free_icons::{icons::ld_icons::LdHome, Icon};
#[component]
pub fn NavBar() -> Element {
    rsx! {
        nav {
            class: "h-fit w-full flex flex-row justify-between p-5",
            ol {
                class: "flex flex-row",
                li {

                    Icon {
                        class: "btn btn-xl",
                        height: 24,
                        width: 24,
                        fill: "white",
                        icon: LdHome,
                    }
                }
                li {

                    h1 {
                        class: "btn btn-xl",
                        "Contact"
                    }
                }
            }
            h1 {
                class: "text-6xl",
                "Nzuzo Magagula"
            }
        }
    }
}
