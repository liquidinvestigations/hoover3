use dioxus::prelude::*;
use hoover3_types::db_schema::GraphEdgeSchemaDynamic;
use hoover3_types::identifier::CollectionId;

use crate::components::search::context::SearchParams;
use crate::api::get_graph_schema;
use dioxus::logger::tracing;

#[component]
pub fn DocumentPreviewDisplay() -> Element {
    let search_params = use_context::<SearchParams>();
    let selected_id = search_params.selected_id;
    let selected_collection_id = search_params.selected_collection_id;
    let selected_table_type = search_params.selected_table_type;

    let graph_schema = use_resource(move || async move {
        match get_graph_schema(()).await {
            Ok(schema) => Some(schema),
            Err(e) => {
                tracing::error!("Failed to fetch graph schema: {:?}", e);
                None
            }
        }
    });

    rsx! {
        div {
            class: "document-preview",
            style: "padding: 1rem; display: flex; flex-direction: column; gap: 1rem;",

            DocumentPreviewHeader {
                selected_id: selected_id,
                selected_collection_id: selected_collection_id,
                selected_table_type: selected_table_type
            }

            if let Some(Some(schema)) = graph_schema.read().as_ref() {
                GraphSchemaDisplay {
                    schema: schema.clone()
                }
            }
        }
    }
}

/// Component for displaying document preview header with selected item information
#[component]
fn DocumentPreviewHeader(
    selected_id: ReadOnlySignal<Option<String>>,
    selected_collection_id: ReadOnlySignal<Option<CollectionId>>,
    selected_table_type: ReadOnlySignal<Option<String>>
) -> Element {
    rsx! {
        if selected_id.read().is_some() {
            div {
                class: "preview-info",
                style: "background-color: #f8fafc; padding: 1rem; border-radius: 0.375rem;",

                h3 {
                    style: "font-size: 1.25rem; font-weight: 600; margin-bottom: 0.5rem;",
                    "Document Information"
                }

                table {
                    style: "width: 100%; border-collapse: collapse;",
                    tr {
                        td { style: "font-weight: 600; padding: 0.5rem; border-bottom: 1px solid #e2e8f0;", "Selected ID:" }
                        td { style: "padding: 0.5rem; border-bottom: 1px solid #e2e8f0;", {selected_id.read().clone().unwrap_or_default()} }
                    }

                    tr {
                        td { style: "font-weight: 600; padding: 0.5rem; border-bottom: 1px solid #e2e8f0;", "Collection ID:" }
                        td {
                            style: "padding: 0.5rem; border-bottom: 1px solid #e2e8f0;",
                            {
                                if let Some(collection_id) = selected_collection_id.read().as_ref() {
                                    collection_id.to_string()
                                } else {
                                    "None".to_string()
                                }
                            }
                        }
                    }

                    tr {
                        td { style: "font-weight: 600; padding: 0.5rem; border-bottom: 1px solid #e2e8f0;", "Table Type:" }
                        td {
                            style: "padding: 0.5rem; border-bottom: 1px solid #e2e8f0;",
                            {selected_table_type.read().clone().unwrap_or_else(|| "None".to_string())}
                        }
                    }
                }
            }
        } else {
            div {
                class: "no-selection",
                style: "
                    display: flex;
                    justify-content: center;
                    align-items: center;
                    height: 100%;
                    color: #64748b;
                ",
                "No document selected"
            }
        }
    }
}

/// Component for displaying the graph schema information
#[component]
fn GraphSchemaDisplay(schema: GraphEdgeSchemaDynamic) -> Element {
    rsx! {
        div {
            class: "graph-schema",
            style: "background-color: #f8fafc; padding: 1rem; border-radius: 0.375rem;",

            h3 {
                style: "font-size: 1.25rem; font-weight: 600; margin-bottom: 0.5rem;",
                "Graph Schema"
            }

            pre {
                style: "max-height: 200px; overflow-y: auto; padding: 0.5rem; background-color: #f1f5f9; border-radius: 0.25rem;",
                "{schema:#?}"
            }
        }
    }
}
