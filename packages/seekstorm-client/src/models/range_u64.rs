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

/// RangeU64 : U64 range filter
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RangeU64 {
    /// range start
    #[serde(rename = "start")]
    pub start: i64,
    /// range end
    #[serde(rename = "end")]
    pub end: i64,
}

impl RangeU64 {
    /// U64 range filter
    pub fn new(start: i64, end: i64) -> RangeU64 {
        RangeU64 {
            start,
            end,
        }
    }
}

