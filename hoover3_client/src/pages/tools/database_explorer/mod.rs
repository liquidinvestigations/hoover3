mod sql_query_tool;
pub use sql_query_tool::*;

use dioxus::prelude::*;
use hoover3_types::{
    db_schema::{
        DatabaseServiceType, GraphEdgeSchemaDynamic, MeilisearchDatabaseSchema,
        ScyllaDatabaseSchema,
    },
    identifier::{CollectionId, DatabaseIdentifier},
};

use crate::{
    api::{get_all_collections, query_collection_schema, scylla_row_count},
    components::{
        cards::{CardGridDisplay, LinkCard},
        page_titles::make_page_title,
    },
    errors::AnyhowErrorDioxusExt,
    routes::{Route, UrlParam},
};

/// Router for the database explorer sub-app.
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize, Default)]
pub enum DatabaseExplorerRoute {
    /// Route for the root page of the database explorer.
    #[default]
    RootPage,
    /// Route for a collection page.
    CollectionPage { collection_id: CollectionId },
    /// Route for a SQL table page.
    SqlTablePage {
        collection_id: CollectionId,
        table_name: DatabaseIdentifier,
    },
    /// Route for a graph node page.
    GraphNodesPage {
        collection_id: CollectionId,
        tag_name: DatabaseIdentifier,
    },
    /// Route for a graph edge page.
    GraphEdgesPage {
        collection_id: CollectionId,
        edge_name: DatabaseIdentifier,
    },
    /// Route for a search index page.
    SearchIndexPage {
        collection_id: CollectionId,
        field_name: String,
    },
    /// Route for the SQL query tool page.
    QueryToolPage {
        collection_id: CollectionId,
        db_type: DatabaseServiceType,
        query_state: SqlQueryToolState,
    },
}

/// Main Page that displays the database explorer.
/// Because the UrlParam data is not available immediately after the component is mounted,
/// we need to use a signal to delay the rendering of the page until the UrlParam is available.
#[component]
pub fn DatabaseExplorerPage(
    explorer_route: ReadOnlySignal<UrlParam<DatabaseExplorerRoute>>,
) -> Element {
    let (state, loaded) = UrlParam::convert_signals(explorer_route);

    rsx! {
        if *loaded.read() {
            _DatabaseExplorerPage{
                explorer_route: state.read().clone()
            }
        }
    }
}

/// Internal component that switches between the different pages of the database explorer.
#[component]
fn _DatabaseExplorerPage(explorer_route: ReadOnlySignal<DatabaseExplorerRoute>) -> Element {
    match explorer_route.read().clone() {
        DatabaseExplorerRoute::RootPage => {
            navigator().replace(Route::DatabaseExplorerRootPage {});
            rsx! {DatabaseExplorerRootPage{}}
        }
        DatabaseExplorerRoute::CollectionPage { collection_id } => {
            rsx! {DatabaseExplorerCollectionPage{collection_id}}
        }
        DatabaseExplorerRoute::SqlTablePage {
            collection_id,
            table_name,
        } => rsx! {DatabaseExplorerCollectionPageSqlTable{collection_id, table_name}},
        DatabaseExplorerRoute::GraphNodesPage {
            collection_id,
            tag_name,
        } => rsx! {DatabaseExplorerCollectionPageGraphNodes{collection_id, tag_name}},
        DatabaseExplorerRoute::GraphEdgesPage {
            collection_id,
            edge_name,
        } => rsx! {DatabaseExplorerCollectionPageGraphEdges{collection_id, edge_name}},
        DatabaseExplorerRoute::SearchIndexPage {
            collection_id,
            field_name,
        } => rsx! {DatabaseExplorerCollectionPageSearchIndex{collection_id, field_name}},
        DatabaseExplorerRoute::QueryToolPage {
            collection_id,
            db_type,
            query_state,
        } => rsx! {DatabaseExplorerQueryToolPage{collection_id, db_type, query_state}},
    }
}

/// The root page for the database explorer.
#[component]
pub fn DatabaseExplorerRootPage() -> Element {
    let collections_res = use_resource(move || async move { get_all_collections(()).await });
    rsx! {
        h1 { "Database Explorer" }
        CardGridDisplay {
            if let Some(Ok(collections)) = collections_res.read().as_ref() {
                for collection in collections {
                    LinkCard {
                        subtitle: "collection".to_string(),
                        title: collection.collection_id.to_string(),
                        link: Route::DatabaseExplorerPage{
                            explorer_route: DatabaseExplorerRoute::CollectionPage {
                            collection_id: collection.collection_id.clone()
                        }.into()},
                        CollectionStatsInfoCard{
                            collection_id: collection.collection_id.clone()
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn CollectionStatsInfoCard(collection_id: CollectionId) -> Element {
    let c2 = collection_id.clone();
    let schema_res = use_resource(move || {
        let c2 = c2.clone();
        async move { query_collection_schema(c2).await }
    });
    let schema = use_memo(move || {
        if let Some(Ok(schema)) = schema_res.read().as_ref() {
            Some(schema.clone())
        } else {
            None
        }
    });
    let search_doc_count = use_memo(move || {
        schema
            .read()
            .as_ref()
            .map(|x| x.meilisearch.doc_count)
            .unwrap_or(0)
    });
    let table_count = use_memo(move || {
        schema
            .read()
            .as_ref()
            .map(|x| x.scylla.tables.len())
            .unwrap_or_default()
    });
    let column_count = use_memo(move || {
        schema
            .read()
            .as_ref()
            .map(|x| {
                x.scylla
                    .tables
                    .values()
                    .map(|x| x.columns.len())
                    .sum::<usize>()
            })
            .unwrap_or_default()
    });

    rsx! {
        div {
            style: "max-width:380px;",
            p { "Table Count: {table_count}" }
            p { "Column Count: {column_count}" }
            p { "Search Doc Count: {search_doc_count}" }
        }
    }
}

#[component]
fn DatabaseExplorerCollectionPage(collection_id: String) -> Element {
    let c = CollectionId::new(&collection_id).throw()?;
    let collection_id = use_signal(move || c);
    let schema_res = use_resource(move || {
        let collection_id = collection_id.read().clone();
        async move { query_collection_schema(collection_id).await }
    });
    let schema = use_memo(move || {
        if let Some(Ok(schema)) = schema_res.read().as_ref() {
            Some(schema.clone())
        } else {
            None
        }
    });

    let sql_schema = use_memo(move || schema.read().as_ref().map(|x| x.scylla.clone()));
    let _graph_schema = use_memo(move || schema.read().as_ref().map(|x| x.graph.clone()));
    let search_schema = use_memo(move || schema.read().as_ref().map(|x| x.meilisearch.clone()));

    rsx! {
        h1 {
            Link {
                to: Route::DatabaseExplorerPage{
                    explorer_route: DatabaseExplorerRoute::RootPage.into()
                },
                "Database Explorer"
            }
            " > "
                {make_page_title(0, "collection", &collection_id.to_string())}
        }
        CardGridDisplay {
            LinkCard {
                subtitle: "SQL".to_string(),
                title: "ScyllaDB".to_string(),
                    link: Route::DatabaseExplorerPage{
                    explorer_route: DatabaseExplorerRoute::QueryToolPage{
                        collection_id: collection_id.read().clone(),
                        db_type: DatabaseServiceType::Scylla,
                        query_state: SqlQueryToolState::default()
                    }.into()
                },
                "Freeform Scylla/Cassandra SQL Query"
            }
            LinkCard {
                subtitle: "Index".to_string(),
                title: "Meilisearch".to_string(),
                link: Route::DatabaseExplorerPage{
                    explorer_route: DatabaseExplorerRoute::QueryToolPage{
                        collection_id: collection_id.read().clone(),
                        db_type: DatabaseServiceType::Meilisearch,
                        query_state: SqlQueryToolState::default()
                    }.into()
                },
                "Freeform Search Query"
            }
        }
        h2 { "SQL Tables"}
        CardGridDisplay {
            SQLTableCards{collection_id, sql_schema}
        }
        // h2 { "Graph Nodes"}
        // CardGridDisplay {
        //     GraphNodesCards{collection_id, graph_schema}
        // }
        // h2 { "Graph Edges"}
        // CardGridDisplay {
        //     GraphEdgesCards{collection_id, graph_schema}
        // }
        h2 { "Search Index"}
        CardGridDisplay {
            SearchIndexCards{collection_id, search_schema}
        }
    }
}

#[component]
fn SQLTableCards(
    collection_id: ReadOnlySignal<CollectionId>,
    sql_schema: ReadOnlySignal<Option<ScyllaDatabaseSchema>>,
) -> Element {
    rsx! {
        if let Some(schema) = sql_schema.read().as_ref() {
            for table in schema.tables.values() {
                LinkCard {
                    subtitle: "table".to_string(),
                    title: table.name.to_string(),
                    link: Route::DatabaseExplorerPage{
                        explorer_route: DatabaseExplorerRoute::SqlTablePage {
                            collection_id: collection_id.read().clone(),
                            table_name: table.name.clone()
                        }.into()
                    },
                    div {
                        p { "Row Count: ", SQLRowCountDisplayString{
                            collection_id, table_name: table.name.clone()}}
                        p { "Column Count: "  "{table.columns.len()}" }
                        p {"Primary Key:", {format!(
                            "{:?}",
                            table.columns.iter()
                                .filter(|x| x.primary).map(|x| x.name.to_string())
                                .collect::<Vec<String>>()
                        )}}
                    }
                }
            }
        }
    }
}

#[component]
fn SQLRowCountDisplayString(
    collection_id: ReadOnlySignal<CollectionId>,
    table_name: ReadOnlySignal<DatabaseIdentifier>,
) -> Element {
    let row_count_res = use_resource(move || {
        let collection_id = collection_id.read().clone();
        let table_name = table_name.read().clone();
        async move { scylla_row_count((collection_id, table_name)).await }
    });
    rsx! {
        if let Some(Ok(row_count)) = row_count_res.read().as_ref() {
            {row_count.to_string()}
        }
    }
}

#[component]
fn GraphNodesCards(
    collection_id: ReadOnlySignal<CollectionId>,
    graph_schema: ReadOnlySignal<Option<GraphEdgeSchemaDynamic>>,
) -> Element {
    rsx! {
        // if let Some(schema) = graph_schema.read().as_ref() {
        //     for tag in schema.edges_by_source.values() {
        //         LinkCard {
        //             subtitle: "tag".to_string(),
        //             title: tag.name.to_string(),
        //             link: Route::DatabaseExplorerPage{
        //                 explorer_route: DatabaseExplorerRoute::GraphNodesPage {
        //                     collection_id: collection_id.read().clone(),
        //                     tag_name: tag.name.clone()
        //                 }.into()
        //             },
        //             div {
        //                 p { "Column Count: {tag.columns.len()}" }
        //             }
        //         }
        //     }
        // }
    }
}

#[component]
fn GraphEdgesCards(
    collection_id: ReadOnlySignal<CollectionId>,
    graph_schema: ReadOnlySignal<Option<GraphEdgeSchemaDynamic>>,
) -> Element {
    rsx! {
        // if let Some(schema) = graph_schema.read().as_ref() {
        //     for edge in schema.edges.iter() {
        //         LinkCard {
        //             subtitle: "edge".to_string(),
        //             title: edge.edge_type.to_string(),
        //             link: Route::DatabaseExplorerPage{
        //                 explorer_route: DatabaseExplorerRoute::GraphEdgesPage {
        //                     collection_id: collection_id.read().clone(),
        //                     edge_name: edge.edge_type.clone()
        //                 }.into()
        //             },
        //             div {
        //                 "..."
        //             }
        //         }
        //     }
        // }
    }
}

#[component]
fn SearchIndexCards(
    collection_id: ReadOnlySignal<CollectionId>,
    search_schema: ReadOnlySignal<Option<MeilisearchDatabaseSchema>>,
) -> Element {
    rsx! {
        if let Some(schema) = search_schema.read().as_ref() {
            for field in schema.fields.iter() {
                LinkCard {
                    subtitle: "field".to_string(),
                    title: field.0.clone().replace(":", ": "),
                    link: Route::DatabaseExplorerPage{
                        explorer_route: DatabaseExplorerRoute::SearchIndexPage {
                            collection_id: collection_id.read().clone(),
                            field_name: field.0.clone()
                        }.into()
                    },
                    div {
                        p { "Row Count: {field.1}" }
                    }
                }
            }
        }
    }
}

#[component]
fn DatabaseExplorerHeader(collection_id: CollectionId, title: String) -> Element {
    rsx! {
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
                        collection_id: collection_id.clone()
                    }.into()
                },
                {make_page_title(0, "collection", &collection_id.to_string())}
            }
            " > "
            {title}
        }
    }
}

#[component]
fn DatabaseExplorerCollectionPageSqlTable(
    collection_id: CollectionId,
    table_name: DatabaseIdentifier,
) -> Element {
    rsx! {
        DatabaseExplorerHeader{collection_id, title: &table_name.to_string()}
        div {
            "SQLTablePage"
        }
    }
}

#[component]
fn DatabaseExplorerCollectionPageGraphNodes(
    collection_id: CollectionId,
    tag_name: DatabaseIdentifier,
) -> Element {
    rsx! {
        DatabaseExplorerHeader{collection_id, title: tag_name.to_string()}
        div {
            "GraphNodesPage"
        }
    }
}
#[component]
fn DatabaseExplorerCollectionPageGraphEdges(
    collection_id: CollectionId,
    edge_name: DatabaseIdentifier,
) -> Element {
    rsx! {
        DatabaseExplorerHeader{collection_id, title: edge_name.to_string()}
        div {
            "GraphEdgesPage"
        }
    }
}
#[component]
fn DatabaseExplorerCollectionPageSearchIndex(
    collection_id: CollectionId,
    field_name: String,
) -> Element {
    rsx! {
        DatabaseExplorerHeader{collection_id, title: field_name.to_string()}
        div {
            "SearchIndexPage"
        }
    }
}
