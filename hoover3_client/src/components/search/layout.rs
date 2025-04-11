use dioxus::prelude::*;

/// Full-screen layout for the search page.
#[component]
pub fn SearchFullscreenLayout() -> Element {
    rsx! {
        div { id: "hero", class: "full-height",
            div { class: "search-grid-container full-height",
                div { class: "sidebar-left debug-border",
                    SearchSidebarLeft {}
                }
                div { class: "sidebar-right debug-border",
                    SearchSidebarRight {}
                }
                div { class: "sidebar-bottom debug-border",
                    SearchSidebarBottom {}
                }
                div { class: "search-main debug-border",
                    SearchMain {}
                }
            }
        }
    }
}

#[component]
fn SearchSidebarLeft() -> Element {
    rsx! {
        div {
            for i in 0..100 {
                div {
                    "Facet {i}"
                }
            }
        }
    }
}

#[component]
fn SearchSidebarRight() -> Element {
    rsx! {
        div {
            for i in 0..100 {
                div {
                    "Result {i}"
                }
            }

        }
    }
}

#[component]
fn SearchSidebarBottom() -> Element {
    rsx! {
        div {

        }
    }
}

#[component]
fn SearchMain() -> Element {
    rsx! {
        div {
            style: "
                display: flex;
                flex-direction: column; flex: 1;
                overflow: hidden; height:100%;
            ",
            div {
                style: "height: 4rem;width: 100%;",
                class: "debug-border",
                SearchInput {}
            }
            div {
                style: "
                    flex: 1;
                    height: 100%; width: 100%;
                    overflow:auto;
                ",
                class: "debug-border",
                SearchResults {}
            }
        }
    }
}

#[component]
fn SearchResults() -> Element {
    rsx! {
        div {
            for i in 0..100 {
                div {
                    "Result {i}"
                }
            }
        }
    }
}
#[component]
fn SearchInput() -> Element {
    rsx! {
        div {
            role: "group",
            input {
                placeholder: "...",
            }
        }
    }
}
