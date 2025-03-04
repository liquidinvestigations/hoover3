//! This module contains the routes for the Hoover3 client.
//! Routes are the URLs that are used to navigate the client.

mod url_param;
use hoover3_types::identifier::CollectionId;
use hoover3_types::identifier::DatabaseIdentifier;
pub use url_param::UrlParam;

use std::path::PathBuf;

use dioxus::prelude::*;

use crate::components::Navbar;
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
