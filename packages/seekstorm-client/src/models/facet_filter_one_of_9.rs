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

/// FacetFilterOneOf9 : F32 range filter
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FacetFilterOneOf9 {
    #[serde(rename = "F32")]
    pub f32: Box<models::FacetFilterOneOf9F32>,
}

impl FacetFilterOneOf9 {
    /// F32 range filter
    pub fn new(f32: models::FacetFilterOneOf9F32) -> FacetFilterOneOf9 {
        FacetFilterOneOf9 { f32: Box::new(f32) }
    }
}
