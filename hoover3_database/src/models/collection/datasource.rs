//! This module contains the table definitions for all the datasource related models.

use crate::impl_model_callbacks;
use charybdis::macros::charybdis_model;
use charybdis::types::{Text, Timestamp};
use hoover3_macro::model;
use hoover3_types::datasource::DatasourceUiRow;
use hoover3_types::identifier::CollectionId;
use hoover3_types::identifier::DatabaseIdentifier;
use serde::Serialize;
// #[model(primary(partition))]

/// Database representation of a datasource in the system.
/// This struct maps directly to a row in the datasources table on the collection keyspace.

// #[model]
// pub struct DatasourceDbRow {
//     /// Unique identifier for the datasource
//     #[model(primary(partition))]
//     pub datasource_id: String,
//     /// Type of the datasource
//     pub datasource_type: String,
//     /// Settings for the datasource
//     pub datasource_settings: String,
//     /// Timestamp when the datasource was initially created
//     pub time_created: Timestamp,
//     /// Timestamp of the most recent modification to the datasource
//     pub time_modified: Timestamp,
// }
#[charybdis_model(
    table_name = datasource,
    partition_keys = [datasource_id],
    clustering_keys = [],
    global_secondary_indexes = [],
    local_secondary_indexes = [],
    static_columns = []
)]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct DatasourceDbRow {
    /// Unique identifier for the datasource
    pub datasource_id: Text,
    /// Type of the datasource
    pub datasource_type: Text,
    /// Settings for the datasource
    pub datasource_settings: Text,
    /// Timestamp when the datasource was initially created
    pub time_created: Timestamp,
    /// Timestamp of the most recent modification to the datasource
    pub time_modified: Timestamp,
}

impl DatasourceDbRow {
    /// Convert a `DatasourceDbRow` to frontend representation.
    pub fn to_ui_row(self, collection_id: &CollectionId) -> DatasourceUiRow {
        DatasourceUiRow {
            collection_id: collection_id.clone(),
            datasource_id: DatabaseIdentifier::new(self.datasource_id).unwrap(),
            datasource_type: self.datasource_type,
            datasource_settings: serde_json::from_str(&self.datasource_settings).unwrap(),
            time_created: self.time_created,
            time_modified: self.time_modified,
        }
    }
}

impl_model_callbacks!(DatasourceDbRow);
