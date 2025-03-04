use dioxus::prelude::*;
use dioxus_logger::tracing::{info, warn};

use crate::{app::ServerCallHistory, pages::{DashboardNavbarDropdown, DatabaseExplorerRoute}, routes::Route, time::sleep};

/// Component for the navbar - a single <nav> element with the stuff on the top row.
/// Also contains some `position_absolute` overlay elements in [ExtraLayout]
#[component]
pub fn Navbar() -> Element {
    info!("Navbar()");
    rsx! {
        ExtraLayout {}
        nav { id: "navbar",
            // title crumbs with links to parent pages
            ul {
                li {
                    NavbarTitleCrumbs {}
                }
            }
            // loading icon and count
            ul {
                LoadingBoxGif {}
            }
            // Dropdowns
            ul {
                // dropdown with dashboard links
                li { DashboardNavbarDropdown {} }
                // dropdown with tools
                li {
                    NavbarDropdown {
                        title: "Tools",
                        links: vec![
                            ("DatabaseExplorer".to_string(), Route::DatabaseExplorerPage {
                                explorer_route: DatabaseExplorerRoute::RootPage.into()
                            }.to_string()),
                            ("ServerCallLogPage".to_string(), Route::ServerCallLogPage {}.to_string()),
                            ("DioxusTranslate".to_string(), Route::DioxusTranslatePage {}.to_string()),
                            ("DockerHealth".to_string(), Route::DockerHealthPage {}.to_string()),
                        ],
                    }
                }
                // dropdown with admin links
                li {
                    NavbarDropdown {
                        title: "Admin",
                        links: vec![
                            ("Collections".to_string(), Route::CollectionsAdminListPage {}.to_string()),
                        ],
                    }
                }
            }
        }
    }
}

#[component]
fn LoadingBoxGif() -> Element {
    let ServerCallHistory {
        show_pic,
        loading_count,
        ..
    } = use_context();
    rsx! {
        div { class: "loading_box",
            if *show_pic.read() {
                img { src: "/assets/img/loading.gif" }
                div { class: "loading_count", "{loading_count}" }
            }
        }
    }
}

#[component]
fn NavbarTitleCrumbs() -> Element {
    let route = use_route::<Route>();
    let mut parents = vec![];
    let mut route2 = route.clone();
    while let Some(route_parent) = route2.parent() {
        route2 = route_parent.clone();
        parents.push(route_parent);
    }
    parents.reverse();

    rsx! {
        div {
            style: "overflow:hidden; white-space:nowrap; max-width: 50vw; margin:0.3rem; padding: 0.3rem;",
            for parent in parents {
                Link {
                    style:"overflow:hidden; white-space:nowrap; max-width: 10vw; margin:0.3rem; padding: 0.3rem; display:inline;",
                    to: parent.clone(),
                    "{parent:?}"
                }
                span {
                    style:"margin:0.3rem; padding: 0.3rem; display:inline;",
                    " > "
                }
            }
            Link {
                style:"overflow:hidden; white-space:nowrap; max-width: 10vw; margin:0.3rem; padding: 0.3rem; display:inline;",
                to:route.clone(),
                " {route:?}"
            }
        }
    }
}


/// Component for a dropdown menu in the navbar.
#[component]
pub fn NavbarDropdown(title: String, links: Vec<(String, String)>) -> Element {
    rsx! {
        details { class: "dropdown",
            summary { {title} }
            ul {
                for (link_name , link) in links {
                    li { key: link.clone(),
                        Link { to: link.clone(), {link_name} }
                    }
                }
            }
        }
    }
}

#[component]
fn ExtraLayout() -> Element {
    rsx! {
        div {
            style: "
                position:absolute;
                bottom:0;
                right:0;
                z-index:1000;
                min-width: 40dvw;
                font-size:0.6rem;
            ",
            div {
                style: "
                    position:relative;
                    width: fit-content;
                    display:flex;
                ",
                MemoryUsageDisplayBrowser{}
                MemoryUsageDisplayServer{}
            }
        }
    }
}

#[component]
fn MemoryUsageDisplayBrowser() -> Element {
    let mut mem_limit = use_signal(|| 0);
    let mut mem_usage = use_signal(|| 0);

    let _c = use_coroutine(move |_: UnboundedReceiver<()>| async move {
        loop {
            let usage = hoover3_tracing::get_process_memory_usage();
            let limit = hoover3_tracing::get_process_memory_limit();
            mem_usage.set(usage);
            mem_limit.set(limit);
            sleep(std::time::Duration::from_secs(60)).await;

        }
    });

    rsx! {
        div {
            style: "margin-right:50px;",
            "Browser memory: {mem_usage} / {mem_limit} MB"
        }
    }
}

#[component]
fn MemoryUsageDisplayServer() -> Element {
    let mut mem_limit = use_signal(|| 0);
    let mut mem_usage = use_signal(|| 0);

    let _c = use_coroutine(move |_: UnboundedReceiver<()>| async move {
        loop {
            if let Ok((usage, limit)) = crate::api::get_server_memory_usage(()).await {
                mem_usage.set(usage);
                mem_limit.set(limit);
            } else {
                warn!("Failed to get server memory usage!");
            }
            sleep(std::time::Duration::from_secs(60)).await;
        }
    });

    rsx! {
        div {
            style: "margin-right:50px;",
            "Server memory: {mem_usage} / {mem_limit} MB"
        }
    }
}