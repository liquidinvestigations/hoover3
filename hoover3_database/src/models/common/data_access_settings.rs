use charybdis::macros::charybdis_model;
use charybdis::types::{Text, Timestamp};
use hoover3_types::data_access::DataAccessSettings;

#[charybdis_model(
    table_name = data_access_settings,
    partition_keys = [data_access_settings_id],
    clustering_keys = [],
    global_secondary_indexes = [],
    local_secondary_indexes = [],
    static_columns = []
)]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct DataAccessSettingsDbRow {
    pub data_access_settings_id: Text,
    pub settings: Text,
    pub time_created: Timestamp,
    pub time_modified: Timestamp,
}

impl DataAccessSettingsDbRow {
    pub fn to_data_access_settings(&self) -> DataAccessSettings {
        serde_json::from_str(&self.settings).unwrap()
    }
}
