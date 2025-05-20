use dioxus::prelude::*;

use crate::components::search::layout::SearchFullscreenLayout;

/// Home page with main search interface.
#[component]
pub fn SearchHomePage() -> Element {
    rsx! {
        SearchFullscreenLayout {}
    }
}
