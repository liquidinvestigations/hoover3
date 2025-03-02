use hoover3_macro::{model, udt_model};
use hoover3_types::db_schema::Timestamp;
use serde::Serialize;

/// Documentation
#[udt_model]
pub struct simple_model_udt {
    /// Some Field
    pub id: String,
    /// Other Field
    pub another_field: Option<i32>,
    /// Timestamp field
    pub created_at: Timestamp,
}


/// Documentation
#[model]
pub struct SimpleModelUdtWithTable {
    /// Some Field
    #[model(primary(partition))]
    pub id: String,
    /// Other Field
    pub another_field: Option<simple_model_udt>,
    /// The Field
    pub the_field: simple_model_udt,
}