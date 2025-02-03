use dioxus::prelude::*;
use dioxus_elements::mover;
use hoover3_types::{db_schema::{CollectionSchema, MeilisearchDatabaseSchema, NebulaDatabaseSchema, ScyllaDatabaseSchema}, identifier::{CollectionId, DatabaseIdentifier}};

use crate::{api::{get_all_collections, get_collection_schema}, errors::AnyhowErrorDioxusExt, routes::Route};

#[component]
fn LinkCard(title: String, link: Route, children: Element) -> Element {
    rsx! {
        Link {
            to: link,
            article {
                style: "max-width:500px; min-height: 300px;",
                h3 { {title} }
                {children}
            }
        }
    }
}

#[component]
fn  CardGridDisplay(children: Element) -> Element {
    rsx! {
        div {
            style: "display: grid; grid-template-columns: repeat(auto-fill, minmax(400px, 600px)); gap: 10px; min-height: 300px;",
            {children}
        }
    }
}


#[component]
pub fn DatabaseExplorerRootPage() -> Element {
    let collections_res = use_resource(move || {
        async move {
            get_all_collections(()).await
        }
    });
    rsx! {
        h1 { "Database Explorer" }
        CardGridDisplay {
            if let Some(Ok(collections)) = collections_res.read().as_ref() {
                for collection in collections {
                    LinkCard {
                        title: format!("collection `{}`", collection.collection_id.to_string()),
                        link: Route::DatabaseExplorerCollectionPage { collection_id: collection.collection_id.clone() },
                        CollectionStatsInfoCard{collection_id: collection.collection_id.clone()}
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
        async move {
            get_collection_schema(c2).await
        }
    });
    let schema = use_memo(move || {
        if let Some(Ok(schema)) = schema_res.read().as_ref() {
            Some(schema.clone())
        } else {
            None
        }
    });
    let search_doc_count = use_memo(move || {
        schema.read().as_ref().map(|x| x.meilisearch.doc_count).unwrap_or(0)
    });
    let table_count = use_memo(move || {
        schema.read().as_ref().map(|x| x.scylla.tables.len()).unwrap_or_default()
    });
    let column_count = use_memo(move || {
        schema.read().as_ref().map(|x| x.scylla.tables.values().map(|x| x.columns.len()).sum::<usize>()).unwrap_or_default()
    });

    rsx! {
        article {
            style: "max-width:500px;",
            p { "Table Count: {table_count}" }
            p { "Column Count: {column_count}" }
            p { "Search Doc Count: {search_doc_count}" }
        }
    }
}

#[component]
pub fn DatabaseExplorerCollectionPage(collection_id: String) -> Element {
    let c = CollectionId::new(&collection_id).throw()?;
    let collection_id = use_signal(move || c);
    let schema_res = use_resource(move || {
        let collection_id = collection_id.read().clone();
        async move {
            get_collection_schema(collection_id).await
        }
    });
    let schema = use_memo(move || {
        if let Some(Ok(schema)) = schema_res.read().as_ref() {
            Some(schema.clone())
        } else {
            None
        }
    });

    let sql_schema = use_memo(move || {
        schema.read().as_ref().map(|x| x.scylla.clone())
    });
    let graph_schema = use_memo(move || {
        schema.read().as_ref().map(|x| x.nebula.clone())
    });
    let search_schema = use_memo(move || {
        schema.read().as_ref().map(|x| x.meilisearch.clone())
    });

    rsx! {
        h1 {
            Link {
                to: Route::DatabaseExplorerRootPage{},
                "Database Explorer"
            }
            " > "
                "collection `{collection_id}`"
        }
        h2 { "SQL Tables"}
        CardGridDisplay {
            SQLTableCards{collection_id, sql_schema}
        }
        h2 { "Graph Nodes"}
        CardGridDisplay {
            GraphNodesCards{collection_id, graph_schema}
        }
        h2 { "Graph Edges"}
        CardGridDisplay {
            GraphEdgesCards{collection_id, graph_schema}
        }
        h2 { "Search Index"}
        CardGridDisplay {
            SearchIndexCards{collection_id, search_schema}
        }
    }
}

#[component]
fn SQLTableCards(collection_id: ReadOnlySignal<CollectionId>, sql_schema: ReadOnlySignal<Option<ScyllaDatabaseSchema>>) -> Element {
    rsx! {
        if let Some(schema) = sql_schema.read().as_ref() {
            for table in schema.tables.values() {
                LinkCard {
                    title: format!("table `{}`", table.name.to_string()),
                    link: Route::DatabaseExplorerCollectionPageSqlTable { collection_id: collection_id.read().clone(), table_name: table.name.clone() },
                    div {
                        "..."
                    }
                }
            }
        }
    }
}


#[component]
fn GraphNodesCards(collection_id: ReadOnlySignal<CollectionId>, graph_schema: ReadOnlySignal<Option<NebulaDatabaseSchema>>) -> Element {
    rsx! {
        if let Some(schema) = graph_schema.read().as_ref() {
            for tag in schema.tags.values() {
                LinkCard {
                    title: format!("tag `{}`", tag.name.to_string()),
                    link: Route::DatabaseExplorerCollectionPageGraphNodes { collection_id: collection_id.read().clone(), tag_name: tag.name.clone() },
                    div {
                        "..."
                    }
                }
            }
        }
    }
}

#[component]
fn GraphEdgesCards(collection_id: ReadOnlySignal<CollectionId>, graph_schema: ReadOnlySignal<Option<NebulaDatabaseSchema>>) -> Element {
    rsx! {
        if let Some(schema) = graph_schema.read().as_ref() {
            for edge in schema.edges.iter() {
                LinkCard {
                    title: format!("edge `{}`", edge.name.to_string()),
                    link: Route::DatabaseExplorerCollectionPageGraphEdges {
                        collection_id: collection_id.read().clone(), edge_name: edge.name.clone() },
                    div {
                        "..."
                    }
                }
            }
        }
    }
}

#[component]
fn SearchIndexCards(collection_id: ReadOnlySignal<CollectionId>, search_schema: ReadOnlySignal<Option<MeilisearchDatabaseSchema>>) -> Element {
    rsx! {
        if let Some(schema) = search_schema.read().as_ref() {
            for field in schema.fields.iter() {
                LinkCard {
                    title: format!("field `{:?}`", field),
                    link: Route::DatabaseExplorerCollectionPageSearchIndex {
                        collection_id: collection_id.read().clone(), field_name: field.0.clone() },
                    div {
                        "..."
                    }
                }
            }
        }
    }
}

#[component]
pub fn DatabaseExplorerCollectionPageSqlTable(collection_id: CollectionId, table_name: DatabaseIdentifier) -> Element {
    rsx! {
        div {
            "SQLTablePage"
        }
    }
}

#[component]
pub fn DatabaseExplorerCollectionPageGraphNodes(collection_id: CollectionId, tag_name: DatabaseIdentifier) -> Element {
    rsx! {
        div {
            "SQLTablePage"
        }
    }
}
#[component]
    pub fn DatabaseExplorerCollectionPageGraphEdges(collection_id: CollectionId, edge_name: DatabaseIdentifier) -> Element {
    rsx! {
        div {
            "SQLTablePage"
        }
    }
}
#[component]
pub fn DatabaseExplorerCollectionPageSearchIndex(collection_id: CollectionId, field_name: String) -> Element {
    rsx! {
        div {
            "SQLTablePage"
        }
    }
}