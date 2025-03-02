//! Types and structures related to collections.

use crate::identifier::CollectionId;

/// UI representation of a collection with metadata
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub struct CollectionUiRow {
    /// Unique identifier for the collection
    pub collection_id: CollectionId,
    /// User-facing Display Title of the collection
    pub collection_title: String,
    /// User-facing Description of the collection's contents
    pub collection_description: String,
    /// Timestamp when the collection was first created
    pub time_created: chrono::DateTime<chrono::Utc>,
    /// Timestamp of the most recent modification to the collection
    pub time_modified: chrono::DateTime<chrono::Utc>,
}
