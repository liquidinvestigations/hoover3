use hoover3_macro::model;
use hoover3_types::db_schema::Timestamp;
use serde::Serialize;

/// Documentation
#[model]
pub struct SimpleModel {
    /// Primary key field
    #[model(primary(partition))]
    pub id: String,
    /// Nullable Field
    pub created_at: Option<Timestamp>,
}