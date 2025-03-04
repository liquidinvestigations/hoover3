/*
 * SeekStorm REST API documentation
 *
 * Search engine library & multi-tenancy server
 *
 * The version of the OpenAPI document: 0.12.11
 * Contact: wolf.garbe@seekstorm.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// FacetFilterOneOf11String : String filter
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FacetFilterOneOf11String {
    /// field name
    #[serde(rename = "field")]
    pub field: String,
    /// filter: array of facet string values
    #[serde(rename = "filter")]
    pub filter: Vec<String>,
}

impl FacetFilterOneOf11String {
    /// String filter
    pub fn new(field: String, filter: Vec<String>) -> FacetFilterOneOf11String {
        FacetFilterOneOf11String { field, filter }
    }
}
