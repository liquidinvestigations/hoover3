#[derive(
    Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub struct CollectionUiRow {
    pub collection_id: String,
    pub collection_title: String,
    pub collection_description: String,
    pub time_created: chrono::DateTime<chrono::Utc>,
    pub time_modified: chrono::DateTime<chrono::Utc>,
}
