//! This module contains the table definitions for the collections table.

use charybdis::macros::charybdis_model;
use charybdis::types::{BigInt, Text};

/// Database representation of a collection in the system.
/// This struct maps directly to a row in the collections table on the common `hoover3`` keyspace.
#[charybdis_model(
    table_name = seekstorm_index_info,
    partition_keys = [collection_id],
    clustering_keys = [],
    global_secondary_indexes = [],
    local_secondary_indexes = [],
    static_columns = []
)]
#[derive(
    Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub struct SeekstormIndexInfo {
    /// Unique identifier for the collection
    pub collection_id: Text,
    /// Seekstorm API key for this index
    pub seekstorm_api_key: Text,
    /// Seekstorm index id for this index
    pub seekstorm_index_id: BigInt,
}
