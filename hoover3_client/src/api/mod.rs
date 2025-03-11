//! This module contains the server wrappers for all the API functions.

use std::path::PathBuf;

use crate::app::nav_push_server_call_event;
use crate::time::current_time;
use dioxus::prelude::*;
use hoover3_types::collection::*;
use hoover3_types::datasource::DatasourceSettings;
use hoover3_types::datasource::DatasourceUiRow;
use hoover3_types::db_schema::CollectionSchemaDynamic;
use hoover3_types::db_schema::DatabaseServiceType;
use hoover3_types::db_schema::DynamicQueryResponse;
use hoover3_types::docker_health::*;
use hoover3_types::filesystem::FsMetadataBasic;
use hoover3_types::filesystem::FsScanResult;
use hoover3_types::identifier::*;
use hoover3_types::tasks::*;

/// Struct records previous server calls, their timing and results.
#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
pub struct ServerCallEvent {
    /// Timestamp when the server call was initiated
    pub ts: f64,
    /// Name of the server function that was called
    pub function: String,
    /// String representation of the arguments passed to the function
    pub arg: String,
    /// Whether the server call has completed
    pub is_finished: bool,
    /// Whether the server call completed successfully
    pub is_successful: bool,
    /// String representation of the return value
    pub ret: String,
    /// Duration of the server call in seconds
    pub duration: f64,
}

/// Push a server start call event to the server call history.
/// This causes the loading spinner to be shown.
fn _before_call(function: &str, argument_val: &str, ts: f64) {
    nav_push_server_call_event(ServerCallEvent {
        ts,
        function: function.to_string(),
        arg: argument_val.to_string(),
        ret: "".to_string(),
        is_finished: false,
        is_successful: false,
        duration: 0.0,
    })
}

/// Push a server end call event to the server call history.
/// This causes the loading spinner to be hidden if no other calls are running..
fn _after_call(
    function: &str,
    argument_val: &str,
    ret_val: &str,
    is_successful: bool,
    ts: f64,
    duration: f64,
) {
    // info!("after_call: {function} {argument_val}");
    nav_push_server_call_event(ServerCallEvent {
        ts,
        function: function.to_string(),
        arg: argument_val.to_string(),
        ret: ret_val.to_string(),
        is_finished: true,
        is_successful,
        duration,
    })
}

fn truncate(s: &str, max_chars: usize) -> &str {
    match s.char_indices().nth(max_chars) {
        None => s,
        Some((idx, _)) => &s[..idx],
    }
}

const TIMEOUT_SECS: u64 = 60;

/// Result::flatten() is unstable/nightly, so we implement it here.
pub fn flatten_result<T, E>(x: Result<Result<T, E>, E>) -> Result<T, E> {
    match x {
        Ok(r) => r,
        Err(e) => Err(e),
    }
}

macro_rules! server_wrapper {
    ($ns:path,$id:ident,$arg:ty,$ret:ty) => {
        ::paste::paste! {
            mod [<__ $id __>] {
                use dioxus::prelude::*;
                #[allow(unused_imports)]
                use super::*;

                #[server]
                #[doc = "API Client wrapper - server part"]
                pub async fn [<__ $id __>](c: $arg)
                -> Result<$ret, ServerFnError>
                {
                    let rv = $ns::$id(c.clone())
                    .await
                    .map_err(|e| {
                        ::dioxus_logger::tracing::error!("Server: API method {} failed: {e:#?}", stringify!($id));
                        ServerFnError::new(
                            format!("{}: {e:#?}", stringify!($id)))
                    });
                    rv
                }
            }
            pub use [<__ $id __>]::[<__ $id __>];

            #[doc = "API Client wrapper - client part - for the backend function"]
            #[doc = "[`"]
            #[doc = stringify!($ns)]
            #[doc = "::"]
            #[doc = stringify!($id)]
            #[doc = "`]"]
            #[doc = "
            This is the wrapper function that runs on the client.
            It retries the server call up to 2 times, and logs the results
            using _before_call and _after_call."]
            #[allow(missing_docs)]
            pub async fn $id(c: $arg)
            -> Result<$ret, ServerFnError>
            {
                let retry = 2;
                for i in 1..=retry {
                    let rv = {
                        let arg_str = format!("{c:?}");
                        let arg_str = truncate(&arg_str, 1024);
                        let t0 = current_time();
                        _before_call(stringify!($id), arg_str, t0);
                        let rv = async_std::future::timeout(
                            std::time::Duration::from_secs(TIMEOUT_SECS),
                            [<__ $id __>](c.clone())).await.map_err(|e| ServerFnError::new(
                                format!("{}: timeout: {e:#?}", stringify!($id))));
                        let rv = flatten_result(rv);
                        let t1 = current_time();
                        let ret_str = format!("{rv:#?}");
                        let ret_str = truncate(&ret_str, 1024);
                        _after_call(stringify!($id), arg_str, ret_str, rv.is_ok(), t1, t1-t0);
                        rv
                    };

                    if rv.is_ok() {
                        return rv;
                    }
                    if i == retry {
                        return rv;
                    }
                    ::dioxus_logger::tracing::info!("Client: API method {} failed, retrying...", stringify!($id));
                    $crate::time::sleep(
                        std::time::Duration::from_secs_f32(
                            0.1+i as f32*0.1)).await;
                }
                unreachable!()
            }
        }
    };
}

server_wrapper!(
    hoover3_database::client_query::collections,
    create_new_collection,
    CollectionId,
    CollectionUiRow
);
server_wrapper!(
    hoover3_database::client_query::collections,
    get_all_collections,
    (),
    Vec<CollectionUiRow>
);
server_wrapper!(
    hoover3_database::client_query::collections,
    update_collection,
    CollectionUiRow,
    CollectionUiRow
);
server_wrapper!(
    hoover3_database::client_query::collections,
    drop_collection,
    CollectionId,
    ()
);

server_wrapper!(
    hoover3_database::client_query::docker_health,
    get_container_status,
    (),
    Vec<ContainerHealthUi>
);

server_wrapper!(
    hoover3_database::client_query::collections,
    get_single_collection,
    CollectionId,
    CollectionUiRow
);

server_wrapper!(
    hoover3_server::hoover3_data_access::api,
    get_all_datasources,
    CollectionId,
    Vec<DatasourceUiRow>
);

server_wrapper!(
    hoover3_database::client_query::list_disk,
    list_directory,
    PathBuf,
    Vec<FsMetadataBasic>
);

server_wrapper!(
    hoover3_server::hoover3_data_access::api,
    create_datasource,
    (CollectionId, DatabaseIdentifier, DatasourceSettings),
    DatasourceUiRow
);

server_wrapper!(
    hoover3_server::hoover3_data_access::api,
    get_datasource,
    (CollectionId, DatabaseIdentifier),
    DatasourceUiRow
);

server_wrapper!(
    hoover3_server::hoover3_filesystem_scanner::api,
    start_scan,
    (CollectionId, DatabaseIdentifier),
    String
);

server_wrapper!(
    hoover3_server::hoover3_filesystem_scanner::api,
    get_scan_status,
    (CollectionId, DatabaseIdentifier),
    UiWorkflowStatus
);

server_wrapper!(
    hoover3_taskdef::api::status_tree,
    get_workflow_status_tree,
    String,
    TemporalioWorkflowStatusTree
);

server_wrapper!(
    hoover3_server::hoover3_filesystem_scanner::api,
    wait_for_scan_results,
    (CollectionId, DatabaseIdentifier),
    FsScanResult
);

server_wrapper!(
    hoover3_database::migrate,
    get_collection_schema,
    CollectionId,
    CollectionSchemaDynamic
);

server_wrapper!(
    hoover3_database::client_query::database_explorer,
    scylla_row_count,
    (CollectionId, DatabaseIdentifier),
    i64
);

server_wrapper!(
    hoover3_database::client_query::database_explorer,
    db_explorer_run_query,
    (CollectionId, DatabaseServiceType, String),
    DynamicQueryResponse
);

server_wrapper!(hoover3_server::api, get_server_memory_usage, (), (u32, u32));
