//! This module contains the routes for the Hoover3 client.
//! Routes are the URLs that are used to navigate the client.

mod url_param;
use hoover3_types::identifier::CollectionId;
use hoover3_types::identifier::DatabaseIdentifier;
pub use url_param::UrlParam;

use std::collections::BTreeMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::path::PathBuf;

use dioxus::prelude::*;
use dioxus_logger::tracing::info;

use crate::api::ServerCallEvent;
use crate::app::ServerCallHistory;
use crate::pages::*;

/// The enum of all the routes for the Hoover3 client.
/// To nest another route type object inside a page, use `UrlParam<T>`.
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
#[allow(missing_docs)]
#[allow(clippy::empty_line_after_outer_attr)]
pub enum Route {
    #[layout(NavbarLayout)]

    /// Route to home page.
    #[route("/")]
    HomePage {},

    #[nest("/tools")]
        /// Tools home page - lists all the sub-pages
        #[route("/")]
        ToolsHomePage {},

        /// Route to Dioxus HTML Translator
        #[route("/dioxus-translate")]
        DioxusTranslatePage {},

        /// Route to Docker Health Checker
        #[route("/docker-health")]
        DockerHealthPage {},

        /// Route to Server Call Logs
        #[route("/server-call-logs")]
        ServerCallLogPage {},

        #[nest("/database-explorer")]
            /// Route to Database Explorer Home Page
            #[route("/")]
            DatabaseExplorerRootPage {},

            /// Route to Database Explorer
            #[route("/:explorer_route")]
            DatabaseExplorerPage {explorer_route: UrlParam<DatabaseExplorerRoute>},
        #[end_nest]

        #[nest("/dashboards")]
            /// Route to Dashboard Home Page
            #[route("/")]
            DashboardsHomePage {},
            /// Route to specific Dashboard Iframe
            #[route("/:id")]
            DashboardIframePage{id:u8},
        #[end_nest]
    #[end_nest]

    #[nest("/admin")]
        /// Route to Admin Home Page
        #[route("/")]
        AdminHomePage {},

        #[nest("/collections")]
            /// Route to Collections Admin List
            #[route("/")]
            CollectionsAdminListPage {},

            /// Route to Collection Admin Details
            #[route("/:collection_id")]
            CollectionAdminDetailsPage{collection_id: CollectionId},

            /// Route to New Datasource Form
            #[route("/:collection_id/new_datasource/#:current_path")]
            NewDatasourceFormPage {collection_id: CollectionId, current_path: UrlParam<PathBuf>},

            /// Route to Datasource Admin Details
            #[route("/:collection_id/datasource/:datasource_id")]
            DatasourceAdminDetailsPage {collection_id: CollectionId, datasource_id: DatabaseIdentifier},
        #[end_nest]
    #[end_nest]

    /// Route to Page Not Found 404 error
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}

#[component]
fn Navbar() -> Element {
    let ServerCallHistory {
        show_pic,
        loading_count,
        ..
    } = use_context();
    rsx! {
        nav { id: "navbar",
            // title crumbs with links to parent pages
            ul {
                li {
                    NavbarTitleCrumbs {}
                }
            }
            // loading icon and count
            ul {
                div { class: "loading_box",

                    if *show_pic.read() {
                        img { src: "/assets/img/loading.gif" }
                        div { class: "loading_count", "{loading_count}" }
                    }
                }
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

/// Component for a dropdown menu in the navbar.
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

/// Component that wraps the main page with the navbar and error handler.
/// Also initializes the component tracking backend server calls.
#[component]
fn NavbarLayout() -> Element {
    rsx! {
        div { class: "page-wrapper",
            div { class: "container-fluid",
                Navbar { }
            }

            main {
                class: "container-fluid",
                style: "height:99%;overflow: auto;",
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

/// Component for displaying a page not found 404 wrong URL error.
#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        DisplayError {
            title: "Page not found".to_string(),
            err: format!("{route:?}"),
        }
    }
}
