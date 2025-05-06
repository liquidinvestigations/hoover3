use dioxus::prelude::*;
use hoover3_types::{
    db_schema::{DatabaseValue, DynamicQueryResult},
    identifier::CollectionId,
};

use crate::api::search_facet_query;

/// Full-screen layout for the search page.
#[component]
pub fn SearchFullscreenLayout() -> Element {
    rsx! {
        div { id: "hero", class: "full-height",
            div { class: "search-grid-container full-height",
                div { class: "sidebar-left debug-border",
                    SearchSidebarLeft {}
                }
                div { class: "sidebar-right debug-border",
                    SearchSidebarRight {}
                }
                div { class: "sidebar-bottom debug-border",
                    SearchSidebarBottom {}
                }
                div { class: "search-main debug-border",
                    SearchMain {}
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
                id: "facet-{value}",
            }
            label {
                r#for: "facet-{value}",
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

#[component]
fn SearchSidebarLeft() -> Element {
    let facets_res = use_resource(move || async move {
        let collection_id = CollectionId::new("test1").unwrap();
        let result = search_facet_query((
            collection_id,
            "*".to_string(),
            vec!["*".to_string()], // TODO: Get actual facet fields from schema
            10,
        ))
        .await;
        result
    });

    let result = use_memo(move || {
        let facets_res = facets_res.read().as_ref().cloned();
        facets_res.map(|f| f.map(move |f| f.result))
    });
    let result_ok = use_memo(move || {
        let result = result.read().as_ref().cloned();
        if let Some(Ok(Ok(result))) = result {
            Some(result.clone())
        } else {
            None
        }
    });

    rsx! {
        div { class: "facets-container",
            style: "
                padding: 1rem;
                overflow-y: auto;
                height: 100%;
            ",
            if let Some(result_ok) = result_ok.read().as_ref() {
                for (col_name, _col_type) in &result_ok.columns {
                    if col_name.starts_with("facet_") {
                        div { class: "facet-group",
                            style: "
                                margin-bottom: 1rem;
                                padding: 0.5rem;
                                border: 1px solid #e2e8f0;
                                border-radius: 0.375rem;
                            ",
                            h3 { class: "facet-title",
                                style: "
                                    font-size: 1rem;
                                    font-weight: 600;
                                    margin-bottom: 0.5rem;
                                ",
                                { col_name.strip_prefix("facet_").unwrap_or(col_name)}
                            }
                            div { class: "facet-values",
                                style: "
                                    display: flex;
                                    flex-direction: column;
                                    gap: 0.25rem;
                                ",
                                {
                                    let mut facet_values = Vec::new();
                                    if let Some(row) = &result_ok.rows.first() {
                                        if let Some(col_index) = &result_ok.columns.iter().position(|(name, _)| name == col_name) {
                                            if let Some(Some(DatabaseValue::Object(values))) = row.get(*col_index) {
                                                for (value, count) in values {
                                                    if let Some(count) = count {
                                                        facet_values.push((value.clone(), count.clone()));
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    facet_values.sort_by(|a, b| b.1.to_string().cmp(&a.1.to_string()));
                                    rsx! {
                                        for (value, count) in facet_values {
                                            FacetValue { value: value.clone(), count: count.clone() }
                                        }
                                    }
                                }
                            }
                        }
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
fn SearchSidebarRight() -> Element {
    rsx! {
        div {
            for i in 0..100 {
                div {
                    "Result {i}"
                }
            }

        }
    }
}

#[component]
fn SearchSidebarBottom() -> Element {
    rsx! {
        div {

        }
    }
}

#[component]
fn SearchMain() -> Element {
    rsx! {
        div {
            style: "
                display: flex;
                flex-direction: column; flex: 1;
                overflow: hidden; height:100%;
            ",
            div {
                style: "height: 4rem;width: 100%;",
                class: "debug-border",
                SearchInput {}
            }
            div {
                style: "
                    flex: 1;
                    height: 100%; width: 100%;
                    overflow:auto;
                ",
                class: "debug-border",
                SearchResults {}
            }
        }
    }
}

#[component]
fn SearchResults() -> Element {
    rsx! {
        div {
            for i in 0..100 {
                div {
                    "Result {i}"
                }
            }
        }
    }
}
#[component]
fn SearchInput() -> Element {
    rsx! {
        div {
            role: "group",
            input {
                placeholder: "...",
            }
        }
    }
}
