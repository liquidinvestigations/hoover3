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

/// QueryFacetOneOf8Timestamp : Range segment definition for numerical facet field values of type Unix timestamp
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryFacetOneOf8Timestamp {
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

impl QueryFacetOneOf8Timestamp {
    /// Range segment definition for numerical facet field values of type Unix timestamp
    pub fn new(
        field: String,
        range_type: models::RangeType,
        ranges: Vec<Vec<serde_json::Value>>,
    ) -> QueryFacetOneOf8Timestamp {
        QueryFacetOneOf8Timestamp {
            field,
            range_type,
            ranges,
        }
    }
}
