//! This module contains the table definitions for the collections table.

use charybdis::macros::charybdis_model;
use charybdis::types::{Text, Timestamp, Int};

/// Database representation of a collection in the system.
/// This struct maps directly to a row in the collections table on the common `hoover3`` keyspace.
#[charybdis_model(
    table_name = collections,
    partition_keys = [collection_id],
    clustering_keys = [],
    global_secondary_indexes = [],
    local_secondary_indexes = [],
    static_columns = []
)]
#[derive(
    Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub struct CollectionDbRow {
    /// Unique identifier for the collection
    pub collection_id: Text,
    /// Title/name of the collection
    pub collection_title: Text,
    /// Detailed description of the collection's contents and purpose
    pub collection_description: Text,
    /// Timestamp when the collection was initially created
    pub time_created: Timestamp,
    /// Timestamp of the most recent modification to the collection
    pub time_modified: Timestamp,
}

use hoover3_types::collection::CollectionUiRow;
use hoover3_types::identifier::CollectionId;
impl CollectionDbRow {
    /// Convert a `CollectionDbRow` to a `CollectionUiRow`.
    pub fn to_ui(&self) -> anyhow::Result<CollectionUiRow> {
        Ok(CollectionUiRow {
            collection_id: CollectionId::new(&self.collection_id)?,
            collection_title: self.collection_title.clone(),
            collection_description: self.collection_description.clone(),
            time_created: self.time_created,
            time_modified: self.time_modified,
        })
    }
}
