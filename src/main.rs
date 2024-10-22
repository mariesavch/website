#![allow(non_snake_case)]

use dioxus::prelude::*;

const _TAILWIND_URL: &str = manganis::mg!(file("assets/tailwind.css"));

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        main { class: "mx-auto max-w-3xl px-6 pb-20",
            div { class: "pt-16",
                div {
                    h1 { class: "mb-2 text-xl font-bold", "marie" }
                    div { class: "text-overlay0 mb-5", "she/her" }
                }
                ul { class: "animated-list grid grid-cols-1 sm:grid-cols-2",
                    li {
                        div { class: "flex py-3 flex-col gap-1",
                            span { class: "text-overlay0", "github" }
                            a {
                                class: "underlined",
                                href: "https://github.com/mariesavch",
                                "mariesavch"
                            }
                        }
                    }

                    li {
                        div { class: "flex py-3 flex-col gap-1",
                            span { class: "text-overlay0", "github" }
                            a {
                                class: "underlined",
                                href: "mailto:mariesavch@icloud.com",
                                "mariesavch@icloud.com"
                            }
                        }
                    }
                }
            }
        }
    }
}
