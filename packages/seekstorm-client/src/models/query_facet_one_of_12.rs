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

/// QueryFacetOneOf12 : Facet field values of type string set
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryFacetOneOf12 {
    #[serde(rename = "StringSet")]
    pub string_set: Box<models::QueryFacetOneOf12StringSet>,
}

impl QueryFacetOneOf12 {
    /// Facet field values of type string set
    pub fn new(string_set: models::QueryFacetOneOf12StringSet) -> QueryFacetOneOf12 {
        QueryFacetOneOf12 {
            string_set: Box::new(string_set),
        }
    }
}
