//! Search results component.

use dioxus::prelude::*;
use futures_util::StreamExt;
use hoover3_types::{
    db_schema::{DatabaseServiceType, DynamicQueryResponse, DynamicQueryResult},
    identifier::CollectionId,
};
use std::collections::HashMap;

use crate::{
    api::db_explorer_run_query,
    components::search::context::SearchParams,
};

/// Represents a search result with its collection ID and data
#[derive(Clone, Debug, PartialEq)]
struct SearchResult {
    /// The collection ID this result came from
    collection_id: CollectionId,
    /// The data for this result
    data: HashMap<String, String>,
}

/// Fetches search results from all selected collections
async fn fetch_search_results(selected_collections: Vec<CollectionId>, search_q: String) -> Result<Vec<SearchResult>, ServerFnError> {
    if selected_collections.is_empty() {
        return Ok(Vec::new());
    }

    let search_q = if search_q.is_empty() { "*".to_string() } else { search_q };

    // Fetch results for each collection
    let mut all_results = Vec::new();
    for collection_id in selected_collections {
        let result = db_explorer_run_query((
            collection_id.clone(),
            DatabaseServiceType::Meilisearch,
            search_q.clone(),
        ))
        .await?;

        if let Ok(result) = result.result {
            // Convert each row into a SearchResult
            for row in result.rows {
                let mut data = HashMap::new();
                for (i, value) in row.iter().enumerate() {
                    if let Some(col_name) = result.columns.get(i).map(|(name, _)| name) {
                        if let Some(value) = value {
                            data.insert(col_name.clone(), format!("{}", value));
                        }
                    }
                }
                all_results.push(SearchResult {
                    collection_id: collection_id.clone(),
                    data,
                });
            }
        }
    }

    Ok(all_results)
}


/// Search results component.
#[component]
pub fn SearchResults() -> Element {
    let search_params = use_context::<SearchParams>();
    let mut results_signal = use_signal(|| None::<Vec<SearchResult>>);

    let _coroutine = use_coroutine(move |mut _r: UnboundedReceiver<()>| {
        let search_params = search_params.clone();

        async move {
            while let Some(_) = _r.next().await {
                while let Ok(Some(_)) = _r.try_next() {
                    // skip
                }
                // Get selected collections from context
                let selected_collections: Vec<CollectionId> = search_params
                    .selected_collections
                    .read()
                    .iter()
                    .filter(|(_, selected)| **selected)
                    .map(|(id, _)| id.clone())
                    .collect();
                let search_q = search_params.search_q.read().clone();

                let result = fetch_search_results(selected_collections, search_q).await;
                results_signal.set(result.ok());
            }
        }
    });

    // Trigger coroutine when search params change
    use_effect(move || {
        let _ = search_params.search_q.read();
        let _ = search_params.selected_collections.read();
        _coroutine.send(());
    });

    rsx! {
        div { class: "search-results",
            style: "
                display: flex;
                flex-direction: column;
                gap: 1rem;
                padding: 1rem;
            ",
            if let Some(results) = results_signal.read().as_ref() {
                if results.is_empty() {
                    div { class: "no-results",
                        style: "
                            display: flex;
                            justify-content: center;
                            align-items: center;
                            height: 100%;
                            color: #64748b;
                        ",
                        "No results found"
                    }
                } else {
                    for result in results {
                        SearchResultDisplay { result: result.clone() }
                    }
                }
            } else {
                div { class: "loading",
                    style: "
                        display: flex;
                        justify-content: center;
                        align-items: center;
                        height: 100%;
                        color: #64748b;
                    ",
                    "Loading results..."
                }
            }
        }
    }
}


#[component]
fn SearchResultDisplay(result: SearchResult) -> Element {
    rsx! {
        div { class: "search-result",
            style: "
                padding: 1rem;
                border: 1px solid #e2e8f0;
                border-radius: 0.375rem;
            ",
            div { class: "result-header",
                style: "
                    font-weight: 600;
                    margin-bottom: 0.5rem;
                    color: #64748b;
                ",
                "Collection: {result.collection_id}"
            }
            div { class: "result-data",
                style: "
                    display: flex;
                    flex-direction: column;
                    gap: 0.5rem;
                ",
                for (key, value) in &result.data {
                    div { class: "result-field",
                        style: "
                            display: flex;
                            gap: 0.5rem;
                        ",
                        span { class: "field-name",
                            style: "
                                font-weight: 500;
                                color: #64748b;
                            ",
                            "{key}:"
                        }
                        span { class: "field-value",
                            "{value}"
                        }
                    }
                }
            }
        }
    }
}