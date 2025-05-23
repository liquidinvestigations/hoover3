//! Front-end component for displaying tables of data.
use std::collections::BTreeMap;

use dioxus::prelude::*;
use hoover3_types::db_schema::{DynamicQueryResponse, DynamicQueryResult};

/// Trait for displaying a data row in a table. The column names and their types are known at compile time.
pub trait DataRowDisplay: serde::Serialize + for<'a> serde::Deserialize<'a> {
    /// Get the headers of the table. These must be known at compile time.
    fn get_headers() -> Vec<&'static str> {
        // we have serde so we can poll the field names from there
        serde_aux::serde_introspection::serde_introspect::<Self>().to_vec()
    }
    /// Render a cell in the table. Default implementation dumps the row as JSON and reads the column.
    fn render_cell(&self, header_name: &str) -> Element {
        use crate::errors::AnyhowErrorDioxusExt;
        // we have serde so we can dump to json and read the column. very inefficient
        let j = serde_json::to_value(self).throw()?;
        let j = j.get(header_name).throw()?;
        let j = format!("{j}");

        rsx!("{j}")
    }
    /// Check if a column can be edited.
    /// These must be known at compile time.
    /// Default implementation is false.
    fn can_edit(_header_name: &str) -> bool {
        false
    }
    /// Get the values of the editable fields in the row.
    /// This is required to populate forms to edit the row.
    fn get_editable_fields(&self) -> BTreeMap<String, String> {
        BTreeMap::new()
    }
    /// Set the editable fields on the row.
    /// This function is used to update a row from form values (input and textarea html elements).
    fn set_editable_fields(&mut self, _h: BTreeMap<String, String>) {}
}

/// Props for the HtmlTable component:
/// - title: The title of the table.
/// - data: The data to display.
/// - extra: An optional extra column with a header and a callback to render on each row.
/// - extra_buttons: An optional callback to render extra elements at the top of the table.
///
#[derive(PartialEq, Props, Clone)]
pub struct HtmlTableProps_<T: 'static + Clone + PartialEq + DataRowDisplay> {
    title: String,
    data: ReadOnlySignal<Vec<T>>,
    extra: Option<(&'static str, Callback<T, Element>)>,
    extra_buttons: Option<Callback<(), Element>>,
}

/// Component that displays a table of data, using the DataRowDisplay trait for rendering.
/// The data is passed in as a vector of rows.
///
/// The component includes a filter function that uses the Debug trait and simple string matching.
#[component]
pub fn HtmlTable<T: 'static + Clone + PartialEq + std::fmt::Debug + DataRowDisplay>(
    props: HtmlTableProps_<T>,
) -> Element {
    let mut headers = T::get_headers();
    if let Some((h, _)) = props.extra {
        headers.push(h);
    }
    let mut search_query = use_signal(|| "".to_string());
    let mut filtered_data = use_signal(|| Vec::<T>::new());
    use_effect(move || {
        let s = search_query.read().clone();
        let s = s.trim().to_lowercase();
        if s.len() >= 2 {
            filtered_data.set(
                props
                    .data
                    .read()
                    .iter()
                    .filter(|x| format!("{x:?}").to_lowercase().contains(&s))
                    .cloned()
                    .collect::<Vec<_>>(),
            )
        } else {
            filtered_data.set(props.data.read().clone())
        }
    });
    rsx! {
        article {
            div { class: "grid",
                h2 { "{props.title}" }
                div {
                    class:"container",
                    div {
                        role: "group",

                        input {
                            placeholder: "Filter",
                            "aria-label": "Filter",
                            name: "search",
                            r#type: "search",
                            oninput: move |_ev| {
                                search_query.set(_ev.value().trim().to_string());
                            },
                        }
                        {props.extra_buttons.map(|cb| cb.call(()))}
                    }
                }
            }

            div {
                class:"overflow-auto",
                table { class: "striped",
                    thead {
                        for k in headers.iter() {
                            th { {k} }
                        }
                    }
                    tbody {
                        for item in filtered_data.read().iter() {
                            tr {
                                for k in T::get_headers().into_iter() {
                                    td {
                                        {T::render_cell(item, k)}
                                    }
                                }
                                if let Some((_, cb)) = props.extra {
                                    td {
                                        {cb.call(item.clone())}
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if filtered_data.read().is_empty() {
                if search_query.read().len() <= 1 {
                    p {
                        i { "No data. " }
                    }
                } else {
                    p {
                        i { "No data for filter '{search_query}'" }
                    }
                }
            }
        }
    }
}

/// Props for the InfoCard component:
/// - title: The title of the card.
/// - data: The data to display.
/// - edited_cb: An optional callback to call when the data is edited.
#[derive(PartialEq, Props, Clone)]
pub struct InfoCardProps_<T: 'static + Clone + PartialEq + DataRowDisplay> {
    title: ReadOnlySignal<Element>,
    data: ReadOnlySignal<Option<T>>,
    edited_cb: Option<Callback<T>>,
}

/// Component that displays a card with a title,
/// fields for a single row, and a form to edit the data if possible.
#[component]
pub fn InfoCard<T: 'static + Clone + PartialEq + std::fmt::Debug + DataRowDisplay>(
    props: InfoCardProps_<T>,
) -> Element {
    let headers = T::get_headers();
    let editable_headers = headers
        .iter()
        .filter(|x| T::can_edit(x))
        .cloned()
        .collect::<Vec<_>>();
    let have_editable = headers.iter().any(|e| T::can_edit(e)) && props.edited_cb.is_some();
    let mut new_value = use_signal(BTreeMap::new);
    use_effect(move || {
        new_value.set(
            props
                .data
                .read()
                .as_ref()
                .map(|i| i.get_editable_fields())
                .unwrap_or_default(),
        );
    });

    let mut edit_mode = use_signal(move || false);

    rsx! {
        article {
            div {
                class: "grid",
                h1 {
                    {props.title}
                }
                if have_editable {
                    button {
                        onclick: move |_| {
                            let editing = *edit_mode.peek();
                            if editing {
                                // set input values
                                if let Some(mut v) = props.data.peek().as_ref().cloned() {
                                    let v0 = v.clone();
                                    v.set_editable_fields(new_value.peek().clone());
                                    if v0 != v {
                                        if let Some(cb) = props.edited_cb {
                                            cb.call(v);
                                        }
                                    }
                                }
                            }
                            edit_mode.set(!editing);
                        },
                        disabled: props.data.read().is_none(),
                        if *edit_mode.read() {"Done"} else {"Edit"}
                    }
                }
            }
            for header in headers {
                div {
                    class: "grid container",
                    style:"min-height:3.5rem;",
                    "{header}: ",
                    if editable_headers.contains(&header) && *edit_mode.read() {
                        input {
                            style:"padding:0;margin:0;",
                            placeholder: "new {header}..",
                            "aria-label": "new {header}...",
                            name: "new_{header}",
                            value: "{new_value.read().get(header).cloned().unwrap_or_default()}",
                            oninput: move |_ev| {
                                let str_val = _ev.value();
                                new_value.write().insert(header.to_string(), str_val);
                            }
                        }
                    } else if let Some(i) = props.data.read().as_ref() {
                        code { {i.render_cell(header)} }
                    }
                }
            }

        }
    }
}

/// Table component that displays the result of a dynamic query.
/// The data format is not known at compile time.
/// This specific component also displays metadata like the query execution time.
#[component]
pub fn DynamicTable(data: ReadOnlySignal<DynamicQueryResponse>) -> Element {
    let time_ms = use_memo(move || (data.read().elapsed_seconds * 10000.0).round() / 10.0);

    let result_rows = data
        .read()
        .result
        .as_ref()
        .ok()
        .map(|r| r.rows.len())
        .unwrap_or(0);

    let result_size_kb = use_memo(move || data.read().result_serialized_size_bytes / 1024);
    let error_display = use_memo(move || {
        data.read()
            .result
            .as_ref()
            .err()
            .unwrap_or(&"".to_string())
            .to_string()
    });
    rsx! {
        small {
            style:"display:block;width:max-content;margin:auto; border: 1px solid gray; padding: 5px;",
            "Query returned {result_rows} rows ({result_size_kb} KB) after {time_ms}ms"
        }
        if let Some(result) = data.read().result.as_ref().ok() {
            DynamicTableInner{data: result.clone()}
        } else {
            pre {
                "DynamicQueryResponse is Error: \n\n{error_display}"
            }
        }
    }
}

/// Table component that displays the result of a dynamic query.
/// The data format is not known at compile time.
///
/// This is the inner component that is used by DynamicTable.
/// Renders only to a <table> element.
#[component]
pub fn DynamicTableInner(data: ReadOnlySignal<DynamicQueryResult>) -> Element {
    rsx! {
        div {
            class:"overflow-auto",
            table { class: "striped",
                thead {
                    for (i, k) in data.read().columns.iter().enumerate() {
                        th { key: "{k.0.clone()},{k.1}{i}", {k.0.clone()} br{} pre{"{k.1}"} }
                    }
                }
                tbody {
                    for row in data.read().rows.iter() {
                        tr {
                            for col in row.iter() {
                                td {
                                    {
                                        if let Some(col) = col {
                                            let col = format!("{}", col);
                                            if col.contains("\n") {
                                                rsx!{pre{"{col}"}}
                                            } else {
                                                rsx!{"{col}"}
                                            }
                                        } else {rsx!()}
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
