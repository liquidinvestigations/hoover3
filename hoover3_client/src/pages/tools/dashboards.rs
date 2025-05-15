use dioxus::prelude::*;

use crate::components::navbar::NavbarDropdown;
use crate::routes::Route;

fn get_dash_links_raw() -> Vec<(String, String)> {
    vec![
        ("TEMPORAL UI  ", "http://localhost:8081"),
        ("MINIO DASH   ", "http://localhost:8084"),
        ("MEILISEARCH  ", "http://localhost:8085"),
        (
            "SCYLLA DB",
            "http://localhost:8086/cassandra/clusters/scylla/explore/",
        ),
        (
            "REDIS",
            "http://localhost:8086/dynomite/clusters/redis/keys",
        ),
        // ("NEBULA GRAPH STUDIO  ", "http://localhost:7001/"),
        (
            "TEMPORAL DB   ",
            "http://localhost:8088/cassandra/clusters/temporal-cassandra/explore/temporal/tables",
        ),
        ("SEAWEEDFS VOL", "http://localhost:8082/ui/index.html"),
        ("SEAWEEDFS MAS", "http://localhost:8083"),
        (
            "CARGO DOCS",
            "http://localhost:8087/hoover3_client/index.html",
        ),
        ("CLICKHOUSE", "http://localhost:3000/0/database/system"),
    ]
    .into_iter()
    .map(|(name, link)| (name.trim().to_string(), link.trim().to_string()))
    .collect()
}

/// Page that displays a dashboard iframe.
#[component]
pub fn DashboardIframePage(id: u8) -> Element {
    let src = get_dash_links_raw()[id as usize].1.clone();

    rsx! {
        iframe {
            id: "dashboard-iframe",
            class: "full-height",
            style: "width:100%;",
            src,
        }
    }
}

/// Get a list of internal dashboard links.
pub fn get_dashboard_links() -> Vec<(String, Route)> {
    let x = get_dash_links_raw();
    let links = x
    .into_iter()
    .enumerate()
    .map(|(id, (name, _link))| {
        (
            name,
            Route::DashboardIframePage { id: id as u8 },
        )
    })
    .collect();
links
}

/// Navbar dropdown that displays a list of dashboard links.
#[component]
pub fn DashboardNavbarDropdown() -> Element {
    let links = get_dashboard_links();
    let links = links.into_iter().map(|(name, link)| (name.to_string(), link.to_string())).collect();
    rsx! {
        NavbarDropdown { title: "Dashboards", links }
    }
}

/// The home page for the dashboards.
#[component]
pub fn DashboardsHomePage() -> Element {
    rsx! {
        h1 {
            "System Dashboards"
        }
        for (id, (name, link)) in get_dash_links_raw().into_iter().enumerate() {
            p {
                "{name}:"
                Link {
                    to: Route::DashboardIframePage { id: id as u8 }.to_string(),
                    "{link}"
                }
            }
        }
    }
}
