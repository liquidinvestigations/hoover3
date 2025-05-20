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
}

#[component]
pub fn SearchContext(children: Element) -> Element {
    let mut search_text = use_signal(|| String::new());
    let mut selected_id = use_signal(|| None::<String>);
    let mut selected_collections = use_signal(|| HashMap::new());

    let search_text_write = Callback::new(move |s: String| {
        search_text.set(s);
    });

    let selected_id_write = Callback::new(move |s: Option<String>| {
        selected_id.set(s);
    });

    let selected_collections_write = Callback::new(move |(id, selected): (CollectionId, bool)| {
        selected_collections.write().insert(id, selected);
    });

    use_context_provider(|| SearchParams {
        search_q: search_text.into(),
        search_q_write: search_text_write.into(),
        selected_id: selected_id.into(),
        selected_id_write: selected_id_write.into(),
        selected_collections: selected_collections.into(),
        selected_collections_write: selected_collections_write.into(),
    });

    rsx! {
        {children}
    }
}
