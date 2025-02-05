use std::time::Duration;

use crate::{
    api::{db_explorer_run_query, get_collection_schema},
    components::{make_page_title, DynamicTable},
    pages::DatabaseExplorerRoute,
    routes::Route,
};
use dioxus::prelude::*;
use dioxus_sdk::utils::timing::use_debounce;
use futures_util::StreamExt;
use hoover3_types::{
    db_schema::{CollectionSchema, DatabaseServiceType},
    identifier::CollectionId,
};

/// State for the SQL query tool, the part kept in the URL.
#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize, Default)]
pub struct SqlQueryToolState {
    /// SQL query as text
    pub query: String,
}

/// Page that displays the SQL query tool.
#[component]
pub fn DatabaseExplorerQueryToolPage(
    collection_id: ReadOnlySignal<CollectionId>,
    db_type: ReadOnlySignal<DatabaseServiceType>,
    query_state: ReadOnlySignal<SqlQueryToolState>,
) -> Element {
    let mut sql_editor = use_signal(move || query_state.peek().query.clone());
    let mut query_result = use_signal(|| None);

    let run_sql = use_coroutine(move |mut rx: UnboundedReceiver<String>| async move {
        while let Some(sql_query) = rx.next().await {
            if sql_query.trim().is_empty() {
                continue;
            }
            let result = db_explorer_run_query((
                collection_id.peek().clone(),
                db_type.peek().clone(),
                sql_query,
            ))
            .await;
            query_result.set(Some(result));
        }
    });

    use_effect(move || {
        run_sql.send(query_state.read().query.clone());
        sql_editor.set(query_state.read().query.clone());
    });

    let mut do_search = use_debounce(Duration::from_millis(50), move |_| {
        let new_query = sql_editor.peek().clone();
        // if link is the same, navigation will not trigger.
        if query_state.peek().query == new_query {
            run_sql.send(new_query.clone());
        }
        navigator().replace(Route::DatabaseExplorerPage {
            explorer_route: DatabaseExplorerRoute::QueryToolPage {
                collection_id: collection_id.read().clone(),
                db_type: db_type.read().clone(),
                query_state: SqlQueryToolState { query: new_query },
            }
            .into(),
        });
    });

    let placeholder = use_memo(move || {
        match db_type.read().clone() {
            DatabaseServiceType::Scylla => "SELECT count(*) FROM ...",
            DatabaseServiceType::Meilisearch => "John Smith ...",
            DatabaseServiceType::Nebula => "YIELD rand32(1, 6);",
        }
    });

    rsx! {
        div { class: "container-fluid",
        style: "display: grid; grid-template-columns: 1fr 6fr;",

        article {
            h3 {"Jump"}
            SqlQueryToolJumpLinks {collection_id, db_type}
        }
        article {
            style: "max-width: 80vw;",
            h1 {
                Link {
                    to: Route::DatabaseExplorerPage{
                        explorer_route: DatabaseExplorerRoute::RootPage.into()
                    },
                    "Database Explorer"
                }
                " > "
                Link {
                    to: Route::DatabaseExplorerPage{
                        explorer_route: DatabaseExplorerRoute::CollectionPage {
                            collection_id: collection_id.read().clone()
                        }.into()
                    },
                    {make_page_title(0, "collection", &collection_id.to_string())}
                }
                " > "
                "{db_type:?}"
            }
            div {
                role: "group",
                textarea {
                    placeholder: "{placeholder}",
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
                    "ServerFnError: {e}"
                }
            }
        }}
    }
}

#[component]
fn SqlQueryToolJumpLinks(
    collection_id: ReadOnlySignal<CollectionId>,
    db_type: ReadOnlySignal<DatabaseServiceType>,
) -> Element {
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
    match db_type.read().clone() {
        DatabaseServiceType::Scylla => {
            rsx! {
                ScyllaQueryJumpLinks {collection_id, schema}
            }
        }
        DatabaseServiceType::Meilisearch => {
            rsx! {
                MeilisearchQueryJumpLinks {collection_id, schema}
            }
        }
        DatabaseServiceType::Nebula => {
            rsx! {
                NebulaQueryJumpLinks {collection_id, schema}
            }
        }
    }
}
#[component]
fn ScyllaQueryJumpLinks(
    collection_id: ReadOnlySignal<CollectionId>,
    schema: ReadOnlySignal<Option<CollectionSchema>>,
) -> Element {
    let tables = use_memo(move || {
        schema
            .read()
            .as_ref()
            .map(|s| s.scylla.tables.keys().cloned().collect::<Vec<_>>())
            .unwrap_or(vec![])
    });

    type QueryFn = Box<dyn Fn(String) -> String>;
    let queries: Vec<(&str, QueryFn)> = vec![
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
                                    to: Route::DatabaseExplorerPage{
                                        explorer_route: DatabaseExplorerRoute::QueryToolPage {
                                            collection_id: collection_id.read().clone(),
                                            db_type: DatabaseServiceType::Scylla,
                                            query_state: SqlQueryToolState {
                                            query: query_str(table.to_string()),
                                            }
                                        }.into()
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

#[component]
fn MeilisearchQueryJumpLinks(
    collection_id: ReadOnlySignal<CollectionId>,
    schema: ReadOnlySignal<Option<CollectionSchema>>,
) -> Element {
    rsx! {
        "TODO Meilisearch"
    }
}

#[component]
fn NebulaQueryJumpLinks(
    collection_id: ReadOnlySignal<CollectionId>,
    schema: ReadOnlySignal<Option<CollectionSchema>>,
) -> Element {
    rsx! {
        "TODO Nebula"
    }
}
