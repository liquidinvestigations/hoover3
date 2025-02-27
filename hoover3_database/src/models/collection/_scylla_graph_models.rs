//! This module contains the table definitions for graph edge-related tables.

use charybdis::macros::charybdis_model;
use charybdis::types::{Counter, Text, Int, Boolean};



/// Tracks the number of pages for a primary key, edge type, and direction.
#[charybdis_model(
    table_name = graph_edge_pages_counter,
    partition_keys = [pk_source],
    clustering_keys = [edge_type, direction_out],
    global_secondary_indexes = [],
    local_secondary_indexes = [],
    static_columns = []
)]
#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub struct GraphEdgePagesCounter {
    /// Primary key string
    pub pk_source: Text,
    /// The type of edge
    pub edge_type: Text,
    /// Edge Direction - true = OUT, false = IN
    pub direction_out: Boolean,
    /// Counter for the number of edges
    pub item_count: Counter,
}


/// Tracks the content for a page of edges.
#[charybdis_model(
    table_name = graph_edge_page,
    partition_keys = [pk_source, edge_type, direction_out, page_id],
    clustering_keys = [pk_target],
    global_secondary_indexes = [],
    local_secondary_indexes = [],
    static_columns = []
)]
#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub struct GraphEdgePage {
    /// Primary key string
    pub pk_source: Text,
    /// The type of edge
    pub edge_type: Text,
    /// Edge Direction - true = OUT, false = IN
    pub direction_out: Boolean,
    /// Page identifier
    pub page_id: Int,
    /// Secondary primary key string
    pub pk_target: Text,
}


/// Maps a node primary key to a value.
#[charybdis_model(
    table_name = graph_node_pk_map,
    partition_keys = [pk],
    clustering_keys = [],
    global_secondary_indexes = [],
    local_secondary_indexes = [],
    static_columns = []
)]
#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub struct GraphNodePkMap {
    /// Primary key string
    pub pk: Text,
    /// Value of the model primary key and clustering keys, as json
    pub value: Text,
}
