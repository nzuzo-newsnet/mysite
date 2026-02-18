use dioxus::prelude::*;

#[component]
pub fn AlgoVisPage() -> Element {
    rsx! {
        main {
            class: "flex-1 flex flex-col overflow-hidden bg-slate-950",
            div {
                class: "w-full h-full",
                iframe {
                    src: "/algovis/index.html",
                    class: "w-full h-full border-none",
                    title: "Algorithm Visualizer Demo",
                    allowfullscreen: true
                }
            }
        }
    }
}
