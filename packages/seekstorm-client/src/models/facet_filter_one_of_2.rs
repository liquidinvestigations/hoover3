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

/// FacetFilterOneOf2 : U32 range filter
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FacetFilterOneOf2 {
    #[serde(rename = "U32")]
    pub u32: Box<models::FacetFilterOneOf2U32>,
}

impl FacetFilterOneOf2 {
    /// U32 range filter
    pub fn new(u32: models::FacetFilterOneOf2U32) -> FacetFilterOneOf2 {
        FacetFilterOneOf2 { u32: Box::new(u32) }
    }
}
