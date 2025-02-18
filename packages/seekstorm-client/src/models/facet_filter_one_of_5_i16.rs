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

/// FacetFilterOneOf5I16 : I16 range filter
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FacetFilterOneOf5I16 {
    /// field name
    #[serde(rename = "field")]
    pub field: String,
    /// filter: range start, range end
    #[serde(rename = "filter")]
    pub filter: Box<models::RangeI16>,
}

impl FacetFilterOneOf5I16 {
    /// I16 range filter
    pub fn new(field: String, filter: models::RangeI16) -> FacetFilterOneOf5I16 {
        FacetFilterOneOf5I16 {
            field,
            filter: Box::new(filter),
        }
    }
}
