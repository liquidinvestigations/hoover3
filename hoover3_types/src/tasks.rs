use crate::datasource::DatasourceSettings;
use crate::identifier::CollectionId;
use crate::identifier::DatabaseIdentifier;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub struct DatasourceScanRequest {
    pub collection_id: CollectionId,
    pub datasource_id: DatabaseIdentifier,
    pub settings: DatasourceSettings,
}
