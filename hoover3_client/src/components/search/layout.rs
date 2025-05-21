//! Main search layout components for full-screen search page.

use std::time::Duration;

use dioxus::prelude::*;
use dioxus_sdk::utils::timing::use_debounce;

use crate::components::search::{context::{SearchContext, SearchParams}, document_preview::DocumentPreviewDisplay, facets::{CollectionSelector, FacetsList}, search_results::SearchResults};

/// Full-screen layout for the search page.
#[component]
pub fn SearchFullscreenLayout() -> Element {
    rsx! {
        SearchContext {
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
}

#[component]
fn SearchSidebarRight() -> Element {
    rsx! {
        DocumentPreviewDisplay {}
    }
}


/// Component with left sidebar
#[component]
pub fn SearchSidebarLeft() -> Element {
    rsx! {
        div { class: "sidebar-left-container",
            style: "
                display: flex;
                flex-direction: column;
                height: 100%;
            ",
            CollectionSelector {}
            FacetsList {}
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
fn SearchInput() -> Element {
    let search_params = use_context::<SearchParams>();
    let search_text = search_params.search_q;

    // using `use_debounce`, we reset the counter after 2 seconds since the last button click.
    let mut debounce = use_debounce(Duration::from_millis(160), move |text| {
        search_params.search_q_write.call(text);
    });

    rsx! {
        div {
            role: "group",
            input {
                placeholder: "*",
                value: "{search_text}",
                oninput: move |e| debounce.action(e.value().clone()),
            }
        }
    }
}
