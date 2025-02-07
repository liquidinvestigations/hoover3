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

/// FacetValueOneOf6 : Signed 16-bit integer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FacetValueOneOf6 {
    /// Signed 16-bit integer
    #[serde(rename = "I16")]
    pub i16: i32,
}

impl FacetValueOneOf6 {
    /// Signed 16-bit integer
    pub fn new(i16: i32) -> FacetValueOneOf6 {
        FacetValueOneOf6 {
            i16,
        }
    }
}

