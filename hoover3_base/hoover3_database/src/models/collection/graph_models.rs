//! This module contains the table definitions for graph edge-related tables.

use charybdis::macros::charybdis_model;
use charybdis::types::{Boolean, Counter, Frozen, Int, Text, Tuple};

/// Tracks the number of edges across all pages for a primary key, edge type, and direction.
/// Useful to know the total page count for this parameter combination.
#[charybdis_model(
    table_name = graph_edge_pages_counter,
    partition_keys = [pk_source],
    clustering_keys = [edge_type, direction_out],
)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub struct GraphEdgeSourceCounter {
    /// Primary key string
    pub pk_source: Text,
    /// The type of edge
    pub edge_type: Text,
    /// Edge Direction - true = OUT, false = IN
    pub direction_out: Boolean,
    /// Counter for the number of edges
    pub item_count: Counter,
}

/// Edge data keyed by page - each partition contains a page of edges sorted by target_pk.
/// Useful for listing through all edges for a given source and type.
#[charybdis_model(
    table_name = graph_edge_page_content,
    partition_keys = [pk_source, edge_type, direction_out, page_id],
    clustering_keys = [pk_target],
)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub struct GraphEdgePageContent {
    /// Source primary key string
    pub pk_source: Text,
    /// The type of edge
    pub edge_type: Text,
    /// Edge Direction - true = OUT, false = IN
    pub direction_out: Boolean,
    /// Page identifier
    pub page_id: Int,
    /// Target primary key string
    pub pk_target: Text,
}

/// Edge data pages - a list of all the pages for a given source, type, direction
#[charybdis_model(
    table_name = graph_edge_page,
    partition_keys = [pk_source, edge_type, direction_out],
    clustering_keys = [page_id],
)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub struct GraphEdgePageList {
    /// Source primary key string
    pub pk_source: Text,
    /// The type of edge
    pub edge_type: Text,
    /// Edge Direction - true = OUT, false = IN
    pub direction_out: Boolean,
    /// Page identifier
    pub page_id: Int,
}

/// Edge data keyed by source and target primary keys.
/// Useful to get the page id for a given source and target, or to check if an edge exists.
#[charybdis_model(
    table_name = graph_edge_page_assignment,
    partition_keys = [edge_pks],
    clustering_keys = [edge_type, direction_out, page_id],
)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub struct GraphEdgePageAssignment {
    /// Source and target primary keys, in a single column, to allow IN queries
    pub edge_pks: Frozen<Tuple<Text, Text>>,
    /// The type of edge
    pub edge_type: Text,
    /// Edge Direction - true = OUT, false = IN
    pub direction_out: Boolean,
    /// Page identifier
    pub page_id: Int,
}

/// Maps a node primary key to a value.
#[charybdis_model(
    table_name = graph_node_pk_map,
    partition_keys = [pk],
    clustering_keys = [],
)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub struct GraphNodePkMap {
    /// Primary key string
    pub pk: Text,
    /// Value of the model primary key and clustering keys, as json
    pub value: Text,
}
