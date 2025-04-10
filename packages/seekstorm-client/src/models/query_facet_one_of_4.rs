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

/// QueryFacetOneOf4 : Range segment definition for numerical facet field values of type i8
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryFacetOneOf4 {
    #[serde(rename = "I8")]
    pub i8: Box<models::QueryFacetOneOf4I8>,
}

impl QueryFacetOneOf4 {
    /// Range segment definition for numerical facet field values of type i8
    pub fn new(i8: models::QueryFacetOneOf4I8) -> QueryFacetOneOf4 {
        QueryFacetOneOf4 { i8: Box::new(i8) }
    }
}
