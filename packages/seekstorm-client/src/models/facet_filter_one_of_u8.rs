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

/// FacetFilterOneOfU8 : U8 range filter
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FacetFilterOneOfU8 {
    /// field name
    #[serde(rename = "field")]
    pub field: String,
    /// filter: range start, range end
    #[serde(rename = "filter")]
    pub filter: Box<models::RangeU8>,
}

impl FacetFilterOneOfU8 {
    /// U8 range filter
    pub fn new(field: String, filter: models::RangeU8) -> FacetFilterOneOfU8 {
        FacetFilterOneOfU8 {
            field,
            filter: Box::new(filter),
        }
    }
}
