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
#[component]
pub fn ToolsHomePage() -> Element {
    rsx! {
        "Tools Home Page"
    }
}
