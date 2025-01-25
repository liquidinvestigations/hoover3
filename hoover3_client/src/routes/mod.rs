mod url_param;
pub use url_param::UrlParam;

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::path::PathBuf;

use dioxus::prelude::*;
use dioxus_logger::tracing::info;

use crate::api::ServerCallEvent;
use crate::pages::CollectionAdminDetailsPage;
use crate::pages::CollectionsAdminListPage;
use crate::pages::DashboardIframePage;
use crate::pages::DashboardNavbarDropdown;
use crate::pages::DatasourceAdminDetailsPage;
use crate::pages::DioxusTranslatePage;
use crate::pages::DockerHealthPage;
use crate::pages::HomePage;
use crate::pages::NewDatasourceFormPage;
use crate::pages::ServerCallLogPage;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
#[allow(clippy::empty_line_after_outer_attr)]
#[allow(clippy::empty_line_before_outer_attr)]
pub enum Route {
    #[layout(NavbarLayout)]

    #[route("/")]
    HomePage {},

    #[nest("/tools")]
        #[route("/dioxus-translate")]
        DioxusTranslatePage {},

        #[route("/docker-health")]
        DockerHealthPage {},

        #[route("/server-call-logs")]
        ServerCallLogPage {},
    #[end_nest]

    #[nest("/admin")]
        #[route("/collections")]
        CollectionsAdminListPage {},

        #[route("/collection/:collection_id")]
        CollectionAdminDetailsPage{collection_id: String},

        #[route("/collection/:collection_id/new_datasource/#:current_path")]
        NewDatasourceFormPage {collection_id: String, current_path: UrlParam<PathBuf>},

        #[route("/collection/:collection_id/datasource/:datasource_id")]
        DatasourceAdminDetailsPage {collection_id: String, datasource_id: String},
    #[end_nest]

    #[route("/dashboards/iframe/:id")]
    DashboardIframePage{id:u8},

    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}

#[component]
fn Navbar(display_loading_icon: ReadOnlySignal<bool>) -> Element {
    rsx! {
        nav { id: "navbar",

            ul {
                li {
                    Link { to: Route::HomePage {}, "Home" }
                }
            }
            ul {
                div { class: "loading_box",
                    if *display_loading_icon.read() {
                        img { src: "/assets/img/loading.gif" }
                    }
                }
            }
            ul {
                li { DashboardNavbarDropdown {} }
                li {
                    NavbarDropdown {
                        title: "Tools",
                        links: vec![
                            ("ServerCallLogPage".to_string(), Route::ServerCallLogPage {}.to_string()),
                            ("DioxusTranslate".to_string(), Route::DioxusTranslatePage {}.to_string()),
                            ("DockerHealth".to_string(), Route::DockerHealthPage {}.to_string()),
                        ],
                    }
                }
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
fn DisplayError(title: String, err: String) -> Element {
    rsx! {
        div {
            class: "container",
            article {
                h1 { {title} }
                pre { color: "red", "{err}" }
            }
        }
    }
}

#[component]
pub fn NavbarDropdown(title: String, links: Vec<(String, String)>) -> Element {
    rsx! {
        details { class: "dropdown",
            summary { {title} }
            ul {
                for (link_name , link) in links {
                    li { key: link,
                        Link { to: link, {link_name} }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
struct ServerCallHistory {
    cb: Callback<ServerCallEvent>,
    hist: ReadOnlySignal<HashMap<String, VecDeque<ServerCallEvent>>>,
}

pub fn nav_push_server_call_event(event: ServerCallEvent) {
    let ServerCallHistory { cb, .. } = use_context();
    cb.call(event);
}

pub fn read_server_call_history() -> ReadOnlySignal<HashMap<String, VecDeque<ServerCallEvent>>> {
    let ServerCallHistory { hist, .. } = use_context();
    hist
}

/// Component that wraps the main page with the navbar and error handler.
/// Also initializes the component tracking backend server calls.
#[component]
fn NavbarLayout() -> Element {
    let mut currently_loading = use_signal(HashSet::new);
    let mut show_pic = use_signal(|| false);
    use_effect(move || {
        show_pic.set(!currently_loading.read().is_empty());
    });
    use_effect(move || {
        let c = currently_loading.read();
        info!("currently_loading: {:#?}", c);
    });

    let mut hist = dioxus_sdk::storage::use_synced_storage::<
        dioxus_sdk::storage::LocalStorage,
        HashMap<String, VecDeque<ServerCallEvent>>,
    >("HashMap_ServerCallEvent".to_string(), || {
        HashMap::<String, VecDeque<ServerCallEvent>>::new()
    });

    // let mut hist = use_signal(|| HashMap::<String, VecDeque<ServerCallEvent>>::new());
    use_context_provider(|| ServerCallHistory {
        cb: Callback::new(move |event: ServerCallEvent| {
            if event.is_finished {
                currently_loading
                    .write()
                    .remove(&(event.function.clone(), event.arg.clone()));
                let mut h = hist
                    .peek()
                    .get(&event.function)
                    .unwrap_or(&VecDeque::new())
                    .clone();
                h.push_front(event.clone());
                if h.len() > 10 {
                    h.pop_back();
                }
                hist.write().insert(event.function, h);
            } else {
                currently_loading
                    .write()
                    .insert((event.function, event.arg));
            }
        }),
        hist: ReadOnlySignal::new(hist),
    });
    rsx! {
        div { class: "page-wrapper",
            div { class: "container",
                Navbar { display_loading_icon: show_pic }
            }

            main {
                class: "container-fluid",
                style: "height:99%;overflow:scroll;",
                ErrorBoundary {
                    handle_error: |err_ctx: ErrorContext| rsx! {
                        DisplayError{title: "Error", err: format!("{:#?}", err_ctx)}
                    },
                    SuspenseBoundary {
                        fallback: |_ctx: SuspenseContext| rsx! {
                            h1 { "Loading..." }
                        },
                        Outlet::<Route> {}
                    }
                }
            }
        }
    }
}

#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        DisplayError {
            title: "Page not found".to_string(),
            err: format!("{route:?}"),
        }
    }
}
