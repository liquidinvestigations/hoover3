mod dx_translate;
pub use dx_translate::DioxusTranslatePage;

mod dashboards;
pub use dashboards::*;

mod docker_health;
pub use docker_health::DockerHealthPage;

mod server_call_log;
pub use server_call_log::ServerCallLogPage;

mod database_explorer;
pub use database_explorer::*;

use dioxus::prelude::*;

use crate::{
    components::{
        cards::{CardGridDisplay, LinkCard},
        navbar::NavbarDropdown,
    },
    routes::Route,
};

fn tools_links() -> Vec<(String, Route)> {
    vec![
        (
            "Database Explorer".to_string(),
            Route::DatabaseExplorerPage {
                explorer_route: DatabaseExplorerRoute::RootPage {}.into(),
            },
        ),
        ("Docker Health".to_string(), Route::DockerHealthPage {}),
        ("Server Call Log".to_string(), Route::ServerCallLogPage {}),
        (
            "Dioxus Translate".to_string(),
            Route::DioxusTranslatePage {},
        ),
    ]
}

/// The navbar dropdown for the tools.
#[component]
pub fn ToolsNavbarDropdown() -> Element {
    rsx! {
        NavbarDropdown {
            title: "Tools".to_string(),
            links: tools_links().iter().map(|(title, link)| (title.to_string(), link.to_string())).collect(),
        }
    }
}
/// The home page for the tools.
#[component]
pub fn ToolsHomePage() -> Element {
    rsx! {
        h1 { "Tools" }
        CardGridDisplay {
            for (title, link) in tools_links() {
                LinkCard {
                    subtitle: "".to_string(),
                    title: title.to_string(),
                    link: link.clone(),
                }
            }
        }

        h1 { "Dashboards" }
        CardGridDisplay {
            for (title, link) in get_dashboard_links() {
                LinkCard {
                    subtitle: "".to_string(),
                    title: title.to_string(),
                    link: link.clone(),
                }
            }
        }
    }
}
