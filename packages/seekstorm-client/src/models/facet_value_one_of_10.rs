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

/// FacetValueOneOf10 : 32-bit floating point number
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FacetValueOneOf10 {
    /// 32-bit floating point number
    #[serde(rename = "F32")]
    pub f32: f32,
}

impl FacetValueOneOf10 {
    /// 32-bit floating point number
    pub fn new(f32: f32) -> FacetValueOneOf10 {
        FacetValueOneOf10 {
            f32,
        }
    }
}

