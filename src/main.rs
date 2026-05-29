mod store;
mod sync;
mod tools;

use dioxus::prelude::*;
use tools::registry;

const CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

fn App() -> Element {
    let tools = registry();
    let first = tools.first().map(|t| t.id).unwrap_or("todo");
    let mut active = use_signal(|| first);
    let show_nav = tools.len() > 1;

    rsx! {
        Stylesheet { href: CSS }

        div { class: "app",

            // 顶部区域：品牌栏 + 导航条（工具 > 1 时才出现）
            div { class: "app-head",
                header { class: "topbar",
                    h1 { class: "brand", "DayKit" }
                }

                if show_nav {
                    nav { class: "navstrip",
                        for t in registry() {
                            button {
                                class: "navtab",
                                class: if active() == t.id { "active" },
                                onclick: {
                                    let id = t.id;
                                    move |_| active.set(id)
                                },
                                span { class: "navtab-icon", "{t.icon}" }
                                span { class: "navtab-label", "{t.name}" }
                            }
                        }
                    }
                }
            }

            main { class: "content",
                ToolOutlet { id: active() }
            }
        }
    }
}

#[component]
fn ToolOutlet(id: &'static str) -> Element {
    match id {
        "todo" => rsx! { tools::todo::TodoView {} },
        _ => rsx! { p { class: "empty", "未知工具:{id}" } },
    }
}
