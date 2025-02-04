use std::time::Duration;

use crate::{
    api::{
        db_explorer_run_scylla_query, get_collection_schema,
    },
    components::{make_page_title, DynamicTable},
    routes::{Route, UrlParam},
};
use dioxus::prelude::*;
use dioxus_sdk::utils::timing::use_debounce;
use futures_util::StreamExt;
use hoover3_types::identifier::CollectionId;

#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize, Default)]
pub struct ScyllaQueryToolState {
    pub query: String,
}

#[component]
pub fn DatabaseExplorerSqlQueryToolPage(
    collection_id: ReadOnlySignal<CollectionId>,
    query_state: ReadOnlySignal<UrlParam<ScyllaQueryToolState>>,
) -> Element {
    let (state, loaded) = UrlParam::convert_signals(query_state);

    rsx! {
        if *loaded.read() {
            _DatabaseExplorerSqlQueryToolPage{
                collection_id: collection_id.read().clone(),
                state: state.read().clone()
            }
        }
    }
}

#[component]
fn _DatabaseExplorerSqlQueryToolPage(
    collection_id: ReadOnlySignal<CollectionId>,
    state: ReadOnlySignal<ScyllaQueryToolState>,
) -> Element {
    let mut sql_editor = use_signal(move || state.peek().query.clone());
    let mut query_result = use_signal(|| None);

    let run_sql = use_coroutine(move |mut rx: UnboundedReceiver<String>| async move {
        while let Some(sql_query) = rx.next().await {
            if sql_query.trim().is_empty() {
                continue;
            }
            let result =
                db_explorer_run_scylla_query((collection_id.peek().clone(), sql_query)).await;
            query_result.set(Some(result));
        }
    });

    use_effect(move || {
        run_sql.send(state.read().query.clone());
        sql_editor.set(state.read().query.clone());
    });

    let mut do_search = use_debounce(Duration::from_millis(50), move |_| {
        let new_query = sql_editor.peek().clone();
        // if link is the same, navigation will not trigger.
        if state.peek().query == new_query {
            run_sql.send(new_query.clone());
        }
        navigator().replace(Route::DatabaseExplorerSqlQueryToolPage {
            collection_id: collection_id.read().clone(),
            query_state: UrlParam::new(ScyllaQueryToolState { query: new_query }),
        });
    });

    rsx! {
        div { class: "container-fluid",
        style: "display: grid; grid-template-columns: 1fr 6fr;",

        article {
            h3 {"Jump"}
            ScyllaQueryJumpLinks {collection_id}
        }
        article {
            h1 {
                Link {
                    to: Route::DatabaseExplorerRootPage{},
                    "Database Explorer"
                }
                " > "
                Link {
                    to: Route::DatabaseExplorerCollectionPage { collection_id: collection_id.read().clone() },
                    {make_page_title(0, "collection", &collection_id.to_string())}
                }
                " > "
                "SQL Query Tool"
            }
            div {
                role: "group",
                textarea {
                    placeholder: "SELECT count(*) FROM ...",
                    value: "{sql_editor}",
                    oninput: move |_ev| {
                        let query  = _ev.value().to_string();
                        sql_editor.set(query);

                    },
                }
                button {
                    style: "margin: 5px;",
                    onclick: move |_ev| {
                        do_search.action(());
                    },
                    "Run" br{} "Query"
                }
            }

            if let Some(Ok(result)) = query_result.read().as_ref() {
                DynamicTable { data: result.clone() }
            }
            if let Some(Err(e)) = query_result.read().as_ref() {
                pre {
                    "Error: {e:#?}"
                }
            }
        }}
    }
}

#[component]
fn ScyllaQueryJumpLinks(collection_id: ReadOnlySignal<CollectionId>) -> Element {
    let schema_res = use_resource(move || {
        let collection_id = collection_id.read().clone();
        async move { get_collection_schema(collection_id).await }
    });
    let schema = use_memo(move || {
        schema_res
            .read()
            .as_ref()
            .and_then(|s| s.as_ref().ok())
            .cloned()
    });
    let tables = use_memo(move || {
        schema
            .read()
            .as_ref()
            .map(|s| s.scylla.tables.keys().cloned().collect::<Vec<_>>())
            .unwrap_or(vec![])
    });
    let queries: Vec<(&str, Box<dyn Fn(String) -> String>)> = vec![
        (
            "SELECT *",
            Box::new(move |x| format!("SELECT * FROM {};", x)),
        ),
        (
            "COUNT",
            Box::new(move |x| format!("SELECT COUNT(*) FROM {};", x)),
        ),
        ("DESCRIBE", Box::new(move |x| format!("DESCRIBE {};", x))),
    ];
    rsx! {
        ul {
            for table in tables.read().iter() {
                li {
                    key: "{table}",
                    h5 { {table.to_string()} }
                    ul {
                        for (query, query_str) in queries.iter() {
                            li {
                                Link {
                                    to: Route::DatabaseExplorerSqlQueryToolPage {
                                        collection_id: collection_id.read().clone(),
                                        query_state: UrlParam::new(ScyllaQueryToolState {
                                            query: query_str(table.to_string()),
                                            ..Default::default()
                                        })
                                    },
                                    "{query}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
