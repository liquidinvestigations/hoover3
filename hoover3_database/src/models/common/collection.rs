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
use hoover3_types::identifier::CollectionId;
impl CollectionDbRow {
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
