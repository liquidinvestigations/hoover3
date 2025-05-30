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

/// QueryFacetOneOf11String : Facet field values of type string
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryFacetOneOf11String {
    /// field name
    #[serde(rename = "field")]
    pub field: String,
    /// Prefix filter of facet values to return
    #[serde(rename = "prefix")]
    pub prefix: String,
    /// maximum number of facet values to return
    #[serde(rename = "length")]
    pub length: i32,
}

impl QueryFacetOneOf11String {
    /// Facet field values of type string
    pub fn new(field: String, prefix: String, length: i32) -> QueryFacetOneOf11String {
        QueryFacetOneOf11String {
            field,
            prefix,
            length,
        }
    }
}
