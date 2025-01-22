use charybdis::macros::charybdis_model;
use charybdis::types::{Text, Timestamp};
use hoover3_types::datasource::DatasourceUiRow;
use hoover3_types::identifier::CollectionId;
#[charybdis_model(
    table_name = datasource,
    partition_keys = [datasource_id],
    clustering_keys = [],
    global_secondary_indexes = [],
    local_secondary_indexes = [],
    static_columns = []
)]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct DatasourceDbRow {
    pub datasource_id: Text,
    pub datasource_type: Text,
    pub datasource_settings: Text,
    pub time_created: Timestamp,
    pub time_modified: Timestamp,
}
impl DatasourceDbRow {
    pub fn to_ui_row(self, collection_id: &CollectionId) -> DatasourceUiRow {
        DatasourceUiRow {
            collection_id: collection_id.name(),
            datasource_id: self.datasource_id,
            datasource_type: self.datasource_type,
            datasource_settings: self.datasource_settings,
            time_created: self.time_created,
            time_modified: self.time_modified,
        }
    }
}
