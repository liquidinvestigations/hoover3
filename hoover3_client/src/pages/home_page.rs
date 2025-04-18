use dioxus::prelude::*;

use crate::{components::fullscreen_links::FullscreenLinks, routes::Route};

/// Home page with main search interface.
#[component]
pub fn HomePage() -> Element {
    let links = vec![
        (Route::SearchHomePage {}, rsx! {"Search"}),
        (
            Route::DatabaseExplorerRootPage {},
            rsx! {"Database Explorer"},
        ),
        (Route::ToolsHomePage {}, rsx! {"Tools"}),
        (Route::AdminHomePage {}, rsx! {"Admin"}),
    ];
    rsx! {
        FullscreenLinks { links }
    }
}
