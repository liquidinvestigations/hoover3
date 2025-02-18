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

/// FacetFilterOneOf4 : I8 range filter
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FacetFilterOneOf4 {
    #[serde(rename = "I8")]
    pub i8: Box<models::FacetFilterOneOf4I8>,
}

impl FacetFilterOneOf4 {
    /// I8 range filter
    pub fn new(i8: models::FacetFilterOneOf4I8) -> FacetFilterOneOf4 {
        FacetFilterOneOf4 { i8: Box::new(i8) }
    }
}
