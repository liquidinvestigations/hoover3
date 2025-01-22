use dioxus::prelude::*;

use crate::routes::NavbarDropdown;
use crate::routes::Route;

fn get_dash_links() -> Vec<(String, String)> {
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
        ("NEBULA GRAPH STUDIO  ", "http://localhost:7001/"),
        (
            "TEMPORAL DB   ",
            "http://localhost:8088/cassandra/clusters/temporal-cassandra/explore/temporal/tables",
        ),
        ("SEAWEEDFS VOL", "http://localhost:8082/ui/index.html"),
        ("SEAWEEDFS MAS", "http://localhost:8083"),
    ]
    .into_iter()
    .map(|(name, link)| (name.trim().to_string(), link.trim().to_string()))
    .collect()
}

#[component]
pub fn DashboardIframePage(id: u8) -> Element {
    let src = get_dash_links()[id as usize].1.clone();
    rsx! {
        iframe {
            class: "full-height",
            style: "width:100%;",
            src,
        }
    }
}

#[component]
pub fn DashboardNavbarDropdown() -> Element {
    let x = get_dash_links();
    let links = x
        .into_iter()
        .enumerate()
        .map(|(id, (name, _link))| {
            (
                name,
                Route::DashboardIframePage { id: id as u8 }.to_string(),
            )
        })
        .collect();
    rsx! {
        NavbarDropdown { title: "Dashboards", links }
    }
}
