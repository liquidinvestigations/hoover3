//! Left sidebar components for search page.

use async_std::stream::StreamExt;
use dioxus::prelude::*;
use hoover3_types::{
    collection::CollectionUiRow,
    db_schema::{DatabaseServiceType, DatabaseValue, DynamicQueryResponse, DynamicQueryResult},
    identifier::CollectionId,
};
use std::collections::HashMap;

use crate::api::{get_all_collections, search_facet_query};
use crate::components::search::context::SearchParams;

/// Component with left sidebar
#[component]
pub fn SearchSidebarLeft() -> Element {
    rsx! {
        div { class: "sidebar-left-container",
            style: "
                display: flex;
                flex-direction: column;
                height: 100%;
            ",
            CollectionSelector {}
            FacetsList {}
        }
    }
}

/// Represents an aggregated facet value across multiple collections
#[derive(Clone, Debug, PartialEq)]
struct AggregatedFacetValue {
    /// The facet value
    value: String,
    /// Total count across all collections
    total_count: i64,
}

/// Represents an aggregated facet field across multiple collections
#[derive(Clone, Debug, PartialEq)]
struct AggregatedFacet {
    /// The facet field name (without the "facet_" prefix)
    name: String,
    /// The aggregated values for this facet
    values: Vec<AggregatedFacetValue>,
}

/// Aggregated facets data structure
#[derive(Clone, Debug, PartialEq)]
struct AggregatedFacets {
    /// The aggregated facets
    facets: Vec<AggregatedFacet>,
}

impl AggregatedFacets {
    fn new() -> Self {
        Self { facets: Vec::new() }
    }
}

#[component]
fn CollectionSelector() -> Element {
    let search_params = use_context::<SearchParams>();
    let collections_res = use_resource(move || async move { get_all_collections(()).await });

    // Initialize collections as selected when they are first loaded
    let collections = use_memo(move || {
        let collections_res = collections_res.read().as_ref().cloned();
        if let Some(Ok(collections)) = collections_res {
            for collection in &collections {
                search_params
                    .selected_collections_write
                    .call((collection.collection_id.clone(), true));
            }
            collections
        } else {
            vec![]
        }
    });

    // Compute collection data with selection states
    let collection_data = use_memo(move || {
        let collections = collections.read();
        let r = collections
            .iter()
            .map(|collection| {
                let id = collection.collection_id.clone();
                let selected = search_params
                    .selected_collections
                    .read()
                    .get(&id)
                    .copied()
                    .unwrap_or(false);
                (collection.clone(), selected, id)
            })
            .collect::<Vec<_>>();
        r
    });

    rsx! {
        div { class: "collection-selector",
            style: "
                height: 20%;
                padding: 1rem;
                border-bottom: 1px solid #e2e8f0;
                overflow-y: auto;
            ",
            h3 { class: "collection-title",
                style: "
                    font-size: 1rem;
                    font-weight: 600;
                    margin-bottom: 0.5rem;
                ",
                "Collections"
            }
            div { class: "collection-list",
                style: "
                    display: flex;
                    flex-direction: column;
                    gap: 0.25rem;
                ",
                    for (collection, selected, id) in collection_data.read().clone() {
                        div { class: "collection-item",
                            style: "
                                display: flex;
                                align-items: center;
                                gap: 0.5rem;
                            ",
                            input {
                                r#type: "checkbox",
                                id: format!("collection-{}", id.clone()),
                                checked: selected,
                                onchange: {let id = id.clone(); move |e: Event<FormData>| {
                                    search_params.selected_collections_write.call((id.clone(), e.value().parse().unwrap_or(false)));
                                }}
                            }
                            label {
                                r#for: format!("collection-{}", id.clone()),
                                {collection.collection_title.clone()}
                            }
                        }
                    }
            }
        }
    }
}

#[component]
fn FacetValue(value: String, count: DatabaseValue) -> Element {
    let value = use_memo(move || value.clone());
    let count = use_memo(move || format!("{}", count));
    rsx! {
        div { class: "facet-value",
            style: "
                display: flex;
                align-items: center;
                gap: 0.5rem;
            ",
            input {
                r#type: "checkbox",
                id: format!("facet-{}", value),
            }
            label {
                r#for: format!("facet-{}", value),
                style: "
                    display: flex;
                    justify-content: space-between;
                    width: 100%;
                ",
                span { {value} }
                span { class: "facet-count",
                    style: "
                        color: #64748b;
                        font-size: 0.875rem;
                    ",
                    {count}
                }
            }
        }
    }
}

/// Fetches facets from all selected collections
async fn fetch_facets(
    selected_collections: Vec<CollectionId>,
    search_q: String,
) -> Result<Vec<DynamicQueryResult>, ServerFnError> {
    if selected_collections.is_empty() {
        return Ok(Vec::new());
    }

    let search_q = if search_q.is_empty() {
        "*".to_string()
    } else {
        search_q
    };

    // Fetch facets for each collection
    let mut all_facets = Vec::new();
    for collection_id in selected_collections {
        let result = search_facet_query((
            collection_id.clone(),
            search_q.clone(),
            vec!["*".to_string()], // TODO: Get actual facet fields from schema
            10,
        ))
        .await?;

        if let Ok(result) = result.result {
            all_facets.push((collection_id, result));
        }
    }

    Ok(all_facets.into_iter().map(|(_, result)| result).collect())
}

/// Aggregates facet results from multiple collections
fn aggregate_facets(results: Vec<DynamicQueryResult>) -> AggregatedFacets {
    let mut aggregated = AggregatedFacets::new();
    let mut facet_map: HashMap<String, HashMap<String, AggregatedFacetValue>> = HashMap::new();

    // Process each result set
    for result in results {
        // Find facet columns
        for (col_name, _) in &result.columns {
            if col_name.starts_with("facet_") {
                let facet_name = col_name
                    .strip_prefix("facet_")
                    .unwrap_or(col_name)
                    .to_string();

                // Get or create the facet's value map
                let value_map = facet_map
                    .entry(facet_name.clone())
                    .or_insert_with(HashMap::new);

                // Process each row's facet values
                if let Some(row) = result.rows.first() {
                    if let Some(col_index) =
                        result.columns.iter().position(|(name, _)| name == col_name)
                    {
                        if let Some(Some(DatabaseValue::Object(values))) = row.get(col_index) {
                            for (value, count) in values {
                                if let Some(count) = count {
                                    let count = count.to_string().parse::<i64>().unwrap_or(0);
                                    let value = value.to_string();

                                    // Update or create the aggregated value
                                    let aggregated_value = value_map
                                        .entry(value.clone())
                                        .or_insert_with(|| AggregatedFacetValue {
                                            value: value.clone(),
                                            total_count: 0,
                                        });

                                    aggregated_value.total_count += count;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Convert the map into the final structure
    for (name, values) in facet_map {
        let mut facet_values: Vec<AggregatedFacetValue> = values.into_values().collect();
        // Sort by total count descending
        facet_values.sort_by(|a, b| b.total_count.cmp(&a.total_count));

        aggregated.facets.push(AggregatedFacet {
            name,
            values: facet_values,
        });
    }

    // Sort facets by name
    aggregated.facets.sort_by(|a, b| a.name.cmp(&b.name));
    aggregated
}

#[component]
fn FacetsList() -> Element {
    let search_params = use_context::<SearchParams>();
    let mut facets_signal = use_signal(|| None::<Vec<DynamicQueryResult>>);

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

                let result = fetch_facets(selected_collections, search_q).await;
                facets_signal.set(result.ok());
                crate::time::sleep(std::time::Duration::from_millis(60)).await;
            }
        }
    });

    // Trigger coroutine when search params change
    use_effect(move || {
        let _ = search_params.search_q.read();
        let _ = search_params.selected_collections.read();
        _coroutine.send(());
    });

    let aggregated_facets = use_memo(move || {
        if let Some(results) = facets_signal.read().as_ref() {
            Some(aggregate_facets(results.clone()))
        } else {
            None
        }
    });

    rsx! {
        div { class: "facets-container",
            style: "
                padding: 1rem;
                overflow-y: auto;
                flex: 1;
            ",
            FacetsControlDisplay { facets: aggregated_facets }
        }
    }
}

#[component]
fn FacetsControlDisplay(facets: ReadOnlySignal<Option<AggregatedFacets>>) -> Element {
    rsx! {
        div { class: "facets-control",
            style: "
                display: flex;
                flex-direction: column;
                gap: 1rem;
            ",
            if let Some(facets) = facets.read().as_ref() {
                if facets.facets.is_empty() {
                    div { class: "no-facets",
                        style: "
                            display: flex;
                            justify-content: center;
                            align-items: center;
                            height: 100%;
                            color: #64748b;
                        ",
                        "No facets available"
                    }
                } else {
                    for facet in &facets.facets {
                        FacetDisplay { facet: facet.clone() }
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
                    "Loading facets..."
                }
            }
        }
    }
}

#[component]
fn FacetDisplay(facet: AggregatedFacet) -> Element {
    rsx! {
        div { class: "facet-group",
            style: "
                margin-bottom: 0.7rem;
                padding: 0.7rem;
                border: 1px solid #e2e8f0;
                border-radius: 0.375rem;
            ",
            h3 { class: "facet-title",
                style: "
                    font-size: 1rem;
                    font-weight: 600;
                    margin-bottom: 0.5rem;
                ",
                {facet.name.clone()}
            }
            div { class: "facet-values",
                style: "
                    display: flex;
                    flex-direction: column;
                    gap: 0.25rem;
                    max-height: 300px;
                    overflow-y: auto;
                ",
                for value in &facet.values {
                    FacetValueDisplay {
                        facet_name: facet.name.clone(),
                        value: value.clone()
                    }
                }
            }
        }
    }
}

#[component]
fn FacetValueDisplay(facet_name: String, value: AggregatedFacetValue) -> Element {
    rsx! {
        div { class: "facet-value",
            style: "
                display: flex;
                align-items: center;
                gap: 0.5rem;
            ",
            input {
                r#type: "checkbox",
                style: "min-width: 1rem; min-height: 1rem;",
                id: format!("facet-{}-{}", facet_name, value.value),
            }
            label {
                r#for: format!("facet-{}-{}", facet_name, value.value),
                style: "
                    display: flex;
                    justify-content: space-between;
                    width: 100%;
                ",
                span { {value.value.clone()} }
                span { class: "facet-count",
                    style: "
                        color: #64748b;
                        font-size: 0.875rem;
                    ",
                    {value.total_count.to_string()}
                }
            }
        }
    }
}
