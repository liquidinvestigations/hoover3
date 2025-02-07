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

/// QueryFacetOneOf5I16 : Range segment definition for numerical facet field values of type i16
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryFacetOneOf5I16 {
    /// field name
    #[serde(rename = "field")]
    pub field: String,
    /// range type (CountWithinRange,CountBelowRange,CountAboveRange)
    #[serde(rename = "range_type")]
    pub range_type: models::RangeType,
    /// range label, range start
    #[serde(rename = "ranges")]
    pub ranges: Vec<Vec<serde_json::Value>>,
}

impl QueryFacetOneOf5I16 {
    /// Range segment definition for numerical facet field values of type i16
    pub fn new(field: String, range_type: models::RangeType, ranges: Vec<Vec<serde_json::Value>>) -> QueryFacetOneOf5I16 {
        QueryFacetOneOf5I16 {
            field,
            range_type,
            ranges,
        }
    }
}

