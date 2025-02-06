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

    let placeholder = use_memo(move || match db_type.read().clone() {
        DatabaseServiceType::Scylla => "SELECT count(*) FROM ...",
        DatabaseServiceType::Meilisearch => "John Smith ...",
        DatabaseServiceType::Nebula => "YIELD rand32(1, 6);",
    });

    rsx! {
        div {
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
    let links = use_memo(move || {
        let c_id = collection_id.read().clone();
        let db_type = db_type.read().clone();
        if let Some(schema) = schema.read().as_ref() {
            get_sidebar_links(&c_id, &db_type, schema)
        } else {
            Default::default()
        }
    });
    rsx! {
        QueryToolSidebarLinksDisplay {
            links
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
struct QueryToolSidebarLinks {
    top_links: Vec<(String, Route)>,
    per_table_links: Vec<(String, Vec<(String, Route)>)>,
}

fn get_sidebar_links(
    collection_id: &CollectionId,
    db_type: &DatabaseServiceType,
    schema: &CollectionSchema,
) -> QueryToolSidebarLinks {
    let make_link = |text: &str, query: &str| {
        (
            text.to_string(),
            Route::DatabaseExplorerPage {
                explorer_route: DatabaseExplorerRoute::QueryToolPage {
                    collection_id: collection_id.clone(),
                    db_type: db_type.clone(),
                    query_state: SqlQueryToolState {
                        query: query.to_string(),
                    },
                }
                .into(),
            },
        )
    };

    match db_type {
        DatabaseServiceType::Scylla => {
            let tables = schema.scylla.tables.keys().cloned().collect::<Vec<_>>();

            let mut per_table_links = vec![];
            for table in tables {
                let table_name = &table.to_string();
                per_table_links.push((
                    table_name.to_string(),
                    vec![
                        make_link("SELECT *", &format!("SELECT * FROM {};", table_name)),
                        make_link("COUNT", &format!("SELECT COUNT(*) FROM {};", table_name)),
                        make_link("DESCRIBE", &format!("DESCRIBE {};", table_name)),
                    ],
                ));
            }
            QueryToolSidebarLinks {
                top_links: vec![
                    make_link(
                        "Scylla Version",
                        "SELECT version FROM system.versions LIMIT 1",
                    ),
                    make_link("Large Cells", "SELECT * FROM system.large_cells"),
                    make_link("Large Partitions", "SELECT * FROM system.large_partitions"),
                    make_link("Large Rows", "SELECT * FROM system.large_rows"),
                    make_link(
                        "View Builds",
                        "SELECT * FROM system.views_builds_in_progress",
                    ),
                ],
                per_table_links,
            }
        }

        DatabaseServiceType::Meilisearch => QueryToolSidebarLinks {
            top_links: vec![make_link("test", "test"), make_link("1234", "1234")],
            per_table_links: vec![],
        },
        DatabaseServiceType::Nebula => QueryToolSidebarLinks {
            top_links: vec![
                make_link("throw dice", "YIELD rand32(1,7);"),
                make_link("show sessions", "SHOW SESSIONS;"),
            ],
            per_table_links: vec![],
        },
    }
}

#[component]
fn QueryToolSidebarLinksDisplay(links: ReadOnlySignal<QueryToolSidebarLinks>) -> Element {
    rsx! {
        ul {
            for (top_link, top_route) in links.read().top_links.iter() {
                li {
                    Link { to: top_route.clone(), "{top_link}" }
                }
            }
            for (table_name, table_links) in links.read().per_table_links.iter() {
                li {
                    h5 { "{table_name}" }
                    ul {
                        for (link_text, link_route) in table_links.iter() {
                            li { Link { to: link_route.clone(), "{link_text}" } }
                        }
                    }
                }
            }
        }
    }
}
