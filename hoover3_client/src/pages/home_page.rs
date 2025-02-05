use dioxus::prelude::*;

/// Home page with main search interface.
#[component]
pub fn HomePage() -> Element {
    rsx! {
        Hero {}
    }
}

#[component]
fn Hero() -> Element {
    rsx! {
        div { id: "hero", class: "full-height",
            div { class: "search-grid-container full-height",
                div { class: "sidebar-left debug-border", "sidebar-left" }
                div { class: "sidebar-right debug-border", "sidebar right" }
                div { class: "sidebar-bottom debug-border", "sidebar-bottom" }
                div { class: "search-main debug-border", "sidebar-top" }
            }
        }
    }
}
