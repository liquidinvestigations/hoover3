use hoover3_macro::udt_model;
use hoover3_types::db_schema::Timestamp;
use serde::Serialize;

/// Documentation
#[udt_model]
pub struct SimpleModelUdt {
    /// Some Field
    pub id: String,
    /// Other Field
    pub another_field: Option<i32>,
    /// Timestamp field
    pub created_at: Timestamp,
}