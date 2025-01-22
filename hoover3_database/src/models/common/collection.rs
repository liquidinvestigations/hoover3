use charybdis::macros::charybdis_model;
use charybdis::types::{Text, Timestamp};

#[charybdis_model(
    table_name = collections,
    partition_keys = [collection_id],
    clustering_keys = [],
    global_secondary_indexes = [],
    local_secondary_indexes = [],
    static_columns = []
)
]
#[derive(
    Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub struct CollectionDbRow {
    pub collection_id: Text,
    pub collection_title: Text,
    pub collection_description: Text,
    pub time_created: Timestamp,
    pub time_modified: Timestamp,
}

use hoover3_types::collection::CollectionUiRow;
impl From<CollectionDbRow> for CollectionUiRow {
    fn from(value: CollectionDbRow) -> Self {
        Self {
            collection_id: value.collection_id,
            collection_title: value.collection_title,
            collection_description: value.collection_description,
            time_created: value.time_created,
            time_modified: value.time_modified,
        }
    }
}
