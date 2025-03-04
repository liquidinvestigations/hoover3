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

/// FacetValueOneOf1 : Unsigned 8-bit integer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FacetValueOneOf1 {
    /// Unsigned 8-bit integer
    #[serde(rename = "U8")]
    pub u8: i32,
}

impl FacetValueOneOf1 {
    /// Unsigned 8-bit integer
    pub fn new(u8: i32) -> FacetValueOneOf1 {
        FacetValueOneOf1 { u8 }
    }
}
