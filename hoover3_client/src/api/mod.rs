use std::path::PathBuf;

use crate::routes::nav_push_server_call_event;
use crate::time::current_time;
use dioxus::prelude::*;
use hoover3_types::collection::*;
use hoover3_types::datasource::DatasourceSettings;
use hoover3_types::datasource::DatasourceUiRow;
use hoover3_types::db_schema::CollectionSchema;
use hoover3_types::db_schema::DatabaseType;
use hoover3_types::db_schema::DynamicQueryResponse;
use hoover3_types::docker_health::*;
use hoover3_types::filesystem::FsMetadataBasic;
use hoover3_types::filesystem::FsScanDatasourceResult;
use hoover3_types::identifier::*;
use hoover3_types::tasks::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
pub struct ServerCallEvent {
    pub ts: f64,
    pub function: String,
    pub arg: String,
    pub is_finished: bool,
    pub is_successful: bool,
    pub ret: String,
    pub duration: f64,
}

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

fn _after_call(
    function: &str,
    argument_val: &str,
    ret_val: &str,
    is_successful: bool,
    ts: f64,
    duration: f64,
) {
    // dioxus_logger::tracing::info!("after_call: {function} {argument_val}");
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

pub fn flatten_result<T, E>(x: Result<Result<T, E>, E>) -> Result<T, E> {
    match x {
        Ok(r) => r,
        Err(e) => Err(e),
    }
}

macro_rules! server_wrapper {
    ($ns:path,$id:ident,$arg:ty,$ret:ty) => {
        ::paste::paste! {
            #[server]
            pub async fn [<__ $id __>](c: $arg)
            -> Result<$ret, ServerFnError>
            {
                let rv = $ns::$id(c.clone())
                .await
                .map_err(|e| ServerFnError::new(
                    format!("{}: {e}", stringify!($id))));
                rv
            }

            /// UI wrapper over a server function $id
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
                            std::time::Duration::from_secs(30),
                            [<__ $id __>](c.clone())).await.map_err(|e| ServerFnError::new(
                                format!("{}: timeout: {e}", stringify!($id))));
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
    hoover3_database::client_query::datasources,
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
    hoover3_database::client_query::datasources,
    create_datasource,
    (CollectionId, DatabaseIdentifier, DatasourceSettings),
    DatasourceUiRow
);

server_wrapper!(
    hoover3_database::client_query::datasources,
    get_datasource,
    (CollectionId, DatabaseIdentifier),
    DatasourceUiRow
);

server_wrapper!(
    hoover3_filesystem_scanner,
    start_scan,
    (CollectionId, DatabaseIdentifier),
    ()
);

server_wrapper!(
    hoover3_filesystem_scanner,
    get_scan_status,
    (CollectionId, DatabaseIdentifier),
    UiWorkflowStatus
);

server_wrapper!(
    hoover3_taskdef::tasks::status_tree,
    get_workflow_status_tree,
    String,
    TemporalioWorkflowStatusTree
);

server_wrapper!(
    hoover3_filesystem_scanner,
    wait_for_scan_results,
    (CollectionId, DatabaseIdentifier),
    FsScanDatasourceResult
);

server_wrapper!(
    hoover3_database::migrate,
    get_collection_schema,
    CollectionId,
    CollectionSchema
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
    (CollectionId, DatabaseType, String),
    DynamicQueryResponse
);
