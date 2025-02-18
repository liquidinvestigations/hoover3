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

/// QueryFacetOneOf9 : Range segment definition for numerical facet field values of type f32
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryFacetOneOf9 {
    #[serde(rename = "F32")]
    pub f32: Box<models::QueryFacetOneOf9F32>,
}

impl QueryFacetOneOf9 {
    /// Range segment definition for numerical facet field values of type f32
    pub fn new(f32: models::QueryFacetOneOf9F32) -> QueryFacetOneOf9 {
        QueryFacetOneOf9 { f32: Box::new(f32) }
    }
}
