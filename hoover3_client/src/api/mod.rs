use std::path::PathBuf;

use crate::routes::nav_push_server_call_event;
use crate::time::current_time;
use dioxus::prelude::*;
use dioxus_logger::tracing::warn;
use hoover3_types::collection::*;
use hoover3_types::data_access::{DataAccessSettings, DataBackend};
use hoover3_types::datasource::DatasourceSettings;
use hoover3_types::datasource::DatasourceUiRow;
use hoover3_types::docker_health::*;
use hoover3_types::filesystem::FsMetadata;
use hoover3_types::identifier::*;
use hoover3_types::tasks::DatasourceScanRequest;

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
    dioxus_logger::tracing::info!("before_call: {function} {argument_val}");

    // {
    //     let f2 = function.to_string();
    //     let a2 = argument_val.to_string();
    //     use_drop(move || {
    //         warn!("dropped!");
    //     _after_call(
    //         &f2,
    //         &a2,
    //           &"dropped!".to_string(),
    //            false, current_time(),
    //             current_time()-ts
    //         );
    // });}
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

macro_rules! server_wrapper {
    ($ns:path,$id:ident,$arg:ty,$ret:ty) => {
        ::paste::paste! {
            #[server]
            pub async fn [<__ $id __>](c: $arg)
            -> Result<$ret, ServerFnError>
            {
                let rv = $ns::$id(c)
                .await
                .map_err(|e| ServerFnError::new(
                    format!("{}: {e}", stringify!($id))));

                // ::dioxus_logger::tracing::trace!("server fn {} returned : {:#?}", stringify!($id),  rv);
                rv
            }

            /// UI wrapper over a server function $id
            pub async fn $id(c: $arg)
            -> Result<$ret, ServerFnError>
            {
                let arg_str = format!("{c:?}");
                let arg_str = truncate(&arg_str, 1024);
                let t0 = current_time();
                _before_call(stringify!($id), arg_str, t0);
                let rv = [<__ $id __>](c).await;
                let t1 = current_time();
                let ret_str = format!("{rv:#?}");
                let ret_str = truncate(&ret_str, 1024);
                _after_call(stringify!($id), arg_str, ret_str, rv.is_ok(), t1, t1-t0);
                rv
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

//hoover3_database::client_query::list_disk,
server_wrapper!(
    hoover3_data_access::data_access::file_system_access,
    list_directory,
    PathBuf,
    Vec<FsMetadata>
);

server_wrapper!(
    hoover3_data_access::data_access,
    list_directory_server,
    (DataBackend, PathBuf),
    Vec<FsMetadata>
);

server_wrapper!(
    hoover3_database::client_query::data_access_settings,
    create_or_update_data_access_settings,
    DataAccessSettings,
    String
);

server_wrapper!(
    hoover3_database::client_query::data_access_settings,
    get_data_access_settings,
    (),
    DataAccessSettings
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
