use dioxus::prelude::*;

use crate::components::search::SearchFullscreenLayout;

/// Home page with main search interface.
#[component]
pub fn SearchHomePage() -> Element {
    rsx! {
        SearchFullscreenLayout {}
    }
}
