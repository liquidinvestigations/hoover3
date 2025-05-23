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

/// QueryFacetOneOf : Range segment definition for numerical facet field values of type u8
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryFacetOneOf {
    #[serde(rename = "U8")]
    pub u8: Box<models::QueryFacetOneOfU8>,
}

impl QueryFacetOneOf {
    /// Range segment definition for numerical facet field values of type u8
    pub fn new(u8: models::QueryFacetOneOfU8) -> QueryFacetOneOf {
        QueryFacetOneOf { u8: Box::new(u8) }
    }
}
