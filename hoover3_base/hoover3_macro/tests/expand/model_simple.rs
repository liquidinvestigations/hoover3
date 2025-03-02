use hoover3_macro::model;
use hoover3_types::db_schema::Timestamp;
use serde::Serialize;

/// Documentation
#[model]
pub struct SimpleModel {
    /// Primary key field
    #[model(primary(partition))]
    pub id: String,
    /// Other Field
    #[model(primary(clustering))]
    pub other_field: i64,
    /// Another field
    #[model(primary(partition))]
    pub another_field: i32,
    /// Timestamp field
    pub created_at: Timestamp,
}