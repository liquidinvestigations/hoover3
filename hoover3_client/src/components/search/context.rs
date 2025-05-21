//! Components related to search page context.

use dioxus::prelude::*;
use hoover3_types::identifier::CollectionId;
use std::collections::HashMap;

/// Context for search page - holds state and callbacks for search.
#[derive(Clone, PartialEq, Copy)]
pub struct SearchParams {
    /// The current search text.
    pub search_q: ReadOnlySignal<String>,
    /// Callback to update the search text.
    pub search_q_write: Callback<String>,
    /// The currently selected ID.
    pub selected_id: ReadOnlySignal<Option<String>>,
    /// Callback to update the selected ID.
    pub selected_id_write: Callback<Option<String>>,
    /// Map of collection IDs to their selection state
    pub selected_collections: ReadOnlySignal<HashMap<CollectionId, bool>>,
    /// Callback to update collection selection state
    pub selected_collections_write: Callback<(CollectionId, bool)>,
    /// The currently selected collection ID
    pub selected_collection_id: ReadOnlySignal<Option<CollectionId>>,
    /// Callback to update the selected collection ID
    pub selected_collection_id_write: Callback<Option<CollectionId>>,
    /// The currently selected table type
    pub selected_table_type: ReadOnlySignal<Option<String>>,
    /// Callback to update the selected table type
    pub selected_table_type_write: Callback<Option<String>>,
}

#[component]
pub fn SearchContext(children: Element) -> Element {
    let mut search_text = use_signal(|| String::new());
    let mut selected_id = use_signal(|| None::<String>);
    let mut selected_collections = use_signal(|| HashMap::new());
    let mut selected_collection_id = use_signal(|| None::<CollectionId>);
    let mut selected_table_type = use_signal(|| None::<String>);

    let search_text_write = Callback::new(move |s: String| {
        search_text.set(s);
    });

    let selected_id_write = Callback::new(move |s: Option<String>| {
        selected_id.set(s);
    });

    let selected_collections_write = Callback::new(move |(id, selected): (CollectionId, bool)| {
        selected_collections.write().insert(id, selected);
    });

    let selected_collection_id_write = Callback::new(move |id: Option<CollectionId>| {
        selected_collection_id.set(id);
    });

    let selected_table_type_write = Callback::new(move |table_type: Option<String>| {
        selected_table_type.set(table_type);
    });

    use_context_provider(|| SearchParams {
        search_q: search_text.into(),
        search_q_write: search_text_write.into(),
        selected_id: selected_id.into(),
        selected_id_write: selected_id_write.into(),
        selected_collections: selected_collections.into(),
        selected_collections_write: selected_collections_write.into(),
        selected_collection_id: selected_collection_id.into(),
        selected_collection_id_write: selected_collection_id_write.into(),
        selected_table_type: selected_table_type.into(),
        selected_table_type_write: selected_table_type_write.into(),
    });

    rsx! {
        {children}
    }
}
