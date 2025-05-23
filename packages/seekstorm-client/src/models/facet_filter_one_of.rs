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

/// FacetFilterOneOf : U8 range filter
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FacetFilterOneOf {
    #[serde(rename = "U8")]
    pub u8: Box<models::FacetFilterOneOfU8>,
}

impl FacetFilterOneOf {
    /// U8 range filter
    pub fn new(u8: models::FacetFilterOneOfU8) -> FacetFilterOneOf {
        FacetFilterOneOf { u8: Box::new(u8) }
    }
}
