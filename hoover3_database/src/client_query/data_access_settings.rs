use hoover3_types::data_access::DataAccessSettings;
use crate::models::common::data_access_settings::DataAccessSettingsDbRow;
use crate::db_management::ScyllaDatabaseHandle;
use crate::db_management::DatabaseSpaceManager;
use charybdis::operations::Insert;
use anyhow::Result;
use tracing::{info, warn};

const DATA_ACCESS_SETTINGS_ID: &str = "1";

pub async fn create_or_update_data_access_settings(
    settings: DataAccessSettings,
) -> Result<String> {
    info!("create_or_update_data_access_settings settings={:?}", settings);
    let session = ScyllaDatabaseHandle::global_session().await?;
    let settings_serialized = serde_json::to_string(&settings)?;
    let now = chrono::offset::Utc::now();

    let row = DataAccessSettingsDbRow {
        data_access_settings_id: DATA_ACCESS_SETTINGS_ID.to_string(),
        settings: settings_serialized,
        time_created: now,
        time_modified: now,
    };

    let result = DataAccessSettingsDbRow::insert(&row).execute(&session).await;
    match result {
        Ok(_) => {
            Ok(DATA_ACCESS_SETTINGS_ID.to_string())
        },
        Err(e) => {
            warn!("Error creating data access settings: {:?}", e);
            Err(anyhow::anyhow!("Error creating data access settings"))
        }
    }
}

pub async fn get_data_access_settings(_: ()) -> Result<DataAccessSettings> {
    let session = ScyllaDatabaseHandle::global_session().await?;
    let row = DataAccessSettingsDbRow::find_by_data_access_settings_id(DATA_ACCESS_SETTINGS_ID.to_string())
        .execute(&session)
        .await?;
    Ok(row.to_data_access_settings())
}
