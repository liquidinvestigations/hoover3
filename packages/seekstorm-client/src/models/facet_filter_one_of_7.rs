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

/// FacetFilterOneOf7 : I64 range filter
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FacetFilterOneOf7 {
    #[serde(rename = "I64")]
    pub i64: Box<models::FacetFilterOneOf7I64>,
}

impl FacetFilterOneOf7 {
    /// I64 range filter
    pub fn new(i64: models::FacetFilterOneOf7I64) -> FacetFilterOneOf7 {
        FacetFilterOneOf7 { i64: Box::new(i64) }
    }
}
